// ============================================================================
// TAPSTREAM EMBEDDED HTTP SERVER
// Runs on Android device's WiFi hotspot
// ============================================================================

use crate::tapstream::*;
use crate::intention_space::*;
use std::sync::Arc;
use warp::Filter;
use serde::{Deserialize, Serialize};

// ============================================================================
// SERVER STATE
// ============================================================================

pub struct TapStreamServer {
    pub session: Arc<TapStreamSession>,
    pub host: String,
    pub port: u16,
}

impl TapStreamServer {
    pub fn new(session_id: impl Into<String>, port: u16) -> Self {
        Self {
            session: Arc::new(TapStreamSession::new(session_id)),
            host: "0.0.0.0".to_string(), // Bind to all interfaces (WiFi)
            port,
        }
    }

    /// Start HTTP server
    pub async fn start(&self) {
        let session = self.session.clone();

        // ============================================================================
        // ROUTES
        // ============================================================================

        // GET / - Server info
        let session_info = session.clone();
        let info = warp::path::end()
            .map(move || {
                warp::reply::json(&serde_json::json!({
                    "service": "TapStream",
                    "version": "1.0.0",
                    "physics": "Intention Space",
                    "session_id": session_info.session_id,
                }))
            });

        // POST /cook/register - Register as cook (producer)
        let session_cook = session.clone();
        let register_cook = warp::path!("cook" / "register")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |req: RegisterRequest| {
                let cook = session_cook.add_cook(req.device_id);
                
                warp::reply::json(&serde_json::json!({
                    "status": "registered",
                    "role": "cook",
                    "cook_id": cook.cook_id,
                }))
            });

        // POST /cook/stream - Start streaming
        let session_stream = session.clone();
        let start_stream = warp::path!("cook" / "stream")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |req: StreamRequest| {
                // Start streaming in background
                let cooks = session_stream.cooks.read().unwrap();
                if let Some(cook) = cooks.iter().find(|c| c.cook_id == req.cook_id) {
                    let cook_clone = cook.clone();
                    let metadata = req.metadata;
                    
                    tokio::spawn(async move {
                        cook_clone.start_streaming(metadata).await;
                    });

                    warp::reply::json(&serde_json::json!({
                        "status": "streaming",
                        "cook_id": req.cook_id,
                    }))
                } else {
                    warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": "Cook not found",
                    }))
                }
            });

        // POST /diner/register - Register as diner (consumer)
        let session_diner = session.clone();
        let register_diner = warp::path!("diner" / "register")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |req: RegisterRequest| {
                let diner = session_diner.add_diner(req.device_id);
                
                warp::reply::json(&serde_json::json!({
                    "status": "registered",
                    "role": "diner",
                    "diner_id": diner.diner_id,
                }))
            });

        // POST /diner/watch - Start watching stream
        let session_watch = session.clone();
        let watch_stream = warp::path!("diner" / "watch")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |req: WatchRequest| {
                let diners = session_watch.diners.read().unwrap();
                if let Some(diner) = diners.iter().find(|d| d.diner_id == req.diner_id) {
                    let diner_clone = diner.clone();
                    
                    // Start receiving in background
                    tokio::spawn(async move {
                        diner_clone.start_receiving().await;
                    });

                    // Start interaction loop
                    let diner_interaction = diner.clone();
                    tokio::spawn(async move {
                        diner_interaction.start_interaction_loop().await;
                    });

                    warp::reply::json(&serde_json::json!({
                        "status": "watching",
                        "diner_id": req.diner_id,
                    }))
                } else {
                    warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": "Diner not found",
                    }))
                }
            });

        // POST /diner/interact - Handle diner interaction
        let session_interact = session.clone();
        let diner_interact = warp::path!("diner" / "interact")
            .and(warp::post())
            .and(warp::body::json())
            .map(move |req: InteractionRequest| {
                let diners = session_interact.diners.read().unwrap();
                if let Some(diner) = diners.iter().find(|d| d.diner_id == req.diner_id) {
                    diner.handle_interaction(
                        req.interaction_type,
                        req.cell_position,
                    );

                    warp::reply::json(&serde_json::json!({
                        "status": "interaction_queued",
                        "diner_id": req.diner_id,
                    }))
                } else {
                    warp::reply::json(&serde_json::json!({
                        "status": "error",
                        "message": "Diner not found",
                    }))
                }
            });

        // GET /stats - Network statistics
        let session_stats = session.clone();
        let stats = warp::path!("stats")
            .map(move || {
                let stats = session_stats.get_network_stats();
                warp::reply::json(&stats)
            });

        // GET /field/pulses - Get all field pulses (for debugging)
        let session_field = session.clone();
        let field_pulses = warp::path!("field" / "pulses")
            .map(move || {
                let pulses = session_field.network_field.field.get_all_pulses();
                warp::reply::json(&pulses)
            });

        // GET /cpux/state - Get CPUX state (Intention Space physics)
        let session_cpux = session.clone();
        let cpux_state = warp::path!("cpux" / "state")
            .map(move || {
                let cooks = session_cpux.cooks.read().unwrap();
                let diners = session_cpux.diners.read().unwrap();

                let cook_states: Vec<_> = cooks.iter().map(|c| {
                    serde_json::json!({
                        "cook_id": c.cook_id,
                        "cpux_id": c.cpux.id,
                        "progressor_delta_ns": c.cpux.progressor_delta_ns,
                        "field_pulses": c.cpux.field.get_all_pulses().len(),
                    })
                }).collect();

                let diner_states: Vec<_> = diners.iter().map(|d| {
                    serde_json::json!({
                        "diner_id": d.diner_id,
                        "cpux_id": d.cpux.id,
                        "progressor_delta_ns": d.cpux.progressor_delta_ns,
                        "field_pulses": d.cpux.field.get_all_pulses().len(),
                    })
                }).collect();

                warp::reply::json(&serde_json::json!({
                    "cooks": cook_states,
                    "diners": diner_states,
                    "network_field_pulses": session_cpux.network_field.field.get_all_pulses().len(),
                }))
            });

        // Combine all routes
        let routes = info
            .or(register_cook)
            .or(start_stream)
            .or(register_diner)
            .or(watch_stream)
            .or(diner_interact)
            .or(stats)
            .or(field_pulses)
            .or(cpux_state);

        println!("🚀 TapStream Server starting...");
        println!("   📡 WiFi: {}:{}", self.host, self.port);
        println!("   🎯 Session: {}", self.session.session_id);
        println!("   ⚡ Physics: Intention Space");
        println!();
        println!("Endpoints:");
        println!("  POST /cook/register    - Register as cook");
        println!("  POST /cook/stream      - Start streaming");
        println!("  POST /diner/register   - Register as diner");
        println!("  POST /diner/watch      - Watch stream");
        println!("  POST /diner/interact   - Interact with stream");
        println!("  GET  /stats            - Network statistics");
        println!("  GET  /field/pulses     - Field pulses (debug)");
        println!("  GET  /cpux/state       - CPUX state (debug)");
        println!();

        warp::serve(routes)
            .run(([0, 0, 0, 0], self.port))
            .await;
    }
}

