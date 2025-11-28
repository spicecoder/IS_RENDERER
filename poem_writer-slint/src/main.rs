// ============================================================================
// POEM WRITER - With Forced UI Refresh
// Fixes: Visual display updates when navigating pages
// ============================================================================

use slint::ComponentHandle;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use serde::{Deserialize, Serialize};

// ============================================================================
// INTENTION SPACE CORE (unchanged from before)
// ============================================================================

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TV { Y, N, U }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldPulse {
    pub id: String,
    pub name: String,
    pub tv: TV,
    pub response: Vec<String>,
    pub timestamp: u64,
}

impl FieldPulse {
    pub fn new(name: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        Self {
            id: format!("{}_{}", name, timestamp),
            name: name.to_string(),
            tv: TV::U,
            response: vec![],
            timestamp,
        }
    }
    
    pub fn with_response(mut self, response: Vec<String>) -> Self {
        self.response = response;
        self
    }
    
    pub fn with_tv(mut self, tv: TV) -> Self {
        self.tv = tv;
        self
    }
}

pub struct DesignNode {
    pub id: String,
    pub name: String,
    pub compute: Box<dyn Fn(&[FieldPulse]) -> Vec<String> + Send + Sync>,
}

impl DesignNode {
    pub fn new<F>(id: &str, name: &str, compute: F) -> Self
    where F: Fn(&[FieldPulse]) -> Vec<String> + Send + Sync + 'static
    {
        Self {
            id: id.to_string(),
            name: name.to_string(),
            compute: Box::new(compute),
        }
    }
    
    pub fn execute(&self, inputs: &[FieldPulse]) -> FieldPulse {
        let result = (self.compute)(inputs);
        FieldPulse::new(&format!("{}_result", self.id))
            .with_response(result)
            .with_tv(TV::Y)
    }
}

pub struct ObjectMapping {
    pub id: String,
    pub transform: Box<dyn Fn(&FieldPulse) -> FieldPulse + Send + Sync>,
}

impl ObjectMapping {
    pub fn new<F>(id: &str, transform: F) -> Self
    where F: Fn(&FieldPulse) -> FieldPulse + Send + Sync + 'static
    {
        Self {
            id: id.to_string(),
            transform: Box::new(transform),
        }
    }
    
    pub fn apply(&self, input: &FieldPulse) -> FieldPulse {
        (self.transform)(input)
    }
}

// ============================================================================
// POEM WRITER DOMAIN MODEL
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoemPage {
    pub id: String,
    pub page_number: usize,
    pub poem_text: String,
    pub notes_text: String,
    pub created_at: u64,
    pub updated_at: u64,
}

impl PoemPage {
    pub fn new(page_number: usize) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        
        Self {
            id: format!("page_{}", page_number),
            page_number,
            poem_text: String::new(),
            notes_text: String::new(),
            created_at: timestamp,
            updated_at: timestamp,
        }
    }
    
    pub fn update_poem(&mut self, text: String) {
        self.poem_text = text;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }
    
    pub fn update_notes(&mut self, text: String) {
        self.notes_text = text;
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoemCollection {
    pub pages: Vec<PoemPage>,
    pub current_page: usize,
    pub file_path: String,
}

impl PoemCollection {
    pub fn new(file_path: String) -> Self {
        Self {
            pages: vec![PoemPage::new(1)],
            current_page: 0,
            file_path,
        }
    }
    
    pub fn add_page(&mut self) -> usize {
        let page_number = self.pages.len() + 1;
        self.pages.push(PoemPage::new(page_number));
        self.current_page = self.pages.len() - 1;
        page_number
    }
    
    pub fn delete_page(&mut self) -> bool {
        if self.pages.len() <= 1 {
            return false;
        }
        
        self.pages.remove(self.current_page);
        
        if self.current_page >= self.pages.len() {
            self.current_page = self.pages.len() - 1;
        }
        
        for (idx, page) in self.pages.iter_mut().enumerate() {
            page.page_number = idx + 1;
            page.id = format!("page_{}", idx + 1);
        }
        
        true
    }
    
    pub fn get_current_page(&self) -> Option<&PoemPage> {
        self.pages.get(self.current_page)
    }
    
    pub fn get_current_page_mut(&mut self) -> Option<&mut PoemPage> {
        self.pages.get_mut(self.current_page)
    }
    
    pub fn next_page(&mut self) -> bool {
        if self.current_page < self.pages.len() - 1 {
            self.current_page += 1;
            true
        } else {
            false
        }
    }
    
    pub fn prev_page(&mut self) -> bool {
        if self.current_page > 0 {
            self.current_page -= 1;
            true
        } else {
            false
        }
    }
    
    pub fn save_to_file(&self) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self)?;
        fs::write(&self.file_path, json)?;
        println!("✅ Saved to: {}", self.file_path);
        Ok(())
    }
    
    pub fn load_from_file(file_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        if std::path::Path::new(&file_path).exists() {
            let json = fs::read_to_string(&file_path)?;
            let mut collection: PoemCollection = serde_json::from_str(&json)?;
            collection.file_path = file_path;
            println!("✅ Loaded from: {}", collection.file_path);
            Ok(collection)
        } else {
            println!("📝 Creating new collection at: {}", file_path);
            Ok(PoemCollection::new(file_path))
        }
    }
}

