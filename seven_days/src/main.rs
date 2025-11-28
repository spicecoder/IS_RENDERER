// ============================================================================
// 7 DAYS FROM TODAY - MAIN APPLICATION
// Integrates Intention Space physics with Slint UI
// ============================================================================

mod intention_space;
mod data_store;

use chrono::{Datelike, Local};
use data_store::*;
use intention_space::*;
use slint::{ModelRc, VecModel};
use std::sync::{Arc, Mutex};

slint::include_modules!();

// ============================================================================
// APPLICATION STATE
// ============================================================================

struct AppState {
    store: Arc<Mutex<SevenDaysStore>>,
    space_loop: Arc<Mutex<SpaceLoop>>,
}

impl AppState {
    fn new() -> Self {
        // Load or create data store
        let storage_path = SevenDaysStore::get_storage_path();
        let store = SevenDaysStore::load(&storage_path)
            .unwrap_or_else(|_| SevenDaysStore::new());

        // Initialize Intention Space
        let mut space_loop = SpaceLoop::new();
        
        // Create CPUX for day management
        let day_cpux = create_day_management_cpux(space_loop.get_field());
        space_loop.add_cpux(day_cpux);

        Self {
            store: Arc::new(Mutex::new(store)),
            space_loop: Arc::new(Mutex::new(space_loop)),
        }
    }

    fn save(&self) {
        let store = self.store.lock().unwrap();
        let storage_path = SevenDaysStore::get_storage_path();
        if let Err(e) = store.save(&storage_path) {
            eprintln!("Failed to save data: {}", e);
        }
    }
}

// ============================================================================
// INTENTION SPACE CPUX SETUP
// ============================================================================

fn create_day_management_cpux(field: Arc<Mutex<NetworkField>>) -> CPUX {
    let mut cpux = CPUX::new("day_management", "Day Management CPUX", field);

    // Design Node: Load day data
    let load_day_dn = DesignNode::new(
        "load_day",
        "Load Day Data",
        |inputs| {
            // Computation: Load day data from store
            inputs.iter()
                .flat_map(|p| p.response.clone())
                .collect()
        },
    );
    cpux.add_design_node(load_day_dn);

    // Design Node: Update todo
    let update_todo_dn = DesignNode::new(
        "update_todo",
        "Update Todo State",
        |inputs| {
            // Computation: Toggle or remove todo
            inputs.iter()
                .flat_map(|p| p.response.clone())
                .collect()
        },
    );
    cpux.add_design_node(update_todo_dn);

    // Design Node: Update note
    let update_note_dn = DesignNode::new(
        "update_note",
        "Update Note Content",
        |inputs| {
            // Computation: Save note content
            inputs.iter()
                .flat_map(|p| p.response.clone())
                .collect()
        },
    );
    cpux.add_design_node(update_note_dn);

    // Object: Data mapper
    let data_mapper = IntentionObject::new(
        "data_mapper",
        "Data Transformation Mapper",
        |input| {
            // Static mapping: Transform data format
            let mut output = input.clone();
            output.name = format!("{}_mapped", input.name);
            output
        },
    );
    cpux.add_object(data_mapper);

    // Grid Lookouts for 7 days (7 columns)
    for day_index in 0..7 {
        let mut gl = GridLookout::new(
            &format!("day_{}", day_index),
            &format!("Day {} Display", day_index),
            0,  // layer
            0,  // row
            day_index,  // column
        );
        gl.bind_pulse(format!("day_{}_data", day_index));
        cpux.add_grid_lookout(gl);
    }

    cpux
}

// ============================================================================
// UI DATA CONVERSION
// ============================================================================

fn day_entry_to_ui(entry: &DayEntry) -> DayData {
    let icon = get_weekday_icon(entry.date.weekday());

    let todos: Vec<TodoItemData> = entry
        .todos
        .iter()
        .map(|todo| TodoItemData {
            id: todo.id.clone().into(),
            text: todo.text.clone().into(),
            completed: todo.completed,
        })
        .collect();

    let note_content = entry
        .notes
        .first()
        .map(|n| n.content.clone())
        .unwrap_or_default();

    DayData {
        date: entry.date.format("%b %d, %Y").to_string().into(),
        weekday: entry.weekday.clone().into(),
        icon: icon.into(),
        todos: ModelRc::new(VecModel::from(todos)),
        note_content: note_content.into(),
    }
}

// ============================================================================
// MAIN APPLICATION
// ============================================================================