// ============================================================================
// REQUEST/RESPONSE TYPES
// ============================================================================

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub device_id: String,
}

#[derive(Debug, Deserialize)]
pub struct StreamRequest {
    pub cook_id: String,
    pub metadata: FrameMetadata,
}

#[derive(Debug, Deserialize)]
pub struct WatchRequest {
    pub diner_id: String,
    pub cook_id: Option<String>, // Optional: watch specific cook
}

#[derive(Debug, Deserialize)]
pub struct InteractionRequest {
    pub diner_id: String,
    pub interaction_type: InteractionType,
    pub cell_position: CellPosition,
}

// ============================================================================
// WIFI HOTSPOT HELPER (Android-specific)
// ============================================================================

pub struct WiFiHotspot {
    pub ssid: String,
    pub password: String,
    pub ip_address: String,
}

impl WiFiHotspot {
    pub fn new(session_id: &str) -> Self {
        Self {
            ssid: format!("TapStream_{}", session_id),
            password: "tapstream123".to_string(),
            ip_address: "192.168.43.1".to_string(), // Common Android hotspot IP
        }
    }

    /// Display QR code for easy connection
    pub fn generate_qr_code(&self) -> String {
        // QR code format: WIFI:T:WPA;S:<SSID>;P:<PASSWORD>;;
        format!("WIFI:T:WPA;S:{};P:{};;", self.ssid, self.password)
    }

    pub fn get_connection_info(&self) -> ConnectionInfo {
        ConnectionInfo {
            ssid: self.ssid.clone(),
            password: self.password.clone(),
            server_url: format!("http://{}:8080", self.ip_address),
            qr_code: self.generate_qr_code(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct ConnectionInfo {
    pub ssid: String,
    pub password: String,
    pub server_url: String,
    pub qr_code: String,
}
