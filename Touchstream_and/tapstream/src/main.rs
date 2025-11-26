// ============================================================================
// TAPSTREAM MAIN APPLICATION
// Integrating Slint UI with Intention Space Physics
// ============================================================================

mod intention_space;
mod tapstream;
mod server;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use slint::{ComponentHandle, ModelRc, VecModel, Timer, TimerMode};

slint::include_modules!();

use intention_space::*;
use tapstream::*;
use server::*;

// Global window storage to keep windows alive
thread_local! {
    static COOK_WINDOW: RefCell<Option<CookPreview>> = RefCell::new(None);
    static DINER_WINDOW: RefCell<Option<DinerStream>> = RefCell::new(None);
    static TIMERS: RefCell<Vec<Rc<Timer>>> = RefCell::new(Vec::new());
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

#[tokio::main]
async fn main() {
    println!("🚀 TapStream - Intention Space Live Streaming");
    println!("   Physics: I-O-I-DN-I-GL-I Flow");
    println!("   Grid: ONE CELL = ONE PULSE");
    println!();

    // Initialize app
    let app = TapStreamApp::new().unwrap();
    
    // Session state
    let session_id = format!("session_{}", chrono::Utc::now().timestamp());
    
    // Server
    let server = Arc::new(TapStreamServer::new(session_id.clone(), 8080));
    let server_clone = server.clone();
    
    // WiFi hotspot
    let hotspot = Arc::new(WiFiHotspot::new(&session_id));
    
    // Handle role selection
    let app_weak = app.as_weak();
    let server_role = server.clone();
    let hotspot_clone = hotspot.clone();
    
    app.on_select_role(move |role| {
        let role_str = role.to_string();
        println!("📱 Selected role: {}", role_str);
        
        let _app_handle = app_weak.upgrade().unwrap();
        let server = server_role.clone();
        let hotspot = hotspot_clone.clone();
        
        match role_str.as_str() {
            "cook" => {
                // Launch cook view without nested event loop
                launch_cook_view_inline(server);
            }
            "diner" => {
                // Launch diner view without nested event loop
                launch_diner_view_inline(server);
            }
            "setup" => {
                // Show WiFi setup information inline
                show_wifi_info(hotspot);
            }
            _ => {}
        }
    });
    
    // Start server in background
    tokio::spawn(async move {
        server_clone.start().await;
    });
    
    // Run UI (single event loop)
    println!("🎬 Starting event loop...");
    app.run().unwrap();
    println!("🛑 Event loop ended");
}

// ============================================================================
// COOK VIEW - INLINE (NO NESTED LOOP)
// ============================================================================

fn launch_cook_view_inline(server: Arc<TapStreamServer>) {
    println!("🔧 Creating cook window...");
    
    let cook_id = format!("cook_{}", chrono::Utc::now().timestamp_millis());
    
    // Register cook
    let cook = server.session.add_cook(&cook_id);
    
    // Show cook preview window
    let cook_preview = CookPreview::new().unwrap();
    
    println!("🔧 Configuring cook window...");
    cook_preview.set_cook_id(cook_id.clone().into());
    cook_preview.set_is_streaming(false);
    
    // Initialize grid cells
    let grid_cells = create_cook_grid_cells();
    cook_preview.set_grid_cells(grid_cells.into());
    
    // Handle start stream
    let cook_clone = cook.clone();
    let preview_weak = cook_preview.as_weak();
    
    cook_preview.on_start_stream(move || {
        if let Some(preview) = preview_weak.upgrade() {
            preview.set_is_streaming(true);
            println!("🎥 Starting stream...");
            
            let cook_stream = cook_clone.clone();
            let metadata = FrameMetadata {
                dish_name: "Spaghetti Carbonara".to_string(),
                cooking_stage: "Preparing sauce".to_string(),
                temperature: Some(65.0),
                timer_seconds: Some(300),
            };
            
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async move {
                    cook_stream.start_streaming(metadata).await;
                });
            });
        }
    });
    
    // Handle stop stream
    let preview_weak2 = cook_preview.as_weak();
    cook_preview.on_stop_stream(move || {
        if let Some(preview) = preview_weak2.upgrade() {
            preview.set_is_streaming(false);
            println!("⏸️ Stream stopped");
        }
    });
    
    // CRITICAL: Store window BEFORE showing it
    println!("🔧 Storing cook window globally...");
    COOK_WINDOW.with(|window| {
        *window.borrow_mut() = Some(cook_preview.clone_strong());
    });
    
    // Update grid periodically using Slint Timer (stays on main thread)
    let cook_gl = cook.preview_gl.clone();
    let preview_weak3 = cook_preview.as_weak();
    
    let timer = Rc::new(Timer::default());
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
        if let Some(preview) = preview_weak3.upgrade() {
            let gl = cook_gl.read().unwrap();
            let cells = convert_gl_to_slint_cells(&gl);
            preview.set_grid_cells(cells.into());
        }
    });
    
    // Store timer globally to keep it alive
    TIMERS.with(|timers| {
        timers.borrow_mut().push(timer);
    });
    
    // Show window NOW (after storing it)
    println!("🔧 Showing cook window...");
    cook_preview.show().unwrap();
    
    println!("✅ Cook window opened and active");
    println!("🔍 Window visible: {}", cook_preview.window().is_visible());
    println!("🔍 Strong count in storage: {}", COOK_WINDOW.with(|w| w.borrow().is_some()));
}

