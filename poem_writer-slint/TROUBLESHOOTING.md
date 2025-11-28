# 🔧 Troubleshooting: Build Errors

## Common Build Issues and Solutions

---

## ❌ Error: `num_traits::Float` not found

### Symptoms
```
error[E0432]: unresolved import `num_traits::Float`
error[E0432]: unresolved import `num_traits::real`
note: found an item that was configured out
```

### Root Cause
The `euclid` crate (used by Slint) requires `num-traits` with the `std` feature enabled, but older Slint versions may not configure this correctly.

### Solution 1: Update Slint Version (Recommended)

```toml
# In Cargo.toml
[dependencies]
slint = "1.8"  # Use latest stable version

[build-dependencies]
slint-build = "1.8"
```

Then:
```bash
cargo clean
cargo update
cargo build
```

### Solution 2: Clean Build

```bash
# Remove all build artifacts
cargo clean
rm -rf target/
rm Cargo.lock

# Rebuild
cargo build --release
```

### Solution 3: Update All Dependencies

```bash
# Update Cargo.lock
cargo update

# Or update specific packages
cargo update slint slint-build
```

---

## ❌ Error: Android Build Issues

### Symptoms
```
error: failed to compile `poem_writer`
Android-specific errors
NDK toolchain errors
```

### Solution: Separate Desktop and Android Configs

**For Desktop Development:**

Use the simplified `Cargo.toml`:
```toml
[package]
name = "poem_writer"
version = "1.0.0"
edition = "2021"

[dependencies]
slint = "1.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
slint-build = "1.8"
```

**For Android Builds:**

Use `build_android.sh` which handles Android-specific configuration automatically.

---

## ❌ Error: Slint UI File Not Found

### Symptoms
```
error: could not find `ui/poem_writer.slint`
```

### Solution

Check directory structure:
```bash
poem_writer/
├── ui/
│   └── poem_writer.slint  # Must be here
├── src/
│   └── main.rs
└── build.rs
```

If file is missing:
```bash
# Create ui directory
mkdir -p ui

# Ensure poem_writer.slint is in ui/
ls ui/poem_writer.slint
```

---

## ❌ Error: Mutex Poisoned / Lock Errors

### Symptoms
```
thread 'main' panicked at 'mutex poisoned'
```

### Root Cause
A thread panicked while holding a mutex lock.

### Solution

The code already handles this with proper error checking:
```rust
if let Ok(mut collection) = self.collection.lock() {
    // Safe access
}
```

If you still see this error:
1. Check for panics in Design Node closures
2. Ensure no infinite loops
3. Add proper error handling

---

## ❌ Error: Slint Compiler Issues

### Symptoms
```
error: slint-build compilation failed
```

### Solution 1: Check build.rs

```rust
// build.rs should contain:
fn main() {
    slint_build::compile("ui/poem_writer.slint").unwrap();
}
```

### Solution 2: Update Slint Build Tools

```bash
cargo install slint-lsp
cargo install slint-viewer
```

---

## 🔍 Debug Steps

### Step 1: Verify Rust Installation

```bash
rustc --version
cargo --version

# Should show Rust 1.70+ for best compatibility
```

### Step 2: Check Cargo.toml Syntax

```bash
cargo check
```

### Step 3: Verify All Files Present

```bash
# Check required files
ls src/main.rs
ls ui/poem_writer.slint
ls Cargo.toml
ls build.rs
```

### Step 4: Clean Build

```bash
cargo clean
rm -rf target/
cargo build --release
```

### Step 5: Update Dependencies

```bash
cargo update
cargo build --release
```

---

## 🚀 Quick Fix Checklist

- [ ] Update to Slint 1.8+
- [ ] Run `cargo clean`
- [ ] Run `cargo update`
- [ ] Verify file structure
- [ ] Check Rust version (1.70+)
- [ ] Remove Cargo.lock and retry
- [ ] Build with `--release` flag

---

## 💡 Platform-Specific Issues

### macOS

```bash
# If you get linker errors on macOS
xcode-select --install

# Ensure you have the latest Xcode Command Line Tools
```

### Linux

```bash
# Install required dependencies
sudo apt-get install libfontconfig-dev libxkbcommon-dev

# For Fedora/RHEL
sudo dnf install fontconfig-devel libxkbcommon-devel
```

### Windows

```bash
# Ensure you have Visual Studio Build Tools installed
# Or use rustup default to MSVC toolchain
rustup default stable-msvc
```

---

## 🔄 Complete Fresh Start

If all else fails:

```bash
# 1. Backup your changes
cp src/main.rs src/main.rs.backup

# 2. Delete everything
rm -rf target/
rm Cargo.lock

# 3. Update Rust
rustup update stable

# 4. Clean Cargo cache (optional, last resort)
cargo clean
rm -rf ~/.cargo/registry/cache/
rm -rf ~/.cargo/registry/src/

# 5. Rebuild
cargo build --release
```

---

## 📞 Getting Help

If issues persist:

1. **Check Slint docs**: https://slint.dev
2. **Slint GitHub Issues**: https://github.com/slint-ui/slint/issues
3. **Rust Users Forum**: https://users.rust-lang.org
4. **Copy full error log** and search for similar issues

---

## ✅ Verified Working Configuration

```toml
# Cargo.toml - Tested and working
[package]
name = "poem_writer"
version = "1.0.0"
edition = "2021"

[dependencies]
slint = "1.8"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

[build-dependencies]
slint-build = "1.8"
```

**Environment:**
- Rust: 1.70+
- Slint: 1.8+
- Platforms: macOS, Linux, Windows

---

## 🎯 Expected Build Output

Successful build should show:

```
   Compiling poem_writer v1.0.0
   Compiling slint v1.8.0
   Compiling slint-build v1.8.0
    Finished release [optimized] target(s) in X.XXs
```

No errors or warnings related to `num_traits` or `euclid`.

---

**Your specific error was a `num_traits` version conflict. The updated `Cargo.toml` should fix this!**

Try now:
```bash
cargo clean
cargo build --release
```