// ============================================================================
// CPUX
// ============================================================================

pub struct PoemWriterCPUX {
    pub id: String,
    pub collection: Arc<Mutex<PoemCollection>>,
    pub design_nodes: Vec<DesignNode>,
    pub objects: Vec<ObjectMapping>,
    pub network_field: Arc<Mutex<Vec<FieldPulse>>>,
}

impl PoemWriterCPUX {
    pub fn new(file_path: String) -> Self {
        let collection = match PoemCollection::load_from_file(file_path.clone()) {
            Ok(c) => Arc::new(Mutex::new(c)),
            Err(e) => {
                eprintln!("⚠️  Could not load file: {}, creating new", e);
                Arc::new(Mutex::new(PoemCollection::new(file_path)))
            }
        };
        
        let design_nodes = vec![
            DesignNode::new("validate_poem", "Validate Poem Text", |inputs| {
                if let Some(pulse) = inputs.first() {
                    let empty = String::new();
                    let text = pulse.response.first().unwrap_or(&empty);
                    vec![text.to_string()]
                } else {
                    vec![String::new()]
                }
            }),
            DesignNode::new("validate_notes", "Validate Notes Text", |inputs| {
                if let Some(pulse) = inputs.first() {
                    let empty = String::new();
                    let text = pulse.response.first().unwrap_or(&empty);
                    vec![text.to_string()]
                } else {
                    vec![String::new()]
                }
            }),
            DesignNode::new("update_poem", "Update Poem", |_| vec!["updated".to_string()]),
            DesignNode::new("update_notes", "Update Notes", |_| vec!["updated".to_string()]),
            DesignNode::new("create_page", "Create Page", |_| vec!["created".to_string()]),
            DesignNode::new("delete_page", "Delete Page", |_| vec!["deleted".to_string()]),
        ];
        
        let objects = vec![
            ObjectMapping::new("route_to_validation", |input| {
                FieldPulse::new("validation_request")
                    .with_response(input.response.clone())
                    .with_tv(TV::U)
            }),
            ObjectMapping::new("reflect_validation_result", |input| {
                FieldPulse::new("validation_success")
                    .with_response(input.response.clone())
                    .with_tv(TV::Y)
            }),
        ];
        
        Self {
            id: "poem_writer_cpux".to_string(),
            collection,
            design_nodes,
            objects,
            network_field: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    pub fn update_poem_text(&self, text: String) {
        if let Ok(mut collection) = self.collection.lock() {
            if let Some(page) = collection.get_current_page_mut() {
                page.update_poem(text);
            }
            let _ = collection.save_to_file();
        }
    }
    
    pub fn update_notes_text(&self, text: String) {
        if let Ok(mut collection) = self.collection.lock() {
            if let Some(page) = collection.get_current_page_mut() {
                page.update_notes(text);
            }
            let _ = collection.save_to_file();
        }
    }
    
    pub fn add_page(&self) -> usize {
        if let Ok(mut collection) = self.collection.lock() {
            let num = collection.add_page();
            let _ = collection.save_to_file();
            num
        } else {
            0
        }
    }
    
    pub fn delete_page(&self) -> bool {
        if let Ok(mut collection) = self.collection.lock() {
            let success = collection.delete_page();
            if success {
                let _ = collection.save_to_file();
            }
            success
        } else {
            false
        }
    }
    
    pub fn next_page(&self) -> bool {
        if let Ok(mut collection) = self.collection.lock() {
            collection.next_page()
        } else {
            false
        }
    }
    
    pub fn prev_page(&self) -> bool {
        if let Ok(mut collection) = self.collection.lock() {
            collection.prev_page()
        } else {
            false
        }
    }
    
    pub fn get_current_page(&self) -> Option<PoemPage> {
        if let Ok(collection) = self.collection.lock() {
            collection.get_current_page().cloned()
        } else {
            None
        }
    }
    
    pub fn get_page_count(&self) -> usize {
        if let Ok(collection) = self.collection.lock() {
            collection.pages.len()
        } else {
            0
        }
    }
    
    pub fn get_current_page_number(&self) -> usize {
        if let Ok(collection) = self.collection.lock() {
            collection.current_page + 1
        } else {
            0
        }
    }
    
    pub fn save(&self) -> Result<(), String> {
        if let Ok(collection) = self.collection.lock() {
            collection.save_to_file()
                .map_err(|e| format!("Save failed: {}", e))
        } else {
            Err("Could not lock collection".to_string())
        }
    }
    
    pub fn export_to_text(&self) -> Result<String, String> {
        if let Ok(collection) = self.collection.lock() {
            let mut output = String::new();
            output.push_str("=====================================\n");
            output.push_str("       POEM COLLECTION\n");
            output.push_str("=====================================\n\n");
            
            for page in &collection.pages {
                output.push_str(&format!("--- Page {} ---\n\n", page.page_number));
                output.push_str("POEM:\n");
                output.push_str(&page.poem_text);
                output.push_str("\n\nNOTES:\n");
                output.push_str(&page.notes_text);
                output.push_str("\n\n");
            }
            
            let export_path = "poem_export.txt";
            fs::write(export_path, &output)
                .map_err(|e| format!("Export failed: {}", e))?;
            
            Ok(format!("Exported to {}", export_path))
        } else {
            Err("Could not lock collection".to_string())
        }
    }
}

// ============================================================================
// SLINT INTEGRATION with FORCED REFRESH
// ============================================================================

slint::include_modules!();

// Helper function to force UI refresh
fn refresh_ui(ui: &PoemWriter, cpux: &Arc<PoemWriterCPUX>) {
    if let Some(page) = cpux.get_current_page() {
        // Clone the strings before moving them
        let poem = page.poem_text.clone();
        let notes = page.notes_text.clone();
        let poem_len = poem.len();
        let notes_len = notes.len();
        
        // CRITICAL: Set to empty first to force TextEdit to refresh
        ui.set_poem_text("".into());
        ui.set_notes_text("".into());
        
        // Then set the actual content
        ui.set_poem_text(poem.into());
        ui.set_notes_text(notes.into());
        ui.set_page_number(page.page_number as i32);
        ui.set_total_pages(cpux.get_page_count() as i32);
        
        println!("📄 Loaded Page {}: poem={} chars, notes={} chars", 
                 page.page_number, poem_len, notes_len);
    }
}

pub fn main() {
    let save_path = std::env::var("HOME")
        .map(|home| format!("{}/poem_writer_data.json", home))
        .unwrap_or_else(|_| "poem_writer_data.json".to_string());
    
    println!("📝 Poem Writer - Intention Space");
    println!("💾 Save file: {}", save_path);
    
    let cpux = Arc::new(PoemWriterCPUX::new(save_path));
    let ui = PoemWriter::new().unwrap();
    
    // Initial load
    refresh_ui(&ui, &cpux);
    
    // Poem text changed
    let cpux_clone = cpux.clone();
    ui.on_poem_text_changed(move |text| {
        cpux_clone.update_poem_text(text.to_string());
    });
    
    // Notes text changed
    let cpux_clone = cpux.clone();
    ui.on_notes_text_changed(move |text| {
        cpux_clone.update_notes_text(text.to_string());
    });
    
    // Add page
    let cpux_clone = cpux.clone();
    let ui_weak = ui.as_weak();
    ui.on_add_page(move || {
        cpux_clone.add_page();
        if let Some(ui) = ui_weak.upgrade() {
            refresh_ui(&ui, &cpux_clone);
        }
    });
    
    // Delete page
    let cpux_clone = cpux.clone();
    let ui_weak = ui.as_weak();
    ui.on_delete_page(move || {
        if cpux_clone.delete_page() {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_ui(&ui, &cpux_clone);
            }
        }
    });
    
    // Next page - WITH FORCED REFRESH
    let cpux_clone = cpux.clone();
    let ui_weak = ui.as_weak();
    ui.on_next_page(move || {
        if cpux_clone.next_page() {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_ui(&ui, &cpux_clone);
            }
        }
    });
    
    // Previous page - WITH FORCED REFRESH
    let cpux_clone = cpux.clone();
    let ui_weak = ui.as_weak();
    ui.on_prev_page(move || {
        if cpux_clone.prev_page() {
            if let Some(ui) = ui_weak.upgrade() {
                refresh_ui(&ui, &cpux_clone);
            }
        }
    });
    
    // Save button
    let cpux_clone = cpux.clone();
    ui.on_save(move || {
        match cpux_clone.save() {
            Ok(_) => println!("✅ Manual save successful"),
            Err(e) => eprintln!("❌ Save error: {}", e),
        }
    });
    
    // Export button
    let cpux_clone = cpux.clone();
    ui.on_export(move || {
        match cpux_clone.export_to_text() {
            Ok(msg) => println!("✅ {}", msg),
            Err(e) => eprintln!("❌ Export error: {}", e),
        }
    });
    
    ui.run().unwrap();
}