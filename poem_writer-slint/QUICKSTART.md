# 🚀 Poem Writer - Quick Start

## Get Running in 5 Minutes

### Step 1: Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# For Android: Install NDK
# Download from: https://developer.android.com/ndk/downloads
export ANDROID_NDK_ROOT=/path/to/your/ndk
```

### Step 2: Build Desktop Version (Testing)

```bash
cd poem_writer

# Build
cargo build --release

# Run
cargo run
```

**Expected output:**
```
🎨 Poem Writer - Intention Space
[App window opens with split-screen layout]
```

### Step 3: Test the App

1. **Type in left side** → See poem text appear
2. **Type in right side** → See notes appear
3. **Click `+` button** → Creates page 2
4. **Click `◀`** → Returns to page 1 (content preserved)
5. **Click `▶`** → Goes to page 2

✅ If all works, you're ready for Android!

### Step 4: Build Android APK

```bash
# Install Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Build APK
chmod +x build_android.sh
./build_android.sh
```

**Expected output:**
```
✅ APK built successfully!

📦 Output:
   File: output/PoemWriter.apk
   Size: ~4 MB
```

### Step 5: Install on Device

```bash
# Via ADB
adb install output/PoemWriter.apk

# Or transfer APK and install manually
```

## 🎯 Verify Intention Space Physics

### Check Data Flow

While app is running, add debug output:

```rust
// In src/main.rs, add to update_poem_text():
println!("🔄 I-O-I-DN-I-GL-I Flow:");
println!("  1. Intention: user_input_poem");
println!("  2. Object: validate_poem_text");
println!("  3. Design Node: update_poem");
println!("  4. State: poem_text updated");
```

### Inspect Network Field

```rust
// Add to main():
let field = cpux.network_field.lock().unwrap();
println!("📡 Network Field: {} pulses", field.len());
for pulse in field.iter().take(5) {
    println!("  - {}: {:?}", pulse.name, pulse.tv);
}
```

## 🐛 Troubleshooting

### Build Fails - Missing Slint

```bash
# Update Cargo.lock
cargo update

# Clean and rebuild
cargo clean
cargo build
```

### Android Build Fails - NDK Not Found

```bash
# Verify NDK path
echo $ANDROID_NDK_ROOT
ls $ANDROID_NDK_ROOT

# If not set:
export ANDROID_NDK_ROOT=/path/to/ndk
# Add to ~/.bashrc or ~/.zshrc
```

### APK Won't Install

```bash
# Check device compatibility
adb shell getprop ro.product.cpu.abi

# Should show: arm64-v8a or armeabi-v7a

# Uninstall old version
adb uninstall com.intentionspace.poemwriter

# Reinstall
adb install output/PoemWriter.apk
```

### App Crashes on Launch

```bash
# Check logs
adb logcat | grep poem_writer

# Common issues:
# 1. Missing NDK libraries → Rebuild with correct NDK
# 2. Wrong target → Build for correct architecture
# 3. Permission issues → Check AndroidManifest.xml
```

## 📱 Using the App

### Writing Your First Poem

1. **Launch app**
2. **Left side:** Type your poem
   ```
   In silicon dreams we code,
   Intentions flow like rivers,
   Pulses dance through nodes,
   Computing delivers.
   ```
3. **Right side:** Add notes
   ```
   Theme: Technology poetry
   Emotion: Wonder, excitement
   Imagery: Rivers, dancing, flow
   Rhyme: ABCB pattern
   ```
4. **Tap `+`** → Start page 2

### Managing Multiple Poems

- Each page = one complete poem + notes
- Navigate freely between pages
- All content auto-saved
- Page counter shows position

### Best Practices

✅ **DO:**
- Write freely, edit later
- Use notes for brainstorming
- Create new page for each poem
- Navigate to review previous work

❌ **DON'T:**
- Worry about saving (it's automatic)
- Delete pages (feature coming soon)
- Type in both sections simultaneously
- Exit without testing navigation

## 🎓 Understanding the Flow

### When You Type in Poem Editor

```
Your keystroke
    ↓