// ============================================================================
// DINER VIEW - INLINE (NO NESTED LOOP)
// ============================================================================

fn launch_diner_view_inline(server: Arc<TapStreamServer>) {
    let diner_id = format!("diner_{}", chrono::Utc::now().timestamp_millis());
    
    // Register diner
    let diner = server.session.add_diner(&diner_id);
    
    // Show diner stream window
    let diner_stream = DinerStream::new().unwrap();
    diner_stream.set_diner_id(diner_id.clone().into());
    diner_stream.set_is_connected(false);
    
    // Initialize grid cells
    let grid_cells = create_diner_grid_cells();
    diner_stream.set_grid_cells(grid_cells.into());
    
    // Handle connect
    let diner_clone = diner.clone();
    let stream_weak = diner_stream.as_weak();
    
    diner_stream.on_connect_to_stream(move |_cook_id| {
        if let Some(stream) = stream_weak.upgrade() {
            stream.set_is_connected(true);
            println!("📡 Connecting to stream...");
            
            // Start receiving in background thread
            let diner_recv = diner_clone.clone();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async move {
                    diner_recv.start_receiving().await;
                });
            });
            
            // Start interaction loop
            let diner_interact = diner_clone.clone();
            std::thread::spawn(move || {
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async move {
                    diner_interact.start_interaction_loop().await;
                });
            });
        }
    });
    
    // Handle disconnect
    let stream_weak2 = diner_stream.as_weak();
    diner_stream.on_disconnect_stream(move || {
        if let Some(stream) = stream_weak2.upgrade() {
            stream.set_is_connected(false);
            println!("🔌 Disconnected");
        }
    });
    
    // Handle interactions
    let diner_interact = diner.clone();
    diner_stream.on_cell_interaction(move |pulse_id| {
        println!("👆 Cell interaction: {}", pulse_id);
        
        diner_interact.handle_interaction(
            InteractionType::Tap,
            CellPosition { layer: 0, row: 0, col: 0 },
        );
    });
    
    // CRITICAL: Store window BEFORE showing it
    DINER_WINDOW.with(|window| {
        *window.borrow_mut() = Some(diner_stream.clone_strong());
    });
    
    // Update grid periodically using Slint Timer (stays on main thread)
    let diner_gl = diner.stream_gl.clone();
    let stream_weak3 = diner_stream.as_weak();
    
    let timer = Rc::new(Timer::default());
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(100), move || {
        if let Some(stream) = stream_weak3.upgrade() {
            let gl = diner_gl.read().unwrap();
            let cells = convert_gl_to_slint_cells(&gl);
            stream.set_grid_cells(cells.into());
        }
    });
    
    // Store timer globally to keep it alive
    TIMERS.with(|timers| {
        timers.borrow_mut().push(timer);
    });
    
    // Show window
    diner_stream.show().unwrap();
    
    println!("✅ Diner window opened and active");
}

// ============================================================================
// WIFI SETUP - SHOW INFO
// ============================================================================

