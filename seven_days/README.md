# 7 Days from Today

A beautiful, physics-based daily planning app built with **Rust + Slint** implementing **Intention Space** computational model.

## ✨ Features

- **7-Day Rolling View**: See the next 7 days at a glance
- **Smart Organization**: 
  - To-do lists for each day
  - Rich text notes
  - Auto-save functionality
- **Historical View**: Click back to see previous occurrences of any weekday (up to 7 weeks)
- **Beautiful Themes**: Each weekday has its own color scheme and icon
- **Persistent Storage**: All data saved in JSON format
- **Cross-Platform**: Works on macOS, Linux, Windows, Android, iOS

## 🏗️ Architecture

### Intention Space Physics

The app implements the **I-O-I-DN-I-GL-I** flow pattern:

```
Intention → Object → Intention → Design Node → Intention → Grid Lookout → Intention
    ↓          ↓          ↓            ↓             ↓            ↓            ↓
  Signal    Mapping   Transfer    COMPUTE      Transfer    Display     Interact
```

### Grid Layout

```
ONE CELL = ONE PULSE

┌─────────┬─────────┬─────────┬─────────┬─────────┬─────────┬─────────┐
│  MON    │  TUE    │  WED    │  THU    │  FRI    │  SAT    │  SUN    │
│  🌸     │  🦋     │  🌻     │  🦜     │  🌺     │  🦩     │  🌞     │
│  Pink   │  Blue   │ Orange  │ Purple  │  Green  │HotPink  │  Gold   │
└─────────┴─────────┴─────────┴─────────┴─────────┴─────────┴─────────┘
```

### Component Flow

```
User Interaction (GL)
        ↓
    Intention
        ↓
    Object (mapping)
        ↓
    Design Node (compute: add/toggle/update)
        ↓
    Data Store (persist)
        ↓
    Field Pulse (state change)
        ↓
    Grid Lookout (UI update)
```

## 🚀 Quick Start

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version
cargo --version
```

### Build & Run

```bash
# Clone or extract project
cd seven_days/

# Build
cargo build --release

# Run
cargo run
```

You should see:
```
🗓️  7 Days from Today
   Physics: Intention Space I-O-I-DN-I-GL-I Flow
   Grid: 7 columns = 7 days

✅ App initialized successfully!
   I-O-I-DN-I-GL-I flow active

📊 Loaded 7 days:
   0 - Monday (0 todos, 0 notes)
   1 - Tuesday (0 todos, 0 notes)
   ...
```

## 📱 Usage

### Main View

1. **7-Day Grid**: Top section shows cards for each of the next 7 days
   - Click any day to view/edit
   - Each day has unique color and icon

2. **Detail View**: Bottom section shows selected day's content
   - **Left**: To-do list
     - Add tasks with the input box
     - Check off completed items
     - Delete with trash icon
   - **Right**: Notes section
     - Rich text editing
     - Auto-saves as you type

### Previous Weekdays

1. Click "◀ Previous" button while viewing any day
2. Shows up to 7 previous occurrences of that weekday
3. See historical tasks and notes
4. Click "✕ Close" to return

### Navigation

- **Next Week ▶**: Advance to next 7-day period
- Day cards show:
  - Weekday icon (🌸 🦋 🌻 🦜 🌺 🦩 🌞)
  - Weekday name
  - Actual date

## 🎨 Customization

### Weekday Themes

Each weekday has a default theme (in `src/data_store.rs`):

```rust
pub fn get_weekday_color(weekday: Weekday) -> (&'static str, &'static str) {
    match weekday {
        Weekday::Mon => ("#FFE5E5", "#FF6B6B"), // Pink
        Weekday::Tue => ("#E5F3FF", "#4A90E2"), // Blue
        Weekday::Wed => ("#FFF5E5", "#FFA500"), // Orange
        Weekday::Thu => ("#F0E5FF", "#9B59B6"), // Purple
        Weekday::Fri => ("#E5FFF0", "#2ECC71"), // Green
        Weekday::Sat => ("#FFE5F5", "#FF69B4"), // Hot Pink
        Weekday::Sun => ("#FFFAE5", "#FFD700"), // Gold
    }
}
```

### Weekday Icons

```rust
pub fn get_weekday_icon(weekday: Weekday) -> &'static str {
    match weekday {
        Weekday::Mon => "🌸", // Monday - Flower
        Weekday::Tue => "🦋", // Tuesday - Butterfly
        Weekday::Wed => "🌻", // Wednesday - Sunflower
        Weekday::Thu => "🦜", // Thursday - Parrot
        Weekday::Fri => "🌺", // Friday - Hibiscus
        Weekday::Sat => "🦩", // Saturday - Flamingo
        Weekday::Sun => "🌞", // Sunday - Sun
    }
}
```

To customize:
1. Edit `src/data_store.rs`
2. Change colors (hex format)
3. Change icons (any emoji or unicode)
4. Rebuild: `cargo build --release`

### Custom User Icons

The data model supports custom images:

```rust
pub struct DayTheme {
    pub background_color: String,
    pub accent_color: String,
    pub icon_type: IconType, // Flower, Bird, Animal, Custom
    pub icon_data: String,   // Path to image or emoji
}
```

Future enhancement: UI to upload custom images per day.

## 💾 Data Storage

### Location

Data is stored in:
- **macOS**: `~/Library/Application Support/seven_days/data.json`
- **Linux**: `~/.local/share/seven_days/data.json`
- **Windows**: `%APPDATA%\seven_days\data.json`

### Format

```json
{
  "entries": {
    "2025-11-26": {
      "date": "2025-11-26",
      "weekday": "Wednesday",
      "todos": [
        {
          "id": "550e8400-e29b-41d4-a716-446655440000",
          "text": "Review project proposal",
          "completed": false,
          "created_at": "2025-11-26T10:00:00Z"
        }
      ],
      "notes": [
        {
          "id": "550e8400-e29b-41d4-a716-446655440001",
          "content": "Meeting notes...",
          "created_at": "2025-11-26T10:00:00Z",
          "updated_at": "2025-11-26T11:30:00Z"
        }
      ],
      "theme": {
        "background_color": "#FFF5E5",
        "accent_color": "#FFA500",
        "icon_type": "Flower",
        "icon_data": "🌻"
      }
    }
  }
}
```

### Backup

```bash
# Backup data
cp ~/.local/share/seven_days/data.json ~/backup/seven_days_backup.json

