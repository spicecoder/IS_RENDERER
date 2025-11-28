// ============================================================================
// INTENTION SPACE PHYSICS ENGINE - 7 Days from Today
// Implements: I-O-I-DN-I-GL-I Flow
// ============================================================================

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

/// Trivalent Logic: Yes, No, Undecided
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TV {
    Y,  // Yes - condition satisfied
    N,  // No - condition not satisfied
    U,  // Undecided - awaiting computation
}

/// Field Pulse - Immutable response carrier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldPulse {
    pub id: String,
    pub name: String,
    pub tv: TV,
    pub response: Vec<String>,
    pub timestamp: DateTime<Local>,
}

impl FieldPulse {
    pub fn new(name: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            tv: TV::U,
            response: Vec::new(),
            timestamp: Local::now(),
        }
    }

    pub fn with_response(mut self, response: Vec<String>) -> Self {
        self.response = response;
        self.tv = TV::Y;
        self
    }
}

/// Design Node (DN) - Computation unit
#[derive(Debug, Clone)]
pub struct DesignNode {
    pub id: String,
    pub name: String,
    pub compute_fn: fn(&[FieldPulse]) -> Vec<String>,
}

impl DesignNode {
    pub fn new(id: &str, name: &str, compute_fn: fn(&[FieldPulse]) -> Vec<String>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            compute_fn,
        }
    }

    /// Execute computation on input pulses
    pub fn compute(&self, inputs: &[FieldPulse]) -> Vec<String> {
        (self.compute_fn)(inputs)
    }
}

/// Object - Static mapping function
#[derive(Debug, Clone)]
pub struct IntentionObject {
    pub id: String,
    pub name: String,
    pub mapping_fn: fn(&FieldPulse) -> FieldPulse,
}

impl IntentionObject {
    pub fn new(id: &str, name: &str, mapping_fn: fn(&FieldPulse) -> FieldPulse) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            mapping_fn,
        }
    }

    /// Apply static mapping to pulse
    pub fn map(&self, input: &FieldPulse) -> FieldPulse {
        (self.mapping_fn)(input)
    }
}

/// Grid Lookout (GL) - Display/interaction layer
#[derive(Debug, Clone)]
pub struct GridLookout {
    pub id: String,
    pub name: String,
    pub layer: u8,
    pub row: u8,
    pub col: u8,
    pub pulse_bindings: Vec<String>, // Pulse IDs this GL displays
}

impl GridLookout {
    pub fn new(id: &str, name: &str, layer: u8, row: u8, col: u8) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            layer,
            row,
            col,
            pulse_bindings: Vec::new(),
        }
    }

    pub fn bind_pulse(&mut self, pulse_id: String) {
        self.pulse_bindings.push(pulse_id);
    }
}

/// CPUX - Computational Path of Understanding and Execution
#[derive(Debug)]
pub struct CPUX {
    pub id: String,
    pub name: String,
    pub design_nodes: Vec<DesignNode>,
    pub objects: Vec<IntentionObject>,
    pub grid_lookouts: Vec<GridLookout>,
    pub field: Arc<Mutex<NetworkField>>,
    pub progressor_delta_ns: u64,
}

impl CPUX {
    pub fn new(id: &str, name: &str, field: Arc<Mutex<NetworkField>>) -> Self {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            design_nodes: Vec::new(),
            objects: Vec::new(),
            grid_lookouts: Vec::new(),
            field,
            progressor_delta_ns: 16_666_667, // ~60 FPS
        }
    }

    pub fn add_design_node(&mut self, dn: DesignNode) {
        self.design_nodes.push(dn);
    }

    pub fn add_object(&mut self, obj: IntentionObject) {
        self.objects.push(obj);
    }

    pub fn add_grid_lookout(&mut self, gl: GridLookout) {
        self.grid_lookouts.push(gl);
    }

    /// Execute one cycle: I-O-I-DN-I-GL-I
    pub fn execute_cycle(&self, intention_pulse: FieldPulse) -> Vec<FieldPulse> {
        let mut results = Vec::new();

        // I -> O: Object mapping
        for obj in &self.objects {
            let mapped_pulse = obj.map(&intention_pulse);
            
            // O -> I -> DN: Design Node computation
            for dn in &self.design_nodes {
                let compute_result = dn.compute(&[mapped_pulse.clone()]);
                
                // DN -> I: Create output pulse
                let output_pulse = FieldPulse::new(&format!("{}_output", dn.id))
                    .with_response(compute_result);
                
                // I -> GL: Grid lookout display
                for gl in &self.grid_lookouts {
                    if gl.pulse_bindings.contains(&output_pulse.id) {
                        // GL receives pulse for display
                        results.push(output_pulse.clone());
                    }
                }
                
                // GL -> I: User interaction creates new intention
                // (handled by UI callbacks)
            }
        }

        // Store results in network field
        let mut field = self.field.lock().unwrap();
        for pulse in &results {
            field.store_pulse(pulse.clone());
        }

        results
    }
}

/// Network Field - Dynamic pulse storage
#[derive(Debug, Default)]
pub struct NetworkField {
    pub pulses: HashMap<String, FieldPulse>,
}

impl NetworkField {
    pub fn new() -> Self {
        Self {
            pulses: HashMap::new(),
        }
    }

    pub fn store_pulse(&mut self, pulse: FieldPulse) {
        self.pulses.insert(pulse.id.clone(), pulse);
    }

    pub fn get_pulse(&self, id: &str) -> Option<&FieldPulse> {
        self.pulses.get(id)
    }

    pub fn get_pulses_by_name(&self, name: &str) -> Vec<&FieldPulse> {
        self.pulses
            .values()
            .filter(|p| p.name == name)
            .collect()
    }

    pub fn clear(&mut self) {
        self.pulses.clear();
    }
}

/// Space Loop - Orchestrates all CPUX units
pub struct SpaceLoop {
    pub cpux_units: Vec<CPUX>,
    pub network_field: Arc<Mutex<NetworkField>>,
}

impl SpaceLoop {
    pub fn new() -> Self {
        Self {
            cpux_units: Vec::new(),
            network_field: Arc::new(Mutex::new(NetworkField::new())),
        }
    }

    pub fn add_cpux(&mut self, cpux: CPUX) {
        self.cpux_units.push(cpux);
    }

    pub fn get_field(&self) -> Arc<Mutex<NetworkField>> {
        Arc::clone(&self.network_field)
    }

    /// Execute one frame of all CPUX units
    pub fn tick(&self) {
        for cpux in &self.cpux_units {
            // Create intention to start cycle
            let intention = FieldPulse::new("tick");
            cpux.execute_cycle(intention);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_pulse_creation() {
        let pulse = FieldPulse::new("test_pulse");
        assert_eq!(pulse.name, "test_pulse");
        assert_eq!(pulse.tv, TV::U);
        assert!(pulse.response.is_empty());
    }

    #[test]
    fn test_network_field() {
        let mut field = NetworkField::new();
        let pulse = FieldPulse::new("test").with_response(vec!["data".to_string()]);
        let id = pulse.id.clone();
        
        field.store_pulse(pulse);
        
        let retrieved = field.get_pulse(&id);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test");
    }
}
