// ============================================================================
// INTENTION SPACE PHYSICS ENGINE
// Core physics implementation for TapStream
// ============================================================================

use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use tokio::time::Duration;

// ============================================================================
// FUNDAMENTAL TYPES - RESPECTING IMMUTABILITY PHYSICS
// ============================================================================

/// Trivalent Truth Value
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TV {
    Y,  // Yes/True
    N,  // No/False
    U,  // Undecided/Unknown
}

/// Pulse - predetermined at design time, but TV can change at runtime
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pulse {
    pub id: String,
    pub name: String,
    pub tv: TV,
    pub response: String,
}

/// PulseRef - references which pulse a response came from
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PulseRef {
    pub pulse_id: String,
}

/// Response - IMMUTABLE once created
/// Only the pulse reference changes, not the value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub pulse_ref: PulseRef,
    pub value: String,
    pub tv: TV,
}

/// Intention - carries immutable responses through the system
/// I → O → I → DN → I flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intention {
    pub id: String,
    #[serde(serialize_with = "serialize_arc_vec", deserialize_with = "deserialize_arc_vec")]
    pub responses: Arc<Vec<Response>>,
    pub timestamp_ns: u64,
}

// Custom serialization for Arc<Vec<Response>>
fn serialize_arc_vec<S>(arc: &Arc<Vec<Response>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    arc.as_ref().serialize(serializer)
}

fn deserialize_arc_vec<'de, D>(deserializer: D) -> Result<Arc<Vec<Response>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let vec = Vec::<Response>::deserialize(deserializer)?;
    Ok(Arc::new(vec))
}

impl Intention {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            responses: Arc::new(vec![]),
            timestamp_ns: Self::now_ns(),
        }
    }

    pub fn with_responses(id: impl Into<String>, responses: Vec<Response>) -> Self {
        Self {
            id: id.into(),
            responses: Arc::new(responses),
            timestamp_ns: Self::now_ns(),
        }
    }

    /// PHYSICS: Transfer responses from one pulse to another
    /// Values remain IMMUTABLE, only pulse references change
    pub fn transfer_to_pulse(&self, from_pulse_id: &str, to_pulse_id: &str) -> Self {
        let new_responses: Vec<Response> = self.responses
            .iter()
            .map(|r| {
                if r.pulse_ref.pulse_id == from_pulse_id {
                    Response {
                        pulse_ref: PulseRef { pulse_id: to_pulse_id.to_string() },
                        value: r.value.clone(),  // VALUE UNCHANGED
                        tv: r.tv,                // TV UNCHANGED
                    }
                } else {
                    r.clone()
                }
            })
            .collect();

        Intention {
            id: format!("{}→{}", self.id, to_pulse_id),
            responses: Arc::new(new_responses),
            timestamp_ns: Self::now_ns(),
        }
    }

    pub fn from_pulse(pulse: &Pulse) -> Self {
        let response = Response {
            pulse_ref: PulseRef { pulse_id: pulse.id.clone() },
            value: pulse.response.clone(),
            tv: pulse.tv,
        };

        Self::with_responses(
            format!("intention_from_{}", pulse.id),
            vec![response],
        )
    }

    fn now_ns() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64
    }
}

// ============================================================================
// DESIGN NODE (DN) - WHERE COMPUTATION HAPPENS
// ============================================================================

pub type ComputeFn = Arc<dyn Fn(&Intention) -> Intention + Send + Sync>;

pub struct DesignNode {
    pub id: String,
    pub compute: ComputeFn,
    pub internal_pulses: RwLock<Vec<Pulse>>,
}

impl DesignNode {
    pub fn new(id: impl Into<String>, compute: ComputeFn) -> Self {
        Self {
            id: id.into(),
            compute,
            internal_pulses: RwLock::new(vec![]),
        }
    }

    /// Execute computation - happens ONLY in DN
    pub fn execute(&self, intention: &Intention) -> Intention {
        (self.compute)(intention)
    }
}

// ============================================================================
// OBJECT (O) - STATIC MAPPING, SYNCHRONOUS WITH PROGRESSOR
// ============================================================================

