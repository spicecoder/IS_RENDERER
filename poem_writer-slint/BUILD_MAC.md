# 🍎 Building Poem Writer on macOS

## Quick Start (5 minutes)

---

## Step 1: Install Rust (if not already installed)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts, then:
source $HOME/.cargo/env

# Verify
rustc --version
cargo --version
```

---

## Step 2: Extract Project

```bash
# Navigate to your Downloads folder (or wherever you downloaded)
cd ~/Downloads

# Extract
tar -xzf poem_writer.tar.gz
# Or if you downloaded .zip:
# unzip poem_writer.zip

# Enter directory
cd poem_writer
```

---

## Step 3: Build the Executable

```bash
# Clean build with correct dependencies
cargo clean
cargo build --release
```

**Build time:** ~2-3 minutes (first time)

**Output:**
```
   Compiling poem_writer v1.0.0
   Compiling slint v1.8.0
    Finished release [optimized] target(s) in 2m 30s
```

---

## Step 4: Find Your Executable

The executable will be at:

```bash
target/release/poem_writer
```

**Full path:**
```
~/Downloads/poem_writer/target/release/poem_writer
```

---

## Step 5: Run It!

```bash
# Option 1: From project directory
cargo run --release

# Option 2: Run executable directly
./target/release/poem_writer

# Option 3: Full path
~/Downloads/poem_writer/target/release/poem_writer
```

---

## 🎯 Directory Structure After Build

```
poem_writer/
├── src/
│   └── main.rs
├── ui/
│   └── poem_writer.slint
├── Cargo.toml
├── build.rs
└── target/              ← Created during build
    ├── debug/
    └── release/         ← Your executable is here
        └── poem_writer  ← THE EXECUTABLE! 🎉
```

---

## 📦 Making It Portable

### Option 1: Copy Executable Anywhere

```bash
# Copy to your Applications folder
cp target/release/poem_writer ~/Applications/

# Run from anywhere
~/Applications/poem_writer
```

### Option 2: Add to PATH

```bash
# Copy to a directory in your PATH
sudo cp target/release/poem_writer /usr/local/bin/

# Now run from anywhere
poem_writer
```

### Option 3: Create macOS App Bundle (Advanced)

```bash
# Create app structure
mkdir -p PoemWriter.app/Contents/MacOS

# Copy executable
cp target/release/poem_writer PoemWriter.app/Contents/MacOS/

# Create Info.plist
cat > PoemWriter.app/Contents/Info.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>poem_writer</string>
    <key>CFBundleIdentifier</key>
    <string>com.intentionspace.poemwriter</string>
    <key>CFBundleName</key>
    <string>Poem Writer</string>
    <key>CFBundleVersion</key>
    <string>1.0.0</string>
    <key>CFBundleShortVersionString</key>
    <string>1.0.0</string>
</dict>
</plist>
EOF

# Now you have PoemWriter.app - drag to Applications folder!
```

---

## 🐛 Troubleshooting macOS

### Error: "Developer cannot be verified"

```bash
# Remove quarantine attribute
xattr -dr com.apple.quarantine target/release/poem_writer
```

### Error: Linker issues

```bash
# Install Xcode Command Line Tools
xcode-select --install
```

### Error: Missing dependencies

```bash
# Usually not needed, but if you get fontconfig errors:
brew install fontconfig
```

---

## 🚀 Quick Commands

```bash
# Build and run in one command
cargo run --release

# Just build (don't run)
cargo build --release

# Clean and rebuild
cargo clean && cargo build --release

# Check for errors without building
cargo check
```

---

## 📏 File Sizes

**Debug build:** ~50 MB (slow, with debug symbols)  
**Release build:** ~8-12 MB (fast, optimized)

Always use `--release` for the executable you'll actually use!

---

## ✨ What You'll See

When you run the app:

```
┌─────────────────────────────────────────┐
│         Poem Writer                     │
│                                         │
│  ┌─────────────┬─────────────┐         │
│  │             │             │         │
│  │   Poem      │   Notes     │         │
│  │   Text      │   & Ideas   │         │
│  │             │             │         │
│  └─────────────┴─────────────┘         │
│                                         │
│  ← | Page 1 of 1 | + →                │
└─────────────────────────────────────────┘
```

---

## 🎯 Expected Build Output

Successful build shows:

```
   Compiling slint-build v1.8.0
   Compiling slint v1.8.0
   Compiling serde v1.0.214
   Compiling poem_writer v1.0.0 (~/Downloads/poem_writer)
    Finished `release` profile [optimized] target(s) in 2m 15s
```

Executable created at: `target/release/poem_writer`

---

## 📋 Complete Build Process

```bash
# 1. Download and extract
cd ~/Downloads
tar -xzf poem_writer.tar.gz
cd poem_writer

# 2. Build
cargo build --release

# 3. Run
./target/release/poem_writer

# That's it! 🎉
```

---

## 💡 Pro Tips

**Speed up rebuild after code changes:**
```bash
# Development build (faster compile, slower runtime)
cargo build
./target/debug/poem_writer

# Production build (slower compile, faster runtime)
cargo build --release
./target/release/poem_writer
```

**Watch mode (auto-rebuild on file changes):**
```bash
cargo install cargo-watch
cargo watch -x run
```

---

## 🔍 Finding Your Executable

If you forget where it is:

```bash
# Find all poem_writer executables
find . -name "poem_writer" -type f

# Should show:
# ./target/release/poem_writer
# ./target/debug/poem_writer (if you built debug too)
```

---

## ✅ Verification

Your executable is ready when:

```bash
ls -lh target/release/poem_writer
# Shows: -rwxr-xr-x ... poem_writer (8-12 MB)

file target/release/poem_writer
# Shows: Mach-O 64-bit executable arm64 (or x86_64)

./target/release/poem_writer --version
# Runs without errors
```

---

**Your executable will be at:** `target/release/poem_writer`

**You need to build it first with:** `cargo build --release`

Happy coding! 🎨✍️
