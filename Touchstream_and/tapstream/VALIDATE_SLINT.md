# Slint File Validation

## Quick Validation (Before Full Build)

### Method 1: Using `slint-viewer` (Recommended)

If you have `slint-viewer` installed:

```bash
# Install slint-viewer (one-time)
cargo install slint-viewer

# Validate the Slint file
slint-viewer ui/tapstream.slint
```

This will:
- ✅ Check syntax errors
- ✅ Show live preview
- ✅ Hot-reload on changes

### Method 2: Minimal Cargo Project

Create a minimal test:

```bash
# Create test directory
mkdir slint_test
cd slint_test

# Create Cargo.toml
cat > Cargo.toml << 'EOF'
[package]
name = "slint_test"
version = "0.1.0"
edition = "2021"

[dependencies]
slint = "1.8"

[build-dependencies]
slint-build = "1.8"
EOF

# Create build.rs
cat > build.rs << 'EOF'
fn main() {
    slint_build::compile("../ui/tapstream.slint").unwrap();
}
EOF

# Create minimal main.rs
cat > src/main.rs << 'EOF'
fn main() {
    println!("Slint file compiled successfully!");
}
EOF

# Test compile (validates Slint)
cargo build
```

### Method 3: Direct Syntax Check

The build process itself validates:

```bash
# Just try to build - Slint errors appear immediately
cargo build 2>&1 | head -20
```

Slint errors appear **before** any Rust compilation, so you'll see them quickly.

---

## Common Slint Syntax Issues

### 1. Property Types

```slint
// ❌ Wrong - input properties are read-only from component
in property <string> name;
TextInput { text: name; }  // Can't modify!

// ✅ Right - use in-out for two-way binding
in-out property <string> name;
TextInput { text <=> name; }
```

### 2. Callbacks

```slint
// ❌ Wrong - edited() takes no arguments
TextInput {
    edited(text) => { ... }
}

// ✅ Right - use self.text to get value
TextInput {
    edited => {
        root.some_callback(self.text);
    }
}
```

### 3. Property Names

```slint
// ❌ Wrong - uses hyphens
TextInput { placeholder-text: "..." }

// ✅ Right - uses underscores
TextInput { placeholder_text: "..." }
```

### 4. Two-Way Binding

```slint
// ❌ Wrong - single arrow is one-way
TextInput { text: root.name; }

// ✅ Right - double arrow is two-way
TextInput { text <=> root.name; }
```

---

## Quick Validation Checklist

Before running `cargo build`:

```bash
# 1. Check file exists
ls ui/tapstream.slint

# 2. Check for common issues
grep "placeholder-text" ui/tapstream.slint  # Should be empty (use _)
grep "edited(.*)" ui/tapstream.slint        # Should be empty (no args)

# 3. Try building just the UI
cargo build 2>&1 | grep -A5 "error:"
```

---

## Fixed Common Issues in TapStream

### Issue 1: LineEdit → TextInput
```slint
// ❌ Wrong
LineEdit { ... }

// ✅ Fixed
TextInput { ... }
```

### Issue 2: edited callback
```slint
// ❌ Wrong
TextInput {
    edited(text) => {
        metadata.dish_name = text;
    }
}

// ✅ Fixed
TextInput {
    text <=> root.metadata.dish_name;
    edited => {
        root.update_metadata(root.metadata);
    }
}
```

### Issue 3: Property declarations
```slint
// ❌ Wrong - can't modify input property
in property <StreamMetadata> metadata;

// ✅ Fixed - in-out allows modification
in-out property <StreamMetadata> metadata;
```

### Issue 4: placeholder
```slint
// ❌ Wrong
TextInput { placeholder-text: "..." }

// ✅ Fixed
TextInput { placeholder_text: "..." }
```

---

## Verify Current File

Check if all fixes are applied:

```bash
cd tapstream

# Should find no occurrences (all fixed)
echo "Checking for LineEdit..."
grep -n "LineEdit" ui/tapstream.slint

echo "Checking for placeholder-text..."
grep -n "placeholder-text" ui/tapstream.slint

echo "Checking for edited(.*) pattern..."
grep -n "edited(..*)" ui/tapstream.slint

# If all return nothing, you're good!
```

---

## Build Now

If validation passes:

```bash
cargo build --release
```

Should compile successfully! ✅