fn main() -> Result<(), slint::PlatformError> {
    println!("🗓️  7 Days from Today");
    println!("   Physics: Intention Space I-O-I-DN-I-GL-I Flow");
    println!("   Grid: 7 columns = 7 days");
    println!();

    let app_state = Arc::new(AppState::new());
    let ui = MainWindow::new()?;

    // Load initial 7 days
    load_seven_days(&ui, &app_state);

    // ========================================================================
    // CALLBACKS - Intention Space interactions
    // ========================================================================

    // Day selected
    {
        let state = Arc::clone(&app_state);
        let ui_weak = ui.as_weak();
        ui.on_day_selected(move |day_index| {
            println!("📅 Day selected: {}", day_index);
            
            // Create intention pulse
            let space_loop = state.space_loop.lock().unwrap();
            let mut field = space_loop.network_field.lock().unwrap();
            let pulse = FieldPulse::new("day_selected")
                .with_response(vec![day_index.to_string()]);
            field.store_pulse(pulse);
            drop(field);
            drop(space_loop);

            // Update UI - reload all data to refresh notes/todos
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_selected_day(day_index);
                load_seven_days(&ui, &state);  // Reload data to get correct notes
            }
        });
    }

    // Add todo
    {
        let state = Arc::clone(&app_state);
        let ui_weak = ui.as_weak();
        ui.on_add_todo(move |day_index, text| {
            println!("➕ Add todo: {} (day {})", text, day_index);
            
            let today = Local::now().date_naive();
            let date = today + chrono::Duration::days(day_index as i64);
            
            {
                let mut store = state.store.lock().unwrap();
                let entry = store.get_or_create_entry(date);
                entry.add_todo(text.to_string());
            } // Release lock here
            
            state.save();
            
            // Update UI
            if let Some(ui) = ui_weak.upgrade() {
                load_seven_days(&ui, &state);
            }
        });
    }

    // Toggle todo
    {
        let state = Arc::clone(&app_state);
        let ui_weak = ui.as_weak();
        ui.on_toggle_todo(move |day_index, todo_id| {
            println!("✓ Toggle todo: {} (day {})", todo_id, day_index);
            
            let today = Local::now().date_naive();
            let date = today + chrono::Duration::days(day_index as i64);
            
            {
                let mut store = state.store.lock().unwrap();
                if let Some(entry) = store.entries.get_mut(&date.format("%Y-%m-%d").to_string()) {
                    entry.toggle_todo(&todo_id.to_string());
                }
            } // Release lock here
            
            state.save();
            
            // Update UI
            if let Some(ui) = ui_weak.upgrade() {
                load_seven_days(&ui, &state);
            }
        });
    }

    // Remove todo
    {
        let state = Arc::clone(&app_state);
        let ui_weak = ui.as_weak();
        ui.on_remove_todo(move |day_index, todo_id| {
            println!("🗑️  Remove todo: {} (day {})", todo_id, day_index);
            
            let today = Local::now().date_naive();
            let date = today + chrono::Duration::days(day_index as i64);
            
            {
                let mut store = state.store.lock().unwrap();
                if let Some(entry) = store.entries.get_mut(&date.format("%Y-%m-%d").to_string()) {
                    entry.remove_todo(&todo_id.to_string());
                }
            } // Release lock here
            
            state.save();
            
            // Update UI
            if let Some(ui) = ui_weak.upgrade() {
                load_seven_days(&ui, &state);
            }
        });
    }

    // Update note
    {
        let state = Arc::clone(&app_state);
        let _ui_weak = ui.as_weak();
        ui.on_update_note(move |day_index, content| {
            let today = Local::now().date_naive();
            let date = today + chrono::Duration::days(day_index as i64);
            
            {
                let mut store = state.store.lock().unwrap();
                let entry = store.get_or_create_entry(date);
                
                // Remove old note and add new one
                if !entry.notes.is_empty() {
                    let note_id = entry.notes[0].id.clone();
                    entry.update_note(&note_id, content.to_string());
                } else {
                    entry.add_note(content.to_string());
                }
            } // Release lock here
            
            state.save();
        });
    }

    // Show previous weekdays
    {
        let state = Arc::clone(&app_state);
        let ui_weak = ui.as_weak();
        ui.on_previous_weekday(move |day_index| {
            println!("◀ Show previous weekdays for day {}", day_index);
            
            let today = Local::now().date_naive();
            let date = today + chrono::Duration::days(day_index as i64);
            let weekday = date.weekday();
            
            let store = state.store.lock().unwrap();
            let previous = store.get_previous_weekdays(weekday, date);
            
            if let Some(ui) = ui_weak.upgrade() {
                let previous_ui: Vec<DayData> = previous
                    .iter()
                    .map(day_entry_to_ui)
                    .collect();
                
                ui.set_previous_days(ModelRc::new(VecModel::from(previous_ui)));
                ui.set_show_previous_mode(true);
            }
        });
    }

    // Next week
    {
        let state = Arc::clone(&app_state);
        let ui_weak = ui.as_weak();
        ui.on_next_week(move || {
            println!("▶ Advance to next week");
            
            // This would require date shifting logic
            // For now, just reload current week
            if let Some(ui) = ui_weak.upgrade() {
                load_seven_days(&ui, &state);
            }
        });
    }

    println!("✅ App initialized successfully!");
    println!("   I-O-I-DN-I-GL-I flow active");
    println!();

    ui.run()
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

fn load_seven_days(ui: &MainWindow, state: &AppState) {
    let mut store = state.store.lock().unwrap();
    let days = store.get_next_seven_days();
    
    let days_ui: Vec<DayData> = days.iter().map(day_entry_to_ui).collect();
    
    ui.set_days(ModelRc::new(VecModel::from(days_ui)));
    
    println!("📊 Loaded 7 days:");
    for (i, day) in days.iter().enumerate() {
        println!("   {} - {} ({} todos, {} notes)", 
            i, 
            day.weekday,
            day.todos.len(),
            day.notes.len()
        );
    }
}