pub type MappingFn = Arc<dyn Fn(&Intention) -> Intention + Send + Sync>;

pub struct Object {
    pub id: String,
    pub pulses: Vec<Pulse>,  // STATIC collection
    pub mapping: MappingFn,
}

impl Object {
    pub fn new(id: impl Into<String>, pulses: Vec<Pulse>, mapping: MappingFn) -> Self {
        Self {
            id: id.into(),
            pulses,
            mapping,
        }
    }

    /// Apply static mapping
    pub fn apply_mapping(&self, intention: &Intention) -> Intention {
        (self.mapping)(intention)
    }
}

// ============================================================================
// FIELD - DYNAMIC PULSE COLLECTION
// ============================================================================

pub struct Field {
    pub id: String,
    pub pulses: RwLock<HashMap<String, Pulse>>,
    pub members: RwLock<Vec<String>>,
}

impl Field {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            pulses: RwLock::new(HashMap::new()),
            members: RwLock::new(vec![]),
        }
    }

    /// PHYSICS: Field dynamically updated as members emit/absorb
    pub fn absorb_intention(&self, intention: &Intention) {
        let mut pulses = self.pulses.write().unwrap();
        
        for response in intention.responses.iter() {
            if let Some(pulse) = pulses.get_mut(&response.pulse_ref.pulse_id) {
                // Update TV based on response
                pulse.tv = response.tv;
                pulse.response = response.value.clone();
            } else {
                // Add new pulse to field
                pulses.insert(
                    response.pulse_ref.pulse_id.clone(),
                    Pulse {
                        id: response.pulse_ref.pulse_id.clone(),
                        name: response.pulse_ref.pulse_id.clone(),
                        tv: response.tv,
                        response: response.value.clone(),
                    },
                );
            }
        }
    }

    pub fn emit_from_pulse(&self, pulse_id: &str) -> Option<Intention> {
        let pulses = self.pulses.read().unwrap();
        pulses.get(pulse_id).map(Intention::from_pulse)
    }

    pub fn get_pulse(&self, pulse_id: &str) -> Option<Pulse> {
        let pulses = self.pulses.read().unwrap();
        pulses.get(pulse_id).cloned()
    }

    pub fn get_all_pulses(&self) -> Vec<Pulse> {
        let pulses = self.pulses.read().unwrap();
        pulses.values().cloned().collect()
    }
}

// ============================================================================
// GRID LOOKOUT (GL) - ONE CELL = ONE PULSE
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellPosition {
    pub layer: u32,
    pub row: u32,
    pub col: u32,
}

#[derive(Clone)]
pub struct GridCell {
    pub position: CellPosition,
    pub pulse: Pulse,
    pub interaction_dn: Option<Arc<DesignNode>>,
}

pub struct GridLayer {
    pub z_index: u32,
    pub cells: Vec<Vec<GridCell>>,
}

pub struct GridLookout {
    pub id: String,
    pub layers: Vec<GridLayer>,
    pub exposed_field: Field,
}

impl GridLookout {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            layers: vec![],
            exposed_field: Field::new("gl_field"),
        }
    }

    /// PHYSICS: Grid exposes a CPUX field from visible cells
    pub fn update_exposed_field(&self) {
        for layer in &self.layers {
            for row in &layer.cells {
                for cell in row {
                    self.exposed_field.absorb_intention(
                        &Intention::from_pulse(&cell.pulse)
                    );
                }
            }
        }
    }

    pub fn get_cell(&self, pos: &CellPosition) -> Option<&GridCell> {
        self.layers
            .get(pos.layer as usize)?
            .cells
            .get(pos.row as usize)?
            .get(pos.col as usize)
    }

    pub fn get_cell_mut(&mut self, pos: &CellPosition) -> Option<&mut GridCell> {
        self.layers
            .get_mut(pos.layer as usize)?
            .cells
            .get_mut(pos.row as usize)?
            .get_mut(pos.col as usize)
    }
}

// ============================================================================
// CPUX MEMBER - DN | O | GL
// ============================================================================

