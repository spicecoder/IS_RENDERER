// ============================================================================
// POEM WRITER - egui + Intention Space Physics
// Grid Lookout: ONE CELL = ONE PULSE
// I-O-I-DN-I-GL-I Flow Pattern
// ============================================================================

use eframe::egui;
use egui::{Color32, Pos2, Stroke, Vec2};
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
use serde::{Deserialize, Serialize};

// ============================================================================
// INTENTION SPACE CORE STRUCTURES
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
pub struct DrawingStroke {
    pub points: Vec<(f32, f32)>,
    pub color: [u8; 4],  // RGBA
    pub width: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoemPage {
    pub id: String,
    pub page_number: usize,
    pub poem_text: String,
    pub notes_text: String,
    pub drawing_strokes: Vec<DrawingStroke>,
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
            drawing_strokes: Vec::new(),
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
    
    pub fn add_stroke(&mut self, stroke: DrawingStroke) {
        self.drawing_strokes.push(stroke);
        self.updated_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
    }
    
    pub fn clear_drawing(&mut self) {
        self.drawing_strokes.clear();
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
// CPUX - COMPUTATIONAL PATH
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
            DesignNode::new("add_stroke", "Add Drawing Stroke", |_| vec!["stroke_added".to_string()]),
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
    
    pub fn add_drawing_stroke(&self, stroke: DrawingStroke) {
        if let Ok(mut collection) = self.collection.lock() {
            if let Some(page) = collection.get_current_page_mut() {
                page.add_stroke(stroke);
            }
            let _ = collection.save_to_file();
        }
    }
    
    pub fn clear_drawing(&self) {
        if let Ok(mut collection) = self.collection.lock() {
            if let Some(page) = collection.get_current_page_mut() {
                page.clear_drawing();
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
                output.push_str(&format!("\n\nDRAWING: {} strokes\n\n", page.drawing_strokes.len()));
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
// EGUI APPLICATION - GRID LOOKOUT RENDERING
// ============================================================================

pub struct PoemWriterApp {
    cpux: Arc<PoemWriterCPUX>,
    
    // Local UI state (Grid Lookout bindings)
    poem_text: String,
    notes_text: String,
    current_stroke: Vec<Pos2>,
    
    // Drawing tools
    pen_color: Color32,
    pen_width: f32,
    eraser_mode: bool,
}

impl PoemWriterApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let save_path = std::env::var("HOME")
            .map(|home| format!("{}/poem_writer_data.json", home))
            .unwrap_or_else(|_| "poem_writer_data.json".to_string());
        
        println!("📝 Poem Writer - Intention Space");
        println!("💾 Save file: {}", save_path);
        
        let cpux = Arc::new(PoemWriterCPUX::new(save_path));
        
        // Load current page into Grid Lookout
        let (poem_text, notes_text) = if let Some(page) = cpux.get_current_page() {
            (page.poem_text, page.notes_text)
        } else {
            (String::new(), String::new())
        };
        
        Self {
            cpux,
            poem_text,
            notes_text,
            current_stroke: Vec::new(),
            pen_color: Color32::BLACK,
            pen_width: 2.0,
            eraser_mode: false,
        }
    }
    
    fn load_current_page(&mut self) {
        if let Some(page) = self.cpux.get_current_page() {
            self.poem_text = page.poem_text;
            self.notes_text = page.notes_text;
            println!("📄 Loaded Page {}: poem={} chars, notes={} chars, strokes={}", 
                     page.page_number, self.poem_text.len(), self.notes_text.len(), 
                     page.drawing_strokes.len());
        }
    }
}

impl eframe::App for PoemWriterApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // ====================================================================
        // TOP PANEL - Header with Page Navigation (Grid Lookout)
        // ====================================================================
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                ui.heading("✍️ Poem Writer");
                ui.label("Intention Space Computing");
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Delete button
                    if ui.button("🗑️").clicked() && self.cpux.get_page_count() > 1 {
                        self.cpux.delete_page();
                        self.load_current_page();
                    }
                    
                    // Add page button
                    if ui.button("➕").clicked() {
                        self.cpux.add_page();
                        self.load_current_page();
                    }
                    
                    // Next page button
                    if ui.button("→").clicked() {
                        if self.cpux.next_page() {
                            self.load_current_page();
                        }
                    }
                    
                    // Page indicator
                    ui.label(format!("Page {} of {}", 
                        self.cpux.get_current_page_number(), 
                        self.cpux.get_page_count()
                    ));
                    
                    // Previous page button
                    if ui.button("←").clicked() {
                        if self.cpux.prev_page() {
                            self.load_current_page();
                        }
                    }
                });
            });
            ui.add_space(8.0);
        });
        
        // ====================================================================
        // BOTTOM PANEL - Footer with Actions (Grid Lookout)
        // ====================================================================
        egui::TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                if ui.button("💾 Save").clicked() {
                    if let Ok(collection) = self.cpux.collection.lock() {
                        let _ = collection.save_to_file();
                    }
                }
                
                if ui.button("📤 Export").clicked() {
                    match self.cpux.export_to_text() {
                        Ok(msg) => println!("✅ {}", msg),
                        Err(e) => eprintln!("❌ {}", e),
                    }
                }
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("I-O-I-DN-I-GL-I Flow Active");
                });
            });
            ui.add_space(8.0);
        });
        
        // ====================================================================
        // CENTRAL PANEL - Split View (Grid Lookout: 2 columns)
        // ====================================================================
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                ui.horizontal(|ui| {
                    // ============================================================
                    // LEFT COLUMN - Poem Editor (Grid Cell 1)
                    // ============================================================
                    ui.group(|ui| {
                        ui.set_width(550.0);
                        ui.vertical(|ui| {
                            ui.heading("Your Poem");
                            ui.label("Write your verses here");
                            ui.separator();
                            
                            let response = egui::TextEdit::multiline(&mut self.poem_text)
                                .desired_width(530.0)
                                .desired_rows(30)
                                .font(egui::TextStyle::Monospace)
                                .show(ui);
                            
                            // I-O-I-DN-I-GL-I Flow: Text changed
                            if response.response.changed() {
                                self.cpux.update_poem_text(self.poem_text.clone());
                            }
                        });
                    });
                    
                    ui.add_space(10.0);
                    
                    // ============================================================
                    // RIGHT COLUMN - Notes + Drawing (Grid Cell 2)
                    // ============================================================
                    ui.group(|ui| {
                        ui.set_width(550.0);
                        ui.vertical(|ui| {
                    ui.heading("Notes & Sketches");
                    ui.label("Capture your thoughts and doodles");
                    ui.separator();
                    
                    // --------------------------------------------------------
                    // DRAWING AREA - Freehand Canvas
                    // --------------------------------------------------------
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("✏️ Drawing:");
                            
                            // Color picker
                            if ui.color_edit_button_srgba(&mut self.pen_color).changed() {
                                self.eraser_mode = false;
                            }
                            
                            // Pen width
                            ui.add(egui::Slider::new(&mut self.pen_width, 1.0..=10.0)
                                .text("Width"));
                            
                            // Eraser toggle
                            if ui.button(if self.eraser_mode { "🖊️ Pen" } else { "🧹 Eraser" }).clicked() {
                                self.eraser_mode = !self.eraser_mode;
                            }
                            
                            // Clear drawing
                            if ui.button("🗑️ Clear").clicked() {
                                self.cpux.clear_drawing();
                            }
                        });
                        
                        // Drawing canvas
                        let (response, painter) = ui.allocate_painter(
                            Vec2::new(ui.available_width(), 250.0),
                            egui::Sense::drag()
                        );
                        
                        // Background
                        painter.rect_filled(
                            response.rect,
                            0.0,
                            Color32::from_rgb(245, 245, 245)
                        );
                        
                        // Draw saved strokes from current page
                        if let Some(page) = self.cpux.get_current_page() {
                            for stroke in &page.drawing_strokes {
                                let points: Vec<Pos2> = stroke.points.iter()
                                    .map(|(x, y)| Pos2::new(*x, *y))
                                    .collect();
                                
                                if points.len() > 1 {
                                    let color = Color32::from_rgba_unmultiplied(
                                        stroke.color[0], stroke.color[1],
                                        stroke.color[2], stroke.color[3]
                                    );
                                    painter.add(egui::Shape::line(
                                        points,
                                        Stroke::new(stroke.width, color)
                                    ));
                                }
                            }
                        }
                        
                        // Draw current stroke being drawn
                        if self.current_stroke.len() > 1 {
                            let color = if self.eraser_mode {
                                Color32::from_rgb(245, 245, 245)
                            } else {
                                self.pen_color
                            };
                            
                            painter.add(egui::Shape::line(
                                self.current_stroke.clone(),
                                Stroke::new(self.pen_width, color)
                            ));
                        }
                        
                        // Handle drawing interaction
                        if response.dragged() {
                            if let Some(pos) = response.interact_pointer_pos() {
                                self.current_stroke.push(pos);
                            }
                        }
                        
                        // Finish stroke on release
                        if response.drag_released() && !self.current_stroke.is_empty() {
                            let color = if self.eraser_mode {
                                Color32::from_rgb(245, 245, 245)
                            } else {
                                self.pen_color
                            };
                            
                            let stroke = DrawingStroke {
                                points: self.current_stroke.iter()
                                    .map(|p| (p.x, p.y))
                                    .collect(),
                                color: color.to_array(),
                                width: self.pen_width,
                            };
                            
                            // I-O-I-DN-I-GL-I Flow: Add stroke
                            self.cpux.add_drawing_stroke(stroke);
                            self.current_stroke.clear();
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    // --------------------------------------------------------
                    // TEXT NOTES AREA
                    // --------------------------------------------------------
                    ui.label("📝 Text Notes:");
                    let response = egui::TextEdit::multiline(&mut self.notes_text)
                        .desired_width(f32::INFINITY)
                        .desired_rows(10)
                        .show(ui);
                    
                            // I-O-I-DN-I-GL-I Flow: Notes changed
                            if response.response.changed() {
                                self.cpux.update_notes_text(self.notes_text.clone());
                            }
                        });
                    });
                });
            });
        });
    }
}

// ============================================================================
// MAIN ENTRY POINT
// ============================================================================

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Poem Writer - Intention Space"),
        ..Default::default()
    };
    
    eframe::run_native(
        "Poem Writer",
        options,
        Box::new(|cc| Ok(Box::new(PoemWriterApp::new(cc)))),
    )
}