[Intention 1] user_input_poem (TV: U)
    ↓
[Object] validate_poem_text
    ↓
[Intention 2] validated_poem (TV: Y)
    ↓
[Design Node] update_poem (COMPUTE)
    ↓
[Intention 3] update_poem_result (TV: Y)
    ↓
[Network Field] stores pulse
    ↓
[Grid Lookout] UI updates
    ↓
You see text appear
```

### When You Add a Page

```
You tap '+' button
    ↓
[Intention] create_new_page (TV: U)
    ↓
[Design Node] create_page (COMPUTE)
    ↓
PoemCollection adds PoemPage
    ↓
[Intention] new_page_created (TV: Y)
    ↓
UI updates to show page 2
    ↓
Page counter: "Page 2 / 2"
```

## ✨ Next Steps

### Explore the Code

1. **Intention Space Core:**
   ```bash
   # Open in editor
   code src/main.rs
   
   # Look for:
   # - FieldPulse struct
   # - DesignNode struct  
   # - ObjectMapping struct
   # - PoemWriterCPUX
   ```

2. **UI Definition:**
   ```bash
   # Open Slint file
   code ui/poem_writer.slint
   
   # Examine:
   # - Split-screen layout
   # - Grid Lookout bindings
   # - Component structure
   ```

3. **Build System:**
   ```bash
   # Review build script
   code build_android.sh
   
   # Understand:
   # - Target compilation
   # - APK generation
   # - Library copying
   ```

### Customize

**Change Colors:**
```slint
// In ui/poem_writer.slint
global Colors := {
    property <color> background: #YOUR_COLOR;
    property <color> primary: #YOUR_ACCENT;
}
```

**Modify Layout:**
```slint
// Change split ratio
horizontal-stretch: 3;  // Poem side larger
horizontal-stretch: 1;  // Notes side smaller
```

**Add Features:**
```rust
// In src/main.rs
// Add new Design Node
DesignNode::new("your_feature", "Feature Name", |inputs| {
    // Your computation here
    vec!["result".to_string()]
})
```

### Deploy

**For Testing:**
```bash
# Install on your device
adb install output/PoemWriter.apk
```

**For Production:**
1. Sign APK with release key
2. Optimize with ProGuard
3. Test on multiple devices
4. Submit to Play Store

### Learn More

📚 **Read:**
- `README.md` - Full documentation
- `ARCHITECTURE.md` - Design details
- `Intention_Space_and_PnR_Computing.docx` - Theory

🔬 **Experiment:**
- Add word count feature
- Implement search
- Create export to PDF
- Add themes/formatting

🤝 **Contribute:**
- Report bugs
- Suggest features
- Submit pull requests
- Share improvements

## 🎯 Success Checklist

- [ ] Desktop version runs
- [ ] Can type in both sections
- [ ] Can create new pages
- [ ] Can navigate between pages
- [ ] Content persists when switching
- [ ] Android APK builds
- [ ] APK installs on device
- [ ] App launches on Android
- [ ] Touch input works
- [ ] All features functional

**If all checked: You're ready to write poems! ✍️**

---

## 📞 Need Help?

### Common Issues

**Q: "Cargo not found"**  
A: Run `source $HOME/.cargo/env` or restart terminal

**Q: "NDK error"**  
A: Set `export ANDROID_NDK_ROOT=/path/to/ndk`

**Q: "Slint error"**  
A: Run `cargo clean && cargo update && cargo build`

**Q: "App crashes"**  
A: Check `adb logcat` for errors

### Resources

- [Rust Installation](https://rustup.rs)
- [Slint Docs](https://slint.dev)
- [Android NDK](https://developer.android.com/ndk)
- [Intention Space Theory](./Intention_Space_and_PnR_Computing.docx)

---

**You're now ready to create beautiful poetry with Intention Space physics!** 🎨

*The flow continues: I-O-I-DN-I-GL-I ...*