pub enum CPUXMember {
    DN(Arc<DesignNode>),
    Object(Arc<Object>),
    GL(Arc<RwLock<GridLookout>>),
}

// ============================================================================
// CPUX THREAD - I-O-I-DN-I-GL-I FLOW PHYSICS
// ============================================================================

pub struct CPUXThread {
    pub id: String,
    pub members: Vec<CPUXMember>,
    pub field: Arc<Field>,
    pub progressor_delta_ns: u64,  // Minimal time between member starts
}

impl CPUXThread {
    pub fn new(
        id: impl Into<String>,
        progressor_delta_ns: u64,
    ) -> Self {
        Self {
            id: id.into(),
            members: vec![],
            field: Arc::new(Field::new("cpux_field")),
            progressor_delta_ns,
        }
    }

    pub fn add_member(&mut self, member: CPUXMember) {
        self.members.push(member);
    }

    /// PHYSICS: Execute I-O-I-DN-I-O-I-DN-I-GL-I flow
    pub async fn execute_flow(&self, mut intention: Intention) -> Intention {
        for member in &self.members {
            // Progressor advancement - minimal delta time
            tokio::time::sleep(Duration::from_nanos(self.progressor_delta_ns)).await;

            intention = match member {
                CPUXMember::DN(dn) => {
                    // Computation happens HERE
                    dn.execute(&intention)
                }

                CPUXMember::Object(obj) => {
                    // Static mapping - synchronous with progressor
                    obj.apply_mapping(&intention)
                }

                CPUXMember::GL(gl) => {
                    // Grid lookout interaction
                    let gl_guard = gl.read().unwrap();
                    gl_guard.exposed_field.absorb_intention(&intention);
                    intention.clone()
                }
            };

            // Emit/Absorb with field
            self.field.absorb_intention(&intention);
        }

        intention
    }

    /// Run continuous loop
    pub async fn run_loop(&self, initial_intention: Intention) {
        let mut current_intention = initial_intention;

        loop {
            current_intention = self.execute_flow(current_intention).await;

            // Check if all preconditions invalid
            let all_invalid = current_intention.responses
                .iter()
                .all(|r| r.tv == TV::N);

            if all_invalid {
                break;
            }
        }
    }
}

// ============================================================================
// SEQUENTIAL INTERACTION RESOLVER
// Multi-sensory inputs → Sequential resolution
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InteractionType {
    Tap,
    LongPress,
    Swipe,
    View,
    Audio,
}

pub struct InteractionEvent {
    pub interaction_type: InteractionType,
    pub cell_position: CellPosition,
    pub timestamp_ns: u64,
}

pub struct SequentialInteractionResolver {
    pub progressor_delta_ns: u64,
    queue: RwLock<Vec<InteractionEvent>>,
}

impl SequentialInteractionResolver {
    pub fn new(progressor_delta_ns: u64) -> Self {
        Self {
            progressor_delta_ns,
            queue: RwLock::new(vec![]),
        }
    }

    pub fn queue_interaction(&self, event: InteractionEvent) {
        let mut queue = self.queue.write().unwrap();
        queue.push(event);
        queue.sort_by_key(|e| e.timestamp_ns);
    }

    /// PHYSICS: Sequential resolution with progressor delta time
    pub async fn resolve_interactions(&self, gl: &Arc<RwLock<GridLookout>>) -> Vec<Intention> {
        let mut intentions = vec![];
        
        loop {
            let event = {
                let mut queue = self.queue.write().unwrap();
                if queue.is_empty() {
                    break;
                }
                queue.remove(0)
            };

            // Minimal delta time between interaction processing
            tokio::time::sleep(Duration::from_nanos(self.progressor_delta_ns)).await;

            let gl_read = gl.read().unwrap();
            if let Some(cell) = gl_read.get_cell(&event.cell_position) {
                if let Some(ref dn) = cell.interaction_dn {
                    let intention = dn.execute(&Intention::from_pulse(&cell.pulse));
                    intentions.push(intention);
                }
            }
        }

        intentions
    }
}