# Restore data
cp ~/backup/seven_days_backup.json ~/.local/share/seven_days/data.json
```

## 🔧 Development

### Project Structure

```
seven_days/
├── src/
│   ├── main.rs              # App entry, UI integration
│   ├── intention_space.rs   # Physics engine
│   └── data_store.rs        # Data models, storage
├── ui/
│   └── seven_days.slint     # UI definition
├── Cargo.toml               # Dependencies
└── build.rs                 # Build script
```

### Key Components

1. **intention_space.rs**: Physics engine
   - `FieldPulse`: Immutable response carriers
   - `DesignNode`: Computation units
   - `IntentionObject`: Static mappers
   - `GridLookout`: Display layer
   - `CPUX`: Computational path
   - `SpaceLoop`: Orchestrator

2. **data_store.rs**: Data layer
   - `TodoItem`, `Note`: Basic data types
   - `DayEntry`: Single day's data
   - `SevenDaysStore`: Main store
   - Persistence helpers

3. **main.rs**: Integration
   - UI callbacks
   - State management
   - Data <-> UI conversion

4. **seven_days.slint**: UI
   - Grid layout (7 columns)
   - Day cards
   - Detail view
   - Components (TodoItem, TextEdit)

### Building for Production

```bash
# Optimized build
cargo build --release

# Executable in:
# target/release/seven_days
```

### Running Tests

```bash
cargo test
```

## 📊 Intention Space Monitoring

### Network Field State

The app uses Intention Space physics with field pulses:

```rust
// Check field state (from code)
let field = app_state.space_loop.lock().unwrap().network_field.lock().unwrap();
println!("Field pulses: {}", field.pulses.len());

// Pulses include:
// - "day_selected" (user clicks day)
// - "load_day" (day data loaded)
// - "update_todo" (todo changed)
// - "update_note" (note edited)
```

### CPUX Execution

Each user action triggers I-O-I-DN-I-GL-I flow:

1. **User clicks day card** (GL interaction)
2. **Intention "day_selected"** created
3. **Object maps** intention to data request
4. **Design Node computes** day data
5. **Intention carries** result
6. **Grid Lookout updates** UI
7. **User sees** new state

## 🐛 Troubleshooting

### Build Errors

```bash
# Clean build
cargo clean
cargo build --release

# Update dependencies
cargo update
```

### Data Not Saving

Check permissions:
```bash
# macOS/Linux
ls -la ~/Library/Application\ Support/seven_days/
chmod 755 ~/Library/Application\ Support/seven_days/

# Verify write access
touch ~/Library/Application\ Support/seven_days/test.txt
```

### UI Not Updating

The app uses reactive Slint models. If UI doesn't update:
1. Check that callbacks are connected
2. Verify `load_seven_days()` is called after data changes
3. Restart the app

## 🚀 Future Enhancements

- [ ] Custom theme editor
- [ ] Image upload for day icons
- [ ] Export to PDF/HTML
- [ ] Calendar view
- [ ] Recurring tasks
- [ ] Task priorities
- [ ] Rich text formatting in notes
- [ ] Search across all days
- [ ] Mobile apps (Android/iOS)
- [ ] Cloud sync
- [ ] Collaboration features

## 📝 License

MIT License - see LICENSE file

## 🙏 Acknowledgments

- **Slint**: Modern UI framework
- **Rust**: Memory safety and performance
- **Intention Space**: Novel computational model
- **Chrono**: Date/time handling

---

**Built with Intention Space physics: I-O-I-DN-I-GL-I**

*The flow continues: User → Intention → Computation → Display → User...*
