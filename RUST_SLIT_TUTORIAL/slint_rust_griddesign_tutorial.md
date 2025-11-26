# Slint + Rust GUI Programming for GridDesign
## A Practical Tutorial for Cross-Platform Universal UI Positioning

---

## Table of Contents
1. [Why Slint for GridDesign?](#why-slint)
2. [Setup & Environment](#setup)
3. [Slint Fundamentals](#fundamentals)
4. [Grid Layout System](#grid-layout)
5. [GridDesign Integration](#griddesign-integration)
6. [Cross-Platform Deployment](#deployment)
7. [Advanced Patterns](#advanced-patterns)
8. [Real-World Examples](#examples)

---

## Why Slint for GridDesign? {#why-slint}

Slint is **perfect** for GridDesign because:

### ✅ Native Performance
- Compiles to native code on each platform
- No WebView overhead like Electron
- Hardware-accelerated rendering

### ✅ Declarative UI Language
- `.slint` files use familiar syntax (like QML/CSS)
- Separates UI from business logic
- Easy to generate from JSON schemas

### ✅ True Cross-Platform
- **Desktop**: Windows, macOS, Linux
- **Mobile**: Android, iOS (via Rust FFI)
- **Embedded**: Microcontrollers, IoT devices
- **Web**: WASM support

### ✅ Perfect for GridDesign Philosophy
```slint
// Slint naturally expresses grid concepts
GridLayout {
    spacing: 10px;
    Row {
        cell1 := Rectangle { /* content */ }
        cell2 := Rectangle { /* content */ }
    }
}
```

---

## Setup & Environment {#setup}

### Install Rust
```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version
```

### Create Your First Slint Project
```bash
# Create new project
cargo new griddesign-slint
cd griddesign-slint

# Add Slint dependency to Cargo.toml
```

**Cargo.toml:**
```toml
[package]
name = "griddesign-slint"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = "1.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
slint-build = "1.3"
```

**build.rs:**
```rust
fn main() {
    slint_build::compile("ui/app_window.slint").unwrap();
}
```

### IDE Setup
```bash
# For VS Code, install the Slint extension
# Search for "Slint" in VS Code extensions

# For other editors, LSP support is available
```

---

## Slint Fundamentals {#fundamentals}

### 1. Basic Window Structure

**ui/app_window.slint:**
```slint
import { VerticalBox, HorizontalBox } from "std-widgets.slint";

export component AppWindow inherits Window {
    title: "GridDesign with Slint";
    preferred-width: 800px;
    preferred-height: 600px;

    VerticalBox {
        Text {
            text: "Hello GridDesign!";
            font-size: 24px;
        }
    }
}
```

**src/main.rs:**
```rust
slint::include_modules!();

fn main() {
    let ui = AppWindow::new().unwrap();
    ui.run().unwrap();
}
```

### 2. Properties & Data Binding

```slint
export component Counter inherits Window {
    // Property declaration
    in-out property <int> counter: 0;
    
    VerticalBox {
        Text {
            // Data binding - updates automatically
            text: "Count: \{counter}";
        }
        
        Button {
            text: "Increment";
            clicked => {
                // Property modification
                counter += 1;
            }
        }
    }
}
```

### 3. Callbacks (Events)

```slint
export component EventDemo inherits Window {
    // Define callbacks
    callback button-clicked(string);
    callback value-changed(int);
    
    Button {
        text: "Click Me";
        clicked => {
            // Emit callback
            button-clicked("Button was clicked!");
        }
    }
}
```

**Rust side:**
```rust
slint::include_modules!();

fn main() {
    let ui = EventDemo::new().unwrap();
    
    // Handle callbacks
    ui.on_button_clicked(|message| {
        println!("Received: {}", message);
    });
    
    ui.on_value_changed(|value| {
        println!("Value changed to: {}", value);
    });
    
    ui.run().unwrap();
}
```

### 4. Conditional Rendering

```slint
export component Conditional inherits Window {
    in-out property <bool> show-content: false;
    
    VerticalBox {
        Button {
            text: show-content ? "Hide" : "Show";
            clicked => { show-content = !show-content; }
        }
        
        if show-content: Rectangle {
            background: lightblue;
            height: 100px;
            Text { text: "Conditional Content"; }
        }
    }
}
```

### 5. Loops & Repeaters

```slint
export component ListDemo inherits Window {
    in-out property <[string]> items: ["Apple", "Banana", "Cherry"];
    
    VerticalBox {
        for item in items: Rectangle {
            height: 40px;
            background: lightgray;
            
            Text {
                text: item;
                vertical-alignment: center;
            }
        }
    }
}
```

---

## Grid Layout System {#grid-layout}

### Native Slint Grid

**ui/grid_basic.slint:**
```slint
import { GridBox } from "std-widgets.slint";

export component BasicGrid inherits Window {
    preferred-width: 600px;
    preferred-height: 400px;
    
    GridBox {
        // Grid configuration
        spacing: 10px;
        padding: 20px;
        
        // Row 1
        Row {
            Rectangle {
                background: red;
                colspan: 2;  // Span 2 columns
                
                Text {
                    text: "Header (spans 2)";
                    color: white;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
        }
        
        // Row 2
        Row {
            Rectangle {
                background: blue;
                rowspan: 2;  // Span 2 rows
                
                Text {
                    text: "Sidebar\n(spans 2 rows)";
                    color: white;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
            
            Rectangle {
                background: green;
                
                Text {
                    text: "Content 1";
                    color: white;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
        }
        
        // Row 3
        Row {
            Rectangle {
                background: orange;
                
                Text {
                    text: "Content 2";
                    color: white;
                    horizontal-alignment: center;
                    vertical-alignment: center;
                }
            }
        }
    }
}
```

### GridDesign-Style Positioning

**ui/griddesign_layout.slint:**
```slint
// GridDesign concept: explicit cell positioning
export component GridDesignLayout inherits Window {
    preferred-width: 800px;
    preferred-height: 600px;
    
    // Define grid dimensions
    property <int> columns: 4;
    property <int> rows: 3;
    property <length> cell-width: self.width / columns;
    property <length> cell-height: self.height / rows;
    
    // Layer system
    Rectangle {
        // Background layer (z-index 0)
        background: #f0f0f0;
    }
    
    // Component 1: Position at column 1, row 1, span 2x1
    Rectangle {
        x: 0 * cell-width;
        y: 0 * cell-height;
        width: 2 * cell-width;
        height: 1 * cell-height;
        background: #667eea;
        
        Text {
            text: "Cell (1,1) - Span 2x1";
            color: white;
            font-size: 18px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
    
    // Component 2: Position at column 3, row 1, span 2x2
    Rectangle {
        x: 2 * cell-width;
        y: 0 * cell-height;
        width: 2 * cell-width;
        height: 2 * cell-height;
        background: #06b6d4;
        
        Text {
            text: "Cell (3,1) - Span 2x2";
            color: white;
            font-size: 18px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
    
    // Component 3: Position at column 1, row 2, span 2x2
    Rectangle {
        x: 0 * cell-width;
        y: 1 * cell-height;
        width: 2 * cell-width;
        height: 2 * cell-height;
        background: #10b981;
        
        Text {
            text: "Cell (1,2) - Span 2x2";
            color: white;
            font-size: 18px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
    
    // Component 4: Position at column 3, row 3, span 2x1
    Rectangle {
        x: 2 * cell-width;
        y: 2 * cell-height;
        width: 2 * cell-width;
        height: 1 * cell-height;
        background: #f59e0b;
        
        Text {
            text: "Cell (3,3) - Span 2x1";
            color: white;
            font-size: 18px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}
```

---

## GridDesign Integration {#griddesign-integration}

### JSON Schema → Slint Generation

**src/griddesign.rs:**
```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GridDesignSpec {
    pub id: String,
    pub viewport: Viewport,
    pub grid: GridConfig,
    pub components: Vec<Component>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Viewport {
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GridConfig {
    pub columns: u32,
    pub rows: u32,
    pub gutter_h: u32,
    pub gutter_v: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub positioning: Positioning,
    pub content: Content,
    pub styling: Styling,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Positioning {
    pub start_cell: Cell,
    pub span_cells: Span,
    pub layer: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub column: u32,
    pub row: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Span {
    pub columns: u32,
    pub rows: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Content {
    pub content_type: String,
    pub text: Option<String>,
    pub image_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Styling {
    pub background_color: String,
    pub text_color: Option<String>,
    pub font_size: Option<u32>,
}

impl GridDesignSpec {
    /// Generate Slint code from GridDesign JSON
    pub fn to_slint(&self) -> String {
        let mut slint_code = String::new();
        
        // Header
        slint_code.push_str(&format!(
            "export component {} inherits Window {{\n",
            self.id
        ));
        slint_code.push_str(&format!(
            "    preferred-width: {}px;\n",
            self.viewport.width
        ));
        slint_code.push_str(&format!(
            "    preferred-height: {}px;\n\n",
            self.viewport.height
        ));
        
        // Grid properties
        slint_code.push_str(&format!(
            "    property <int> columns: {};\n",
            self.grid.columns
        ));
        slint_code.push_str(&format!(
            "    property <int> rows: {};\n",
            self.grid.rows
        ));
        slint_code.push_str("    property <length> cell-width: self.width / columns;\n");
        slint_code.push_str("    property <length> cell-height: self.height / rows;\n\n");
        
        // Background
        slint_code.push_str("    Rectangle {\n");
        slint_code.push_str("        background: #f5f5f5;\n");
        slint_code.push_str("    }\n\n");
        
        // Generate components
        for component in &self.components {
            slint_code.push_str(&self.component_to_slint(component));
        }
        
        slint_code.push_str("}\n");
        
        slint_code
    }
    
    fn component_to_slint(&self, component: &Component) -> String {
        let pos = &component.positioning;
        let content = &component.content;
        let style = &component.styling;
        
        format!(
            r#"    // Component: {}
    Rectangle {{
        x: {} * cell-width;
        y: {} * cell-height;
        width: {} * cell-width;
        height: {} * cell-height;
        background: {};
        
        {}
    }}

"#,
            component.id,
            pos.start_cell.column - 1,  // 0-indexed
            pos.start_cell.row - 1,
            pos.span_cells.columns,
            pos.span_cells.rows,
            style.background_color,
            self.content_to_slint(content, style)
        )
    }
    
    fn content_to_slint(&self, content: &Content, style: &Styling) -> String {
        match content.content_type.as_str() {
            "text" => {
                let text = content.text.as_ref().unwrap_or(&String::from(""));
                let color = style.text_color.as_ref().unwrap_or(&String::from("white"));
                let font_size = style.font_size.unwrap_or(16);
                
                format!(
                    r#"Text {{
            text: "{}";
            color: {};
            font-size: {}px;
            horizontal-alignment: center;
            vertical-alignment: center;
        }}"#,
                    text, color, font_size
                )
            }
            "image" => {
                let url = content.image_url.as_ref().unwrap_or(&String::from(""));
                format!(
                    r#"Image {{
            source: @image-url("{}");
            image-fit: contain;
        }}"#,
                    url
                )
            }
            _ => String::from("// Unsupported content type")
        }
    }
}

/// Load GridDesign JSON and generate Slint
pub fn json_to_slint(json_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let json_content = std::fs::read_to_string(json_path)?;
    let spec: GridDesignSpec = serde_json::from_str(&json_content)?;
    Ok(spec.to_slint())
}
```

### Example JSON → Slint Workflow

**griddesign_spec.json:**
```json
{
  "id": "DashboardLayout",
  "viewport": {
    "width": 800,
    "height": 600
  },
  "grid": {
    "columns": 4,
    "rows": 3,
    "gutter_h": 10,
    "gutter_v": 10
  },
  "components": [
    {
      "id": "header",
      "positioning": {
        "start_cell": { "column": 1, "row": 1 },
        "span_cells": { "columns": 4, "rows": 1 },
        "layer": 10
      },
      "content": {
        "content_type": "text",
        "text": "GridDesign Dashboard"
      },
      "styling": {
        "background_color": "#667eea",
        "text_color": "white",
        "font_size": 24
      }
    },
    {
      "id": "sidebar",
      "positioning": {
        "start_cell": { "column": 1, "row": 2 },
        "span_cells": { "columns": 1, "rows": 2 },
        "layer": 10
      },
      "content": {
        "content_type": "text",
        "text": "Navigation"
      },
      "styling": {
        "background_color": "#06b6d4",
        "text_color": "white",
        "font_size": 18
      }
    },
    {
      "id": "content",
      "positioning": {
        "start_cell": { "column": 2, "row": 2 },
        "span_cells": { "columns": 3, "rows": 2 },
        "layer": 10
      },
      "content": {
        "content_type": "text",
        "text": "Main Content Area"
      },
      "styling": {
        "background_color": "#10b981",
        "text_color": "white",
        "font_size": 16
      }
    }
  ]
}
```

**src/main.rs:**
```rust
mod griddesign;

use std::fs;

fn main() {
    // Generate Slint from JSON
    let slint_code = griddesign::json_to_slint("griddesign_spec.json")
        .expect("Failed to generate Slint from JSON");
    
    // Save generated Slint file
    fs::write("ui/generated.slint", slint_code)
        .expect("Failed to write Slint file");
    
    println!("✅ Generated Slint UI from GridDesign JSON");
    println!("Run with: cargo run");
}
```

---

## Cross-Platform Deployment {#deployment}

### Desktop (Windows/macOS/Linux)
```bash
# Build for desktop
cargo build --release

# Binary will be in target/release/
```

### Android
```toml
# Add to Cargo.toml
[lib]
crate-type = ["cdylib"]

[target.'cfg(target_os = "android")'.dependencies]
jni = "0.21"
```

```rust
// Android JNI bridge
#[cfg(target_os = "android")]
#[no_mangle]
pub extern "C" fn Java_com_griddesign_MainActivity_createUI(
    env: JNIEnv,
    _class: JClass,
) {
    // Initialize Slint UI for Android
}
```

### iOS
```bash
# Use cargo-xcode or direct Rust FFI
cargo install cargo-xcode
cargo xcode
```

### WebAssembly
```toml
# Add to Cargo.toml
[dependencies]
slint = { version = "1.3", features = ["renderer-femtovg"] }

[profile.release]
opt-level = 's'
lto = true
```

```bash
# Build for WASM
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/griddesign_slint.wasm --out-dir www
```

---

## Advanced Patterns {#advanced-patterns}

### 1. Responsive Layouts

```slint
export component ResponsiveGrid inherits Window {
    property <bool> is-mobile: self.width < 600px;
    property <int> columns: is-mobile ? 2 : 4;
    
    GridBox {
        spacing: is-mobile ? 5px : 10px;
        
        for item in ["A", "B", "C", "D"]: Rectangle {
            background: lightblue;
            height: is-mobile ? 80px : 120px;
            
            Text {
                text: item;
                font-size: is-mobile ? 14px : 18px;
            }
        }
    }
}
```

### 2. Custom Components

```slint
// Reusable GridCell component
component GridCell inherits Rectangle {
    in property <string> cell-text;
    in property <color> cell-color: blue;
    
    background: cell-color;
    border-radius: 8px;
    
    Text {
        text: cell-text;
        color: white;
        horizontal-alignment: center;
        vertical-alignment: center;
    }
}

export component GridWithCustomCells inherits Window {
    GridBox {
        Row {
            GridCell { cell-text: "Cell 1"; cell-color: red; }
            GridCell { cell-text: "Cell 2"; cell-color: green; }
        }
    }
}
```

### 3. State Management

```rust
use slint::SharedString;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
struct AppState {
    items: Vec<String>,
}

impl AppState {
    fn new() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            items: vec!["Item 1".to_string(), "Item 2".to_string()],
        }))
    }
}

slint::include_modules!();

fn main() {
    let ui = AppWindow::new().unwrap();
    let state = AppState::new();
    
    // Update UI from state
    let state_clone = state.clone();
    ui.on_load_items(move || {
        let items: Vec<SharedString> = state_clone
            .borrow()
            .items
            .iter()
            .map(|s| SharedString::from(s.as_str()))
            .collect();
        
        // Return items to Slint
        slint::ModelRc::new(slint::VecModel::from(items))
    });
    
    ui.run().unwrap();
}
```

### 4. Animations

```slint
export component AnimatedGrid inherits Window {
    in-out property <bool> expanded: false;
    
    Rectangle {
        background: blue;
        
        // Animate width
        width: expanded ? 400px : 100px;
        height: 100px;
        
        animate width {
            duration: 300ms;
            easing: ease-in-out;
        }
        
        TouchArea {
            clicked => { expanded = !expanded; }
        }
    }
}
```

---

## Real-World Examples {#examples}

### Example 1: Interactive Dashboard

**ui/dashboard.slint:**
```slint
import { Button, LineEdit, ListView } from "std-widgets.slint";

export component Dashboard inherits Window {
    title: "GridDesign Dashboard";
    preferred-width: 1000px;
    preferred-height: 700px;
    
    callback add-item(string);
    in-out property <[string]> items: [];
    
    property <int> columns: 5;
    property <int> rows: 4;
    property <length> cell-width: self.width / columns;
    property <length> cell-height: self.height / rows;
    
    // Header
    Rectangle {
        x: 0;
        y: 0;
        width: parent.width;
        height: cell-height;
        background: #667eea;
        
        Text {
            text: "GridDesign Dashboard";
            color: white;
            font-size: 24px;
            font-weight: 700;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
    
    // Sidebar
    Rectangle {
        x: 0;
        y: cell-height;
        width: cell-width;
        height: cell-height * 3;
        background: #06b6d4;
        
        VerticalLayout {
            padding: 20px;
            spacing: 10px;
            
            Text {
                text: "Navigation";
                color: white;
                font-size: 18px;
                font-weight: 600;
            }
            
            Button {
                text: "Home";
            }
            
            Button {
                text: "Settings";
            }
            
            Button {
                text: "About";
            }
        }
    }
    
    // Main Content
    Rectangle {
        x: cell-width;
        y: cell-height;
        width: cell-width * 4;
        height: cell-height * 3;
        background: white;
        
        VerticalLayout {
            padding: 20px;
            spacing: 15px;
            
            Text {
                text: "Content Area";
                font-size: 20px;
                font-weight: 600;
            }
            
            HorizontalLayout {
                spacing: 10px;
                
                LineEdit {
                    placeholder-text: "Add new item...";
                }
                
                Button {
                    text: "Add";
                    clicked => {
                        add-item("New Item");
                    }
                }
            }
            
            ListView {
                for item in items: Text {
                    text: item;
                }
            }
        }
    }
}
```

**src/main.rs:**
```rust
slint::include_modules!();

fn main() {
    let ui = Dashboard::new().unwrap();
    
    // Handle add-item callback
    let ui_weak = ui.as_weak();
    ui.on_add_item(move |item| {
        let ui = ui_weak.unwrap();
        let mut items = ui.get_items();
        let mut vec_items: Vec<_> = items.iter().collect();
        vec_items.push(item);
        ui.set_items(slint::ModelRc::new(slint::VecModel::from(vec_items)));
    });
    
    ui.run().unwrap();
}
```

### Example 2: Responsive Photo Gallery

```slint
export component PhotoGallery inherits Window {
    in property <[image]> photos;
    property <bool> is-mobile: self.width < 600px;
    property <int> columns: is-mobile ? 2 : 4;
    
    GridBox {
        padding: 20px;
        spacing: 10px;
        
        for photo in photos: Rectangle {
            background: #f0f0f0;
            border-radius: 8px;
            
            Image {
                source: photo;
                image-fit: cover;
                width: 100%;
                height: 100%;
            }
            
            TouchArea {
                clicked => {
                    // Handle photo click
                }
            }
        }
    }
}
```

---

## Quick Reference

### Slint Layout Components
- `VerticalBox` / `HorizontalBox` - Flex-like layouts
- `GridBox` - CSS Grid equivalent
- `VerticalLayout` / `HorizontalLayout` - Manual layouts

### Common Properties
- `width`, `height` - Dimensions
- `x`, `y` - Absolute positioning
- `background` - Color or gradient
- `border-radius`, `border-width`, `border-color` - Styling

### Data Types
- `<string>` - Text
- `<int>`, `<float>` - Numbers
- `<bool>` - Boolean
- `<color>` - Colors (#hex or named)
- `<length>` - Dimensions (px, %, etc.)
- `<[T]>` - Arrays

### Callbacks
```slint
callback my-callback(string) -> int;
```

```rust
ui.on_my_callback(|text| {
    println!("Received: {}", text);
    42  // Return value
});
```

---

## Next Steps

1. **Practice**: Start with basic layouts, gradually add complexity
2. **Read Docs**: [slint.dev/docs](https://slint.dev/docs)
3. **Examples**: Check [github.com/slint-ui/slint/tree/master/examples](https://github.com/slint-ui/slint/tree/master/examples)
4. **Build GridDesign**: Implement full JSON → Slint pipeline
5. **Deploy**: Test on multiple platforms

---

## Resources

- **Official Docs**: https://slint.dev/docs
- **GitHub**: https://github.com/slint-ui/slint
- **Examples**: https://slint.dev/examples
- **Discord**: Join Slint community for help

---

**Happy building with Slint + GridDesign! 🚀**
