# TapStream Quick Start Guide

## 🚀 Get Running in 5 Minutes

### Step 0: Set Up Project Directory

**IMPORTANT: First, extract/copy all source files!**

#### Option A: From Compressed Archive
```bash
# Extract the archive
tar -xzf tapstream.tar.gz

# Verify files are there
cd tapstream
ls -la src/        # Should see: intention_space.rs, tapstream.rs, server.rs, main.rs
ls -la ui/         # Should see: tapstream.slint
ls -la             # Should see: Cargo.toml, build.rs, build_android.sh

# Now you're ready to build!
```

#### Option B: From Downloaded Directory
```bash
# If you downloaded individual files, ensure this structure:
tapstream/
├── src/
│   ├── intention_space.rs
│   ├── tapstream.rs
│   ├── server.rs
│   └── main.rs
├── ui/
│   └── tapstream.slint
├── Cargo.toml
├── build.rs
└── build_android.sh

# Navigate to project
cd tapstream
```

#### Option C: Manual Setup
```bash
# Create project structure
mkdir -p tapstream/src
mkdir -p tapstream/ui

# Copy/download all files to correct locations:
# - All Rust files (.rs) → tapstream/src/
# - Slint UI file (.slint) → tapstream/ui/
# - Build files (Cargo.toml, build.rs, build_android.sh) → tapstream/

cd tapstream
```

### Step 1: Prerequisites

```bash
# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify installation
rustc --version
cargo --version

# For Android builds: Install Android NDK
# Download from: https://developer.android.com/ndk/downloads
export ANDROID_NDK_HOME=/path/to/ndk
```

### Step 2: Build Desktop Version (Testing)

```bash
# Make sure you're in the tapstream directory with all source files
cd tapstream

# Build
cargo build --release

# Run
cargo run
```

You should see:
```
🚀 TapStream - Intention Space Live Streaming
   Physics: I-O-I-DN-I-GL-I Flow
   Grid: ONE CELL = ONE PULSE

[App window opens with role selector]
```

**If build fails**, verify all source files are present:
```bash
# Check required files
ls src/intention_space.rs src/tapstream.rs src/server.rs src/main.rs
ls ui/tapstream.slint
ls Cargo.toml build.rs

# All files should exist. If not, extract/copy them again.
```

### Step 3: Build Android APK

```bash
# Install Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi

# Make build script executable
chmod +x build_android.sh

# Build APK
./build_android.sh
```

Output:
```
🚀 TapStream - Android APK Build
✅ APK built successfully!
   Output: output/TapStream.apk
   Size: ~5 MB
```

### Install on Device

```bash
# Via ADB
adb install output/TapStream.apk

# Or transfer APK to device and install manually
```

## 📁 Verify Project Setup

Before building, ensure you have all required files:

```bash
# In the tapstream directory, run:
find . -type f -name "*.rs" -o -name "*.slint" -o -name "*.toml" -o -name "*.sh"

# You should see:
# ./src/intention_space.rs
# ./src/tapstream.rs
# ./src/server.rs
# ./src/main.rs
# ./ui/tapstream.slint
# ./Cargo.toml
# ./build.rs
# ./build_android.sh
```

**Required Files Checklist:**
- [ ] src/intention_space.rs (13 KB) - Physics engine
- [ ] src/tapstream.rs (17 KB) - App logic
- [ ] src/server.rs (11 KB) - HTTP server
- [ ] src/main.rs (15 KB) - Slint integration
- [ ] ui/tapstream.slint (16 KB) - UI definition
- [ ] Cargo.toml (1.4 KB) - Dependencies
- [ ] build.rs (109 B) - Build script
- [ ] build_android.sh (7.9 KB) - APK builder

**If any files are missing:**
1. Re-extract tapstream.tar.gz, OR
2. Download missing files from the tapstream/ directory, OR
3. Check the FILE_MANIFEST.txt for correct file locations

## 📱 Usage

### As Cook (Stream Producer)

1. **Launch app** → Tap "🍳 I'm the Cook"
2. **Setup WiFi**:
   - App creates hotspot: `TapStream_<session_id>`
   - Password: `tapstream123`
   - Server: `http://192.168.43.1:8080`
3. **Show QR code** to diners
4. **Start streaming** → Camera opens, preview on 3x3 grid
5. **Monitor**: See viewer count, frame rate, quality

### As Diner (Stream Consumer)

1. **Connect to cook's WiFi** (scan QR or manual)
2. **Launch app** → Tap "🍽️ I'm a Diner"
3. **Enter cook ID** or auto-discover
4. **Watch stream** on 4x4 grid:
   - Main video display
   - Dish metadata (name, stage, temp, timer)
   - Interactive buttons (❤️ Like, 💬 Comment, 🔗 Share, 🔖 Save)
   - Status info (viewers, quality, latency)

