# TapStream Setup Guide (Visual)

## 📦 Step-by-Step Setup

```
┌─────────────────────────────────────────────────────────────┐
│ STEP 0: EXTRACT/SETUP PROJECT (MOST IMPORTANT!)            │
└─────────────────────────────────────────────────────────────┘

Option A: Extract Archive (Easiest)
────────────────────────────────────────────────────────────────

1. Download tapstream.tar.gz
   
2. Extract it:
   $ tar -xzf tapstream.tar.gz
   
3. You now have this structure:
   
   tapstream/
   ├── src/
   │   ├── intention_space.rs  ✓ Physics engine
   │   ├── tapstream.rs        ✓ App logic
   │   ├── server.rs           ✓ HTTP server
   │   └── main.rs             ✓ UI integration
   ├── ui/
   │   └── tapstream.slint     ✓ UI definition
   ├── Cargo.toml              ✓ Dependencies
   ├── build.rs                ✓ Build script
   └── build_android.sh        ✓ APK builder

4. Verify:
   $ cd tapstream
   $ ls src/
   intention_space.rs  main.rs  server.rs  tapstream.rs
   
   ✅ All 4 files present? Good!
   ❌ Missing files? Re-extract the archive


Option B: Download Directory (Browse Files)
────────────────────────────────────────────────────────────────

1. Download the tapstream/ folder
   
2. Make sure you have ALL these files:
   
   Required Files:
   ☐ src/intention_space.rs   (13 KB)
   ☐ src/tapstream.rs         (17 KB)
   ☐ src/server.rs            (11 KB)
   ☐ src/main.rs              (15 KB)
   ☐ ui/tapstream.slint       (16 KB)
   ☐ Cargo.toml               (1.4 KB)
   ☐ build.rs                 (109 B)
   ☐ build_android.sh         (7.9 KB)

3. Verify structure:
   $ cd tapstream
   $ find . -type f -name "*.rs"
   ./src/intention_space.rs
   ./src/tapstream.rs
   ./src/server.rs
   ./src/main.rs
   
   ✅ All 4 Rust files? Good!
   ❌ Missing? Download them individually


Option C: Manual Setup (File-by-File)
────────────────────────────────────────────────────────────────

1. Create structure:
   $ mkdir -p tapstream/src tapstream/ui
   
2. Download and place files:
   
   intention_space.rs  →  tapstream/src/
   tapstream.rs        →  tapstream/src/
   server.rs           →  tapstream/src/
   main.rs             →  tapstream/src/
   tapstream.slint     →  tapstream/ui/
   Cargo.toml          →  tapstream/
   build.rs            →  tapstream/
   build_android.sh    →  tapstream/

3. Verify:
   $ cd tapstream
   $ ls -la src/
   drwxr-xr-x  intention_space.rs
   drwxr-xr-x  tapstream.rs
   drwxr-xr-x  server.rs
   drwxr-xr-x  main.rs


┌─────────────────────────────────────────────────────────────┐
│ STEP 1: BUILD DESKTOP VERSION                               │
└─────────────────────────────────────────────────────────────┘

Prerequisites:
──────────────
1. Install Rust:
   $ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
2. Verify:
   $ rustc --version
   $ cargo --version

Build:
──────
$ cd tapstream      # Make sure you're in the right directory!
$ cargo build --release

Expected output:
   Compiling tapstream v1.0.0 (/path/to/tapstream)
   ...
   Finished release [optimized] target(s) in 1m 30s

Run:
────
$ cargo run

Expected result:
   🚀 TapStream - Intention Space Live Streaming
      Physics: I-O-I-DN-I-GL-I Flow
      Grid: ONE CELL = ONE PULSE
   
   [App window opens]


┌─────────────────────────────────────────────────────────────┐
│ STEP 2: BUILD ANDROID APK (Optional)                        │
└─────────────────────────────────────────────────────────────┘

Prerequisites:
──────────────
1. Install Android NDK:
   Download from: https://developer.android.com/ndk/downloads
   
2. Set environment:
   $ export ANDROID_NDK_HOME=/path/to/ndk
   
3. Install targets:
   $ rustup target add aarch64-linux-android armv7-linux-androideabi

Build:
──────
$ cd tapstream
$ chmod +x build_android.sh
$ ./build_android.sh

Expected output:
   🚀 TapStream - Android APK Build
   🔨 Building Rust libraries...
      Building for arm64-v8a...
      ✓ Built arm64-v8a
      Building for armeabi-v7a...
      ✓ Built armeabi-v7a
   📦 Building APK...
   ✅ APK built successfully!
      Output: output/TapStream.apk
      Size: 5.2M

Install:
────────
$ adb install output/TapStream.apk


┌─────────────────────────────────────────────────────────────┐
│ COMMON ERRORS & SOLUTIONS                                   │
└─────────────────────────────────────────────────────────────┘

❌ Error: "could not find `Cargo.toml`"
────────────────────────────────────────────
   Problem: You're not in the right directory
   
   Solution:
   $ cd tapstream      # Make sure you're in project root
   $ ls Cargo.toml     # Should see the file


❌ Error: "file not found: src/intention_space.rs"
────────────────────────────────────────────────────
   Problem: Source files missing or not extracted
   
   Solution:
   $ ls src/           # Check if files exist
   
   If empty:
   $ cd ..
   $ tar -xzf tapstream.tar.gz    # Re-extract
   $ cd tapstream


❌ Error: "no such file: ui/tapstream.slint"
────────────────────────────────────────────
   Problem: UI file missing
   
   Solution:
   $ ls ui/tapstream.slint    # Check if file exists
   
   If missing, download it and place in ui/ folder


❌ Build takes forever / stuck
───────────────────────────────
   Problem: First build downloads dependencies
   
   Solution: Wait! First build takes 2-5 minutes
   Subsequent builds: 30 seconds


┌─────────────────────────────────────────────────────────────┐
│ VERIFICATION CHECKLIST                                       │
└─────────────────────────────────────────────────────────────┘

Before building, verify:

Directory Structure:
├─ ☐ You're in the "tapstream" directory
├─ ☐ src/ directory exists
├─ ☐ ui/ directory exists
├─ ☐ Cargo.toml exists in current directory

Source Files (4 Rust files):
├─ ☐ src/intention_space.rs exists (~13 KB)
├─ ☐ src/tapstream.rs exists (~17 KB)
├─ ☐ src/server.rs exists (~11 KB)
└─ ☐ src/main.rs exists (~15 KB)

UI File:
└─ ☐ ui/tapstream.slint exists (~16 KB)

Build Files:
├─ ☐ Cargo.toml exists (~1.4 KB)
├─ ☐ build.rs exists (~109 B)
└─ ☐ build_android.sh exists (~7.9 KB)

Tools Installed:
├─ ☐ rustc --version works
├─ ☐ cargo --version works
└─ ☐ For Android: $ANDROID_NDK_HOME is set

All checked? Run: cargo build --release


┌─────────────────────────────────────────────────────────────┐
│ FINAL DIRECTORY STRUCTURE                                    │
└─────────────────────────────────────────────────────────────┘

This is what you should have before building:

tapstream/                          ← You are here (cd tapstream)
│
├── src/                            ← Rust source code
│   ├── intention_space.rs         ← IS physics engine (13 KB)
│   ├── tapstream.rs               ← Cook/Diner logic (17 KB)
│   ├── server.rs                  ← HTTP server (11 KB)
│   └── main.rs                    ← Slint integration (15 KB)
│
├── ui/                             ← Slint UI definitions
│   └── tapstream.slint            ← Grid UI (16 KB)
│
├── Cargo.toml                      ← Rust dependencies (1.4 KB)
├── build.rs                        ← Build script (109 B)
├── build_android.sh                ← APK builder (7.9 KB)
│
└── Documentation (optional):
    ├── README.md
    ├── QUICKSTART.md
    ├── INDEX.md
    └── ...

After first build, you'll also have:
├── target/                         ← Build outputs
│   └── release/
│       └── tapstream              ← Desktop binary (~8 MB)
│
└── output/                         ← Android builds
    └── TapStream.apk              ← Android APK (~5 MB)


═══════════════════════════════════════════════════════════════

🎯 REMEMBER: Extract/setup files FIRST (Step 0), THEN build!

═══════════════════════════════════════════════════════════════
