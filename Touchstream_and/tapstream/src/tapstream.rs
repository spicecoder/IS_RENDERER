// ============================================================================
// TAPSTREAM APPLICATION
// Cook → Network Field → Diner
// ============================================================================

use crate::intention_space::*;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};

// ============================================================================
// TAPSTREAM ROLES
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StreamRole {
    Cook,   // Producer
    Diner,  // Consumer
}

// ============================================================================
// STREAM FRAME - Data carried through network field
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamFrame {
    pub frame_id: u64,
    pub timestamp_ns: u64,
    pub image_data: Vec<u8>,  // JPEG or similar
    pub cook_id: String,
    pub metadata: FrameMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameMetadata {
    pub dish_name: String,
    pub cooking_stage: String,
    pub temperature: Option<f32>,
    pub timer_seconds: Option<u32>,
}

// ============================================================================
// NETWORK FIELD - WiFi pulses carrying stream data
// ============================================================================

pub struct NetworkField {
    pub field: Arc<Field>,
    pub stream_channel: broadcast::Sender<StreamFrame>,
}

impl NetworkField {
    pub fn new() -> Self {
        let (tx, _rx) = broadcast::channel(100);
        
        Self {
            field: Arc::new(Field::new("wifi_network_field")),
            stream_channel: tx,
        }
    }

    /// Publish frame to network field
    pub fn publish_frame(&self, frame: StreamFrame) {
        // Create pulse for this frame
        let pulse = Pulse {
            id: format!("frame_{}", frame.frame_id),
            name: "stream_frame".to_string(),
            tv: TV::Y,
            response: serde_json::to_string(&frame).unwrap(),
        };

        // Absorb into field
        self.field.absorb_intention(&Intention::from_pulse(&pulse));

        // Broadcast to subscribers
        let _ = self.stream_channel.send(frame);
    }

    /// Subscribe to stream
    pub fn subscribe(&self) -> broadcast::Receiver<StreamFrame> {
        self.stream_channel.subscribe()
    }
}

// ============================================================================
// COOK (PRODUCER) - Camera DN → Encoder O → Network Field
// ============================================================================

pub struct CookProducer {
    pub cook_id: String,
    pub cpux: Arc<CPUXThread>,
    pub preview_gl: Arc<RwLock<GridLookout>>,
    pub network_field: Arc<NetworkField>,
    pub frame_counter: RwLock<u64>,
}

impl CookProducer {
    pub fn new(
        cook_id: impl Into<String>,
        network_field: Arc<NetworkField>,
    ) -> Self {
        let cook_id = cook_id.into();
        
        // Create preview Grid Lookout (3x3 grid for cook's view)
        let preview_gl = Arc::new(RwLock::new(GridLookout::new("cook_preview")));
        
        // Build CPUX: Camera DN → Encoder O → Network Field
        let mut cpux = CPUXThread::new(
            format!("cook_{}_cpux", cook_id),
            1_000_000, // 1ms progressor delta
        );

        // Camera DN - captures frames
        let camera_dn = Arc::new(DesignNode::new(
            "camera_dn",
            Arc::new(|intention: &Intention| {
                // In real app: capture camera frame
                // For now: simulate
                let _frame_data = vec![0u8; 1024]; // Placeholder
                
                let response = Response {
                    pulse_ref: PulseRef { pulse_id: "camera_frame".to_string() },
                    value: format!("frame_captured_{}", intention.timestamp_ns),
                    tv: TV::Y,
                };

                Intention::with_responses(
                    format!("camera_output_{}", intention.timestamp_ns),
                    vec![response],
                )
            }),
        ));

        // Encoder Object - encodes frames
        let encoder_obj = Arc::new(Object::new(
            "encoder",
            vec![
                Pulse {
                    id: "encoding_ready".to_string(),
                    name: "encoder_ready".to_string(),
                    tv: TV::Y,
                    response: "ready".to_string(),
                }
            ],
            Arc::new(|intention: &Intention| {
                // Static mapping: camera frame → encoded frame
                intention.transfer_to_pulse("camera_frame", "encoded_frame")
            }),
        ));

        cpux.add_member(CPUXMember::DN(camera_dn));
        cpux.add_member(CPUXMember::Object(encoder_obj));
        cpux.add_member(CPUXMember::GL(preview_gl.clone()));

        Self {
            cook_id,
            cpux: Arc::new(cpux),
            preview_gl,
            network_field,
            frame_counter: RwLock::new(0),
        }
    }

    /// Start streaming
    pub async fn start_streaming(&self, metadata: FrameMetadata) {
        let mut interval = tokio::time::interval(
            std::time::Duration::from_millis(33) // ~30 fps
        );

        loop {
            interval.tick().await;

            // Execute CPUX flow
            let initial_intention = Intention::new("capture_frame");
            let result = self.cpux.execute_flow(initial_intention).await;

            // Create stream frame
            let frame_id = {
                let mut counter = self.frame_counter.write().unwrap();
                *counter += 1;
                *counter
            };

            let frame = StreamFrame {
                frame_id,
                timestamp_ns: result.timestamp_ns,
                image_data: vec![0u8; 1024], // Placeholder
                cook_id: self.cook_id.clone(),
                metadata: metadata.clone(),
            };

            // Publish to network field
            self.network_field.publish_frame(frame);

            // Update preview GL
            self.update_preview_grid(frame_id);
        }
    }

    fn update_preview_grid(&self, frame_id: u64) {
        let mut gl = self.preview_gl.write().unwrap();
        
        // Update GL pulses
        if gl.layers.is_empty() {
            // Initialize 3x3 preview grid
            self.initialize_preview_grid(&mut gl);
        }

        // Update frame counter pulse
        if let Some(cell) = gl.get_cell_mut(&CellPosition { layer: 0, row: 0, col: 0 }) {
            cell.pulse.response = format!("Frame: {}", frame_id);
            cell.pulse.tv = TV::Y;
        }

        gl.update_exposed_field();
    }

    fn initialize_preview_grid(&self, gl: &mut GridLookout) {
        // Initialize with proper 2D vec structure
        let mut cells: Vec<Vec<GridCell>> = Vec::new();
        
        for row in 0..3 {
            let mut row_cells = Vec::new();
            for col in 0..3 {
                let pulse = Pulse {
                    id: format!("preview_{}_{}", row, col),
                    name: format!("Preview Cell [{},{}]", row, col),
                    tv: TV::U,
                    response: "".to_string(),
                };

                row_cells.push(GridCell {
                    position: CellPosition { layer: 0, row: row as u32, col: col as u32 },
                    pulse,
                    interaction_dn: None,
                });
            }
            cells.push(row_cells);
        }

        gl.layers.push(GridLayer {
            z_index: 0,
            cells,
        });
    }
}

// ============================================================================
// DINER (CONSUMER) - Network Field → Decoder O → Display DN → GL
// ============================================================================

pub struct DinerConsumer {
    pub diner_id: String,
    pub cpux: Arc<CPUXThread>,
    pub stream_gl: Arc<RwLock<GridLookout>>,
    pub network_field: Arc<NetworkField>,
    pub interaction_resolver: Arc<SequentialInteractionResolver>,
}

impl DinerConsumer {
    pub fn new(
        diner_id: impl Into<String>,
        network_field: Arc<NetworkField>,
    ) -> Self {
        let diner_id = diner_id.into();
        
        // Create stream Grid Lookout (4x4 grid for diner's view)
        let stream_gl = Arc::new(RwLock::new(GridLookout::new("diner_stream")));
        
        // Build CPUX: Decoder O → Display DN → GL
        let mut cpux = CPUXThread::new(
            format!("diner_{}_cpux", diner_id),
            1_000_000, // 1ms progressor delta
        );

        // Decoder Object - decodes frames
        let decoder_obj = Arc::new(Object::new(
            "decoder",
            vec![
                Pulse {
                    id: "decoding_ready".to_string(),
                    name: "decoder_ready".to_string(),
                    tv: TV::Y,
                    response: "ready".to_string(),
                }
            ],
            Arc::new(|intention: &Intention| {
                // Static mapping: encoded frame → decoded frame
                intention.transfer_to_pulse("encoded_frame", "decoded_frame")
            }),
        ));

        // Display DN - renders frames
        let display_dn = Arc::new(DesignNode::new(
            "display_dn",
            Arc::new(|intention: &Intention| {
                // In real app: render to screen
                // For now: simulate
                
                let response = Response {
                    pulse_ref: PulseRef { pulse_id: "display_ready".to_string() },
                    value: "rendered".to_string(),
                    tv: TV::Y,
                };

                Intention::with_responses(
                    format!("display_output_{}", intention.timestamp_ns),
                    vec![response],
                )
            }),
        ));

        cpux.add_member(CPUXMember::Object(decoder_obj));
        cpux.add_member(CPUXMember::DN(display_dn));
        cpux.add_member(CPUXMember::GL(stream_gl.clone()));

        // Create interaction resolver
        let interaction_resolver = Arc::new(SequentialInteractionResolver::new(100_000)); // 0.1ms

        Self {
            diner_id,
            cpux: Arc::new(cpux),
            stream_gl,
            network_field,
            interaction_resolver,
        }
    }

    /// Start receiving stream
    pub async fn start_receiving(&self) {
        let mut receiver = self.network_field.subscribe();

        loop {
            match receiver.recv().await {
                Ok(frame) => {
                    // Execute CPUX flow with received frame
                    let intention = Intention::with_responses(
                        format!("frame_{}", frame.frame_id),
                        vec![Response {
                            pulse_ref: PulseRef { pulse_id: "encoded_frame".to_string() },
                            value: serde_json::to_string(&frame).unwrap(),
                            tv: TV::Y,
                        }],
                    );

                    let _result = self.cpux.execute_flow(intention).await;

                    // Update stream GL
                    self.update_stream_grid(&frame);
                }
                Err(_) => {
                    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                }
            }
        }
    }

    fn update_stream_grid(&self, frame: &StreamFrame) {
        let mut gl = self.stream_gl.write().unwrap();
        
        if gl.layers.is_empty() {
            self.initialize_stream_grid(&mut gl);
        }

        // Update cells with frame metadata
        // [0,0] - Main video display
        if let Some(cell) = gl.get_cell_mut(&CellPosition { layer: 0, row: 0, col: 0 }) {
            cell.pulse.response = format!("Frame {}", frame.frame_id);
            cell.pulse.tv = TV::Y;
        }

        // [1,0] - Dish name
        if let Some(cell) = gl.get_cell_mut(&CellPosition { layer: 0, row: 1, col: 0 }) {
            cell.pulse.response = frame.metadata.dish_name.clone();
            cell.pulse.tv = TV::Y;
        }

        // [1,1] - Cooking stage
        if let Some(cell) = gl.get_cell_mut(&CellPosition { layer: 0, row: 1, col: 1 }) {
            cell.pulse.response = frame.metadata.cooking_stage.clone();
            cell.pulse.tv = TV::Y;
        }

        // [1,2] - Temperature
        if let Some(cell) = gl.get_cell_mut(&CellPosition { layer: 0, row: 1, col: 2 }) {
            if let Some(temp) = frame.metadata.temperature {
                cell.pulse.response = format!("{}°C", temp);
                cell.pulse.tv = TV::Y;
            }
        }

        gl.update_exposed_field();
    }

    fn initialize_stream_grid(&self, gl: &mut GridLookout) {
        // Initialize with proper 2D vec structure
        let mut cells: Vec<Vec<GridCell>> = Vec::new();
        
        // Define grid cells (ONE CELL = ONE PULSE)
        let cell_definitions = vec![
            // Row 0: Main video display (spans full width)
            (0, 0, "video_display", "Stream Display"),
            (0, 1, "video_control_1", ""),
            (0, 2, "video_control_2", ""),
            (0, 3, "video_control_3", ""),
            // Row 1: Metadata
            (1, 0, "dish_name", "Dish"),
            (1, 1, "cooking_stage", "Stage"),
            (1, 2, "temperature", "Temp"),
            (1, 3, "timer", "Timer"),
            // Row 2: Interactive controls
            (2, 0, "btn_like", "❤️ Like"),
            (2, 1, "btn_comment", "💬 Comment"),
            (2, 2, "btn_share", "🔗 Share"),
            (2, 3, "btn_bookmark", "🔖 Save"),
            // Row 3: Status info
            (3, 0, "viewer_count", "Viewers"),
            (3, 1, "connection_status", "Status"),
            (3, 2, "quality", "Quality"),
            (3, 3, "latency", "Latency"),
        ];

        // Initialize 4 rows
        for _ in 0..4 {
            cells.push(Vec::new());
        }

        for (row, col, id, name) in cell_definitions {
            let pulse = Pulse {
                id: id.to_string(),
                name: name.to_string(),
                tv: TV::U,
                response: "".to_string(),
            };

            // Add interaction DN for buttons
            let interaction_dn = if id.starts_with("btn_") {
                Some(Arc::new(DesignNode::new(
                    format!("{}_dn", id),
                    Arc::new(move |intention: &Intention| {
                        println!("🎯 Button interaction: {}", id);
                        intention.clone()
                    }),
                )))
            } else {
                None
            };

            cells[row].push(GridCell {
                position: CellPosition { layer: 0, row: row as u32, col: col as u32 },
                pulse,
                interaction_dn,
            });
        }

        gl.layers.push(GridLayer {
            z_index: 0,
            cells,
        });
    }

    /// Handle user interaction (tap, swipe, etc.)
    pub fn handle_interaction(&self, interaction_type: InteractionType, cell_pos: CellPosition) {
        self.interaction_resolver.queue_interaction(InteractionEvent {
            interaction_type,
            cell_position: cell_pos,
            timestamp_ns: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
        });
    }

    /// Start interaction resolver loop
    pub async fn start_interaction_loop(&self) {
        loop {
            let intentions = self.interaction_resolver.resolve_interactions(&self.stream_gl).await;
            
            for intention in intentions {
                self.cpux.field.absorb_intention(&intention);
            }

            tokio::time::sleep(std::time::Duration::from_millis(16)).await; // ~60Hz
        }
    }
}

// ============================================================================
// TAPSTREAM SESSION
// ============================================================================

pub struct TapStreamSession {
    pub session_id: String,
    pub network_field: Arc<NetworkField>,
    pub cooks: RwLock<Vec<Arc<CookProducer>>>,
    pub diners: RwLock<Vec<Arc<DinerConsumer>>>,
}

impl TapStreamSession {
    pub fn new(session_id: impl Into<String>) -> Self {
        Self {
            session_id: session_id.into(),
            network_field: Arc::new(NetworkField::new()),
            cooks: RwLock::new(vec![]),
            diners: RwLock::new(vec![]),
        }
    }

    pub fn add_cook(&self, cook_id: impl Into<String>) -> Arc<CookProducer> {
        let cook = Arc::new(CookProducer::new(
            cook_id,
            self.network_field.clone(),
        ));

        self.cooks.write().unwrap().push(cook.clone());
        cook
    }

    pub fn add_diner(&self, diner_id: impl Into<String>) -> Arc<DinerConsumer> {
        let diner = Arc::new(DinerConsumer::new(
            diner_id,
            self.network_field.clone(),
        ));

        self.diners.write().unwrap().push(diner.clone());
        diner
    }

    pub fn get_network_stats(&self) -> NetworkStats {
        let pulses = self.network_field.field.get_all_pulses();
        let cooks_count = self.cooks.read().unwrap().len();
        let diners_count = self.diners.read().unwrap().len();

        NetworkStats {
            active_pulses: pulses.len(),
            active_cooks: cooks_count,
            active_diners: diners_count,
            session_id: self.session_id.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct NetworkStats {
    pub active_pulses: usize,
    pub active_cooks: usize,
    pub active_diners: usize,
    pub session_id: String,
}