## 🔬 Verify Intention Space Physics

### Check CPUX State

```bash
# While app is running
curl http://localhost:8080/cpux/state | jq
```

Output:
```json
{
  "cooks": [{
    "cook_id": "cook_1234567890",
    "cpux_id": "cook_cook_1234567890_cpux",
    "progressor_delta_ns": 1000000,
    "field_pulses": 12
  }],
  "diners": [{
    "diner_id": "diner_0987654321",
    "cpux_id": "diner_diner_0987654321_cpux",
    "progressor_delta_ns": 1000000,
    "field_pulses": 8
  }],
  "network_field_pulses": 24
}
```

### View Network Field Pulses

```bash
curl http://localhost:8080/field/pulses | jq
```

You'll see pulses like:
- `frame_123` (stream frames)
- `camera_frame` (captured frames)
- `encoded_frame` (encoded data)
- `decoded_frame` (decoded data)

Each pulse has:
- `id`: Unique identifier
- `name`: Human-readable name
- `tv`: Trivalent value (Y/N/U)
- `response`: Current value

## 🧪 Development Workflow

### With IPTP Shell

```bash
# Terminal 1: IPTP shell
./iptp

iptp> name "tapstream testing"
iptp> goto /path/to/tapstream

# Build and run
iptp> tapstream build-desktop
iptp> cargo run

# In another terminal
iptp> jump tapstream_testing
iptp> tapstream cpux-state
iptp> tapstream field-pulses
```

### Without IPTP Shell

```bash
# Terminal 1: Run app
cargo run

# Terminal 2: Monitor
curl http://localhost:8080/stats
curl http://localhost:8080/cpux/state
curl http://localhost:8080/field/pulses
```

## 🎯 Understanding the Flow

### Cook's CPUX

```
[Camera DN] → [Encoder Object] → [Network Field] → [Preview GL]
     ↓              ↓                   ↓               ↓
  Capture      Encode Frame      WiFi Pulses      Cook sees
  (compute)     (mapping)        (dynamic)        preview
```

### Diner's CPUX

```
[Network Field] → [Decoder Object] → [Display DN] → [Stream GL]
      ↓                ↓                   ↓             ↓
  WiFi Pulses    Decode Frame         Render       Diner sees
  (dynamic)       (mapping)         (compute)       stream
```

### I-O-I-DN-I-GL-I Flow

```
Intention → Object → Intention → Design Node → Intention → Grid Lookout → Intention
    ↓          ↓          ↓            ↓             ↓            ↓            ↓
  Signal    Mapping   Transfer    COMPUTE      Transfer    Display     Interact
```

## 🐛 Troubleshooting

### "No such file or directory" errors

**Problem:** Build fails because source files aren't found

**Solution:**
```bash
# Check if you're in the right directory
pwd  # Should end with /tapstream

# Verify all source files exist
ls -la src/
ls -la ui/

# If files are missing:
# Option 1: Re-extract archive
cd ..
tar -xzf tapstream.tar.gz
cd tapstream

# Option 2: Download individual files to correct locations
# Make sure directory structure matches:
# tapstream/
#   src/
#   ui/
```

### APK Build Fails

```bash
# Check NDK
echo $ANDROID_NDK_HOME

# Verify targets
rustup target list --installed

# Clean and rebuild
cargo clean
./build_android.sh
```

### Server Not Starting

```bash
# Check port availability
lsof -i :8080

# Try different port
TAPSTREAM_PORT=9090 cargo run
```

### Can't Connect from Diner

```bash
# Verify WiFi hotspot is active
# Check IP address
ip addr show wlan0

# Ping from diner device
ping 192.168.43.1
```

## 📚 Next Steps

1. **Read the docs**:
   - `README.md` - Full architecture
   - `SUMMARY.md` - Implementation details
   - `IPTP_INTEGRATION.md` - IPTP workflow

2. **Explore the code**:
   - `src/intention_space.rs` - Physics engine
   - `src/tapstream.rs` - Application logic
   - `ui/tapstream.slint` - Grid-based UI

3. **Extend the app**:
   - Add real camera integration
   - Implement video encoding
   - Add audio streaming
   - Create recording feature

4. **Deploy to production**:
   - Sign APK for release
   - Test on multiple devices
   - Optimize performance
   - Add error handling

## ✨ You're Ready!

TapStream is now running with **full Intention Space physics**:
- ✅ Immutable responses
- ✅ Computation locality (DN)
- ✅ Static mapping (Object)
- ✅ Dynamic fields
- ✅ ONE CELL = ONE PULSE
- ✅ Sequential resolution
- ✅ I-O-I-DN-I-GL-I flow

**Welcome to physics-based application development!**

---

Questions? Check the docs or examine the working code.

*The flow continues: I-O-I-DN-I-GL-I ...*
