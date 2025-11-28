# 7 Days from Today - Quick Start

## 🚀 Get Running in 2 Minutes

### Step 1: Prerequisites (1 minute)

```bash
# Check Rust installation
rustc --version

# If not installed:
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

### Step 2: Build & Run (1 minute)

```bash
# Navigate to project
cd seven_days/

# Build (first time takes ~2 minutes)
cargo build --release

# Run immediately
cargo run
```

**That's it!** The app will open showing your next 7 days.

## 📊 What You'll See

```
🗓️  7 Days from Today
   Physics: Intention Space I-O-I-DN-I-GL-I Flow
   Grid: 7 columns = 7 days

✅ App initialized successfully!
   I-O-I-DN-I-GL-I flow active

📊 Loaded 7 days:
   0 - Wednesday (0 todos, 0 notes)
   1 - Thursday (0 todos, 0 notes)
   2 - Friday (0 todos, 0 notes)
   ...
```

## 🎯 Quick Actions

### Add Your First Todo

1. **Click any day card** in the top grid
2. **Type in the input box** (bottom left)
3. **Press Enter** or click ➕
4. ✅ Done! Todo appears immediately

### Write a Note

1. **Click any day card**
2. **Click in the notes area** (bottom right)
3. **Start typing** - auto-saves as you type
4. ✅ Done! Note saved to JSON

### See Previous Weeks

1. **Click any day card**
2. **Click "◀ Previous" button**
3. **See up to 7 previous** occurrences of that weekday
4. ✅ View historical tasks and notes

### Complete a Task

1. **Check the checkbox** next to any todo
2. ✅ Done! Task marked complete (grayed out)
3. **Uncheck to reactivate**
4. **Click 🗑️ to delete**

## 💾 Where's My Data?

Your data is automatically saved to:

**macOS**: `~/Library/Application Support/seven_days/data.json`  
**Linux**: `~/.local/share/seven_days/data.json`  
**Windows**: `%APPDATA%\seven_days\data.json`

## 🎨 Weekday Themes

Each day has a unique color and icon:

- **Monday** 🌸 - Pink
- **Tuesday** 🦋 - Blue  
- **Wednesday** 🌻 - Orange
- **Thursday** 🦜 - Purple
- **Friday** 🌺 - Green
- **Saturday** 🦩 - Hot Pink
- **Sunday** 🌞 - Gold

## 🏗️ Architecture at a Glance

```
User Click (Grid Lookout)
        ↓
    Intention Pulse
        ↓
    Object Mapping
        ↓
    Design Node (Compute)
        ↓
    Data Store (JSON)
        ↓
    Field Pulse (Update)
        ↓
    UI Refresh
```

**ONE CELL = ONE PULSE**

The 7 day cards = 7 Grid Lookouts = 7 computational cells

## 🐛 Troubleshooting

### Build Fails

```bash
# Clean and retry
cargo clean
cargo build --release
```

### App Won't Start

```bash
# Check dependencies
cargo check

# Update Rust
rustup update

# Try debug build
cargo run
```

### Data Not Saving

```bash
# Check permissions
ls -la ~/Library/Application\ Support/seven_days/

# Create directory manually
mkdir -p ~/Library/Application\ Support/seven_days/
```

## 📱 Build Android APK

```bash
# Install Android NDK first
export ANDROID_NDK_HOME=/path/to/ndk

# Build APK
./build_android.sh

# Install
adb install output/SevenDays.apk
```

## 🔧 Development

### Project Files

- `src/main.rs` - App logic, UI integration
- `src/intention_space.rs` - Physics engine  
- `src/data_store.rs` - Data models, JSON storage
- `ui/seven_days.slint` - UI definition
- `Cargo.toml` - Dependencies

### Run Tests

```bash
cargo test
```

### Watch Mode (auto-rebuild)

```bash
cargo install cargo-watch
cargo watch -x run
```

## 🎯 Next Steps

1. **Read README.md** - Full documentation
2. **Customize themes** - Edit `src/data_store.rs`
3. **Extend features** - Add your own Design Nodes
4. **Deploy** - Build for Android/iOS

## 💡 Key Concepts

### Intention Space Physics

Every action follows **I-O-I-DN-I-GL-I**:

- **I** (Intention): User clicks
- **O** (Object): Maps to data operation  
- **I** (Intention): Carries mapped request
- **DN** (Design Node): Computes (add/update/delete)
- **I** (Intention): Carries result
- **GL** (Grid Lookout): Updates display
- **I** (Intention): Ready for next interaction

### Field Pulses

Immutable data carriers:
- `name`: What happened
- `tv`: Trivalent (Y/N/U)
- `response`: Data payload
- `timestamp`: When it happened

### CPUX

Computational Path that executes the I-O-I-DN-I-GL-I flow for each user action.

## ✨ Features You Get

✅ **7-day rolling planner**  
✅ **Persistent storage** (JSON)  
✅ **Beautiful themes** (per weekday)  
✅ **Historical view** (previous weeks)  
✅ **To-do lists** (add, complete, delete)  
✅ **Rich notes** (auto-save)  
✅ **Cross-platform** (macOS, Linux, Windows)  
✅ **Physics-based** (Intention Space)  
✅ **Grid layout** (7 columns = 7 days)  
✅ **Reactive UI** (instant updates)

## 🚀 Ready to Go!

```bash
cargo run
```

**Enjoy your physics-based daily planner!**

---

*Built with Intention Space: I-O-I-DN-I-GL-I Flow*