fn show_wifi_info(hotspot: Arc<WiFiHotspot>) {
    let conn_info = hotspot.get_connection_info();
    
    println!();
    println!("📶 WiFi Hotspot Configuration");
    println!("============================");
    println!("SSID:     {}", conn_info.ssid);
    println!("Password: {}", conn_info.password);
    println!("Server:   {}", conn_info.server_url);
    println!();
    println!("QR Code: {}", conn_info.qr_code);
    println!();
    println!("📱 On Android, this would:");
    println!("   1. Enable WiFi hotspot");
    println!("   2. Configure network");
    println!("   3. Start HTTP server");
    println!();
}

// ============================================================================
// GRID CELL CONVERSION HELPERS
// ============================================================================

fn create_cook_grid_cells() -> ModelRc<GridCellData> {
    let cells = Rc::new(VecModel::default());
    
    for row in 0..3 {
        for col in 0..3 {
            cells.push(GridCellData {
                pulse_id: format!("preview_{}_{}", row, col).into(),
                pulse_name: format!("Cell [{},{}]", row, col).into(),
                pulse_value: "".into(),
                pulse_tv: "U".into(),
                layer: 0,
                row: row as i32,
                col: col as i32,
            });
        }
    }
    
    ModelRc::new(cells)
}

fn create_diner_grid_cells() -> ModelRc<GridCellData> {
    let cells = Rc::new(VecModel::default());
    
    let cell_data = vec![
        // Row 0 (indices 0-3)
        ("video_display", "Stream Display", "U"),
        ("video_control_1", "", "U"),
        ("video_control_2", "", "U"),
        ("video_control_3", "", "U"),
        // Row 1 (indices 4-7)
        ("dish_name", "Dish", "U"),
        ("cooking_stage", "Stage", "U"),
        ("temperature", "Temp", "U"),
        ("timer", "Timer", "U"),
        // Row 2 (indices 8-11)
        ("btn_like", "❤️ Like", "U"),
        ("btn_comment", "💬 Comment", "U"),
        ("btn_share", "🔗 Share", "U"),
        ("btn_bookmark", "🔖 Save", "U"),
        // Row 3 (indices 12-15)
        ("viewer_count", "Viewers", "U"),
        ("connection_status", "Status", "U"),
        ("quality", "Quality", "U"),
        ("latency", "Latency", "U"),
    ];
    
    for (idx, (id, name, tv)) in cell_data.iter().enumerate() {
        let row = idx / 4;
        let col = idx % 4;
        
        cells.push(GridCellData {
            pulse_id: id.to_string().into(),
            pulse_name: name.to_string().into(),
            pulse_value: "".into(),
            pulse_tv: tv.to_string().into(),
            layer: 0,
            row: row as i32,
            col: col as i32,
        });
    }
    
    ModelRc::new(cells)
}

fn convert_gl_to_slint_cells(gl: &GridLookout) -> ModelRc<GridCellData> {
    let cells = Rc::new(VecModel::default());
    
    for layer in &gl.layers {
        for (row_idx, row) in layer.cells.iter().enumerate() {
            for (col_idx, cell) in row.iter().enumerate() {
                let tv_str = match cell.pulse.tv {
                    TV::Y => "Y",
                    TV::N => "N",
                    TV::U => "U",
                };
                
                cells.push(GridCellData {
                    pulse_id: cell.pulse.id.clone().into(),
                    pulse_name: cell.pulse.name.clone().into(),
                    pulse_value: cell.pulse.response.clone().into(),
                    pulse_tv: tv_str.into(),
                    layer: layer.z_index as i32,
                    row: row_idx as i32,
                    col: col_idx as i32,
                });
            }
        }
    }
    
    ModelRc::new(cells)
}

// ============================================================================
// NETWORK STATS MONITOR
// ============================================================================

#[allow(dead_code)]
async fn monitor_network_stats(server: Arc<TapStreamServer>) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
    
    loop {
        interval.tick().await;
        
        let stats = server.session.get_network_stats();
        println!("📊 Network Stats:");
        println!("   Active pulses: {}", stats.active_pulses);
        println!("   Active cooks: {}", stats.active_cooks);
        println!("   Active diners: {}", stats.active_diners);
        println!();
    }
}

//slint::invoke_from_event_loop with time delay