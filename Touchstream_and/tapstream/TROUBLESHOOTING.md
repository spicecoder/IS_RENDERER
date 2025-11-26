# TapStream Build Troubleshooting Guide

## Common Build Errors and Solutions

### 1. ❌ "Unknown element 'LineEdit'" (FIXED)

**Error:**
```
error: Unknown element 'LineEdit'
 --> ui/tapstream.slint:191:17
```

**Cause:** Slint uses `TextInput` not `LineEdit`

**Solution:** 
✅ This is already fixed in the latest version!
- Download the updated `tapstream.tar.gz` 
- Or manually replace all `LineEdit` with `TextInput` in `ui/tapstream.slint`

---

### 2. ❌ "failed to run custom build command"

**Error:**
```
error: failed to run custom build command for `tapstream v1.0.0`
```

**Common Causes:**

#### A. Missing Slint Dependencies

**macOS:**
```bash
# Install required system libraries
brew install cmake ninja

# For older macOS, may need:
xcode-select --install
```

**Linux (Ubuntu/Debian):**
```bash
sudo apt-get update
sudo apt-get install cmake libfontconfig1-dev libxcb-render0-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev \
    libssl-dev ninja-build
```

**Linux (Fedora):**
```bash
sudo dnf install cmake fontconfig-devel libxcb-devel \
    libxkbcommon-devel openssl-devel ninja-build
```

#### B. Outdated Rust Version

```bash
# Check Rust version
rustc --version

# Should be 1.70 or higher
# If not, update:
rustup update stable
```

#### C. Corrupted Build Cache

```bash
# Clean and rebuild
cargo clean
cargo build --release
```

---

### 3. ❌ "could not find `Cargo.toml`"

**Error:**
```
error: could not find `Cargo.toml` in current directory
```

**Solution:**
```bash
# Make sure you're in the tapstream directory
cd tapstream

# Verify files exist
ls Cargo.toml
ls src/
ls ui/

# If missing, re-extract
cd ..
tar -xzf tapstream.tar.gz
cd tapstream
```

---

### 4. ❌ Slint version mismatch

**Error:**
```
error: slint version mismatch
```

**Solution:**
```bash
# Update Cargo.toml to use latest Slint
# Change line:
slint = "1.14"

# Then update dependencies
cargo update
cargo build --release
```

---

### 5. ❌ "linker `cc` not found" (macOS)

**Error:**
```
error: linker `cc` not found
```

**Solution:**
```bash
# Install Xcode command line tools
xcode-select --install

# Verify installation
cc --version
```

---

### 6. ❌ Permission denied (build script)

**Error:**
```
permission denied: ./build_android.sh
```

**Solution:**
```bash
# Make script executable
chmod +x build_android.sh

# Then run
./build_android.sh
```

---

### 7. ❌ Memory allocation errors during build

**Error:**
```
error: memory allocation failed
```

**Solution:**
```bash
# Build in debug mode (uses less memory)
cargo build

# Or increase system swap space
# Or close other applications

# For incremental builds (faster, less memory)
cargo build --release -j 2  # Use 2 cores instead of all
```

---

### 8. ❌ "no such file or directory: ui/tapstream.slint"

**Error:**
```
error: failed to read file: ui/tapstream.slint
```

**Solution:**
```bash
# Check directory structure
ls -la ui/

# Should see tapstream.slint
# If missing, re-extract archive

# Verify you're in right directory
pwd  # Should end with /tapstream
```

---

### 9. ❌ Android NDK not found (when building APK)

**Error:**
```
error: ANDROID_NDK_HOME not set
```

**Solution:**
```bash
# Download Android NDK from:
# https://developer.android.com/ndk/downloads

# Set environment variable (replace path with your NDK location)
export ANDROID_NDK_HOME=/path/to/android-ndk-r26b

# Add to shell profile for persistence (~/.bashrc or ~/.zshrc)
echo 'export ANDROID_NDK_HOME=/path/to/android-ndk-r26b' >> ~/.bashrc
```

---

### 10. ❌ Rust Android targets not installed

**Error:**
```
error: can't find target aarch64-linux-android
```

**Solution:**
```bash
# Install Android targets
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android

# Verify installation
rustup target list --installed
```

---

## 🔍 Debug Mode Build

If you're stuck, try building in debug mode for more information:

```bash
# Clean build
cargo clean

# Build with verbose output
cargo build --verbose

# Or with even more details
RUST_BACKTRACE=1 cargo build --verbose
```

---

## ✅ Verify Installation

Before building, verify your environment:

```bash
# Check Rust
rustc --version
cargo --version

# Check required tools (macOS)
cmake --version
ninja --version

# Check directory structure
ls -la
# Should see: Cargo.toml, build.rs, src/, ui/

# Check file contents
head -5 Cargo.toml
# Should start with [package]
```

---

## 🆘 Still Having Issues?

If you're still stuck:

1. **Check the error message carefully** - it usually tells you what's missing

2. **Try the minimal build:**
   ```bash
   cargo build  # Without --release
   ```

3. **Check system requirements:**
   - Rust 1.70+
   - 4GB RAM minimum
   - 2GB free disk space
   - CMake, Ninja (for Slint)

4. **Platform-specific issues:**
   - **macOS:** Install Xcode command line tools
   - **Linux:** Install development libraries (fontconfig, xcb, etc.)
   - **Windows:** Install Visual Studio Build Tools

5. **Report the issue:**
   - Include full error message
   - Include `rustc --version` output
   - Include OS and version
   - Include contents of `Cargo.toml`

---

## 📝 Quick Recovery Steps

If completely stuck, start fresh:

```bash
# 1. Remove everything
cd ..
rm -rf tapstream

# 2. Re-extract
tar -xzf tapstream.tar.gz
cd tapstream

# 3. Verify files
ls src/*.rs  # Should see 4 files
ls ui/*.slint  # Should see tapstream.slint

# 4. Clean build
cargo build --release

# Should work now!
```

---

## 🎯 Most Common Issues Summary

| Issue | Solution |
|-------|----------|
| LineEdit error | ✅ Fixed in latest version |
| Missing dependencies | Install CMake, Ninja, system libs |
| Wrong directory | `cd tapstream` first |
| Outdated Rust | `rustup update stable` |
| Android NDK | Set `ANDROID_NDK_HOME` |
| Permissions | `chmod +x build_android.sh` |

---

**After fixing, try again:**

```bash
cargo build --release
cargo run
```

**Should see:**
```
🚀 TapStream - Intention Space Live Streaming
   Physics: I-O-I-DN-I-GL-I Flow
   Grid: ONE CELL = ONE PULSE

[App window opens]
```

✅ Success!
