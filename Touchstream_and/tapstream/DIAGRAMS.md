# TapStream Architecture Diagrams

## Complete System Architecture

```
╔══════════════════════════════════════════════════════════════════════╗
║                         IPTP SHELL LAYER (Go)                         ║
║  ┌────────────────────────────────────────────────────────────────┐  ║
║  │ Commands: name, goto, jump, back, save, list, state            │  ║
║  │ Extended: tapstream build-desktop, build-android, serve, etc.  │  ║
║  │ State: /tmp/iptp_state.json (processes, pulses, intentions)    │  ║
║  └────────────────────────────┬───────────────────────────────────┘  ║
╚═══════════════════════════════╪═══════════════════════════════════════╝
                                 │
                                 ↓
╔══════════════════════════════════════════════════════════════════════╗
║                   INTENTION SPACE PHYSICS ENGINE (Rust)              ║
║  ┌────────────────────────────────────────────────────────────────┐  ║
║  │  Intention → Object → Intention → DN → Intention → GL → I      │  ║
║  │      ↓          ↓          ↓        ↓        ↓        ↓        │  ║
║  │   Signal   Mapping   Transfer  COMPUTE  Transfer  Display      │  ║
║  └────────────────────────────────────────────────────────────────┘  ║
║                                                                       ║
║  Core Components:                                                     ║
║  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐            ║
║  │Intention │  │Design    │  │Object    │  │Field     │            ║
║  │Arc<Vec<  │  │Node (DN) │  │(Static)  │  │(Dynamic) │            ║
║  │Response>>│  │Compute!  │  │Mapping   │  │Pulses    │            ║
║  └──────────┘  └──────────┘  └──────────┘  └──────────┘            ║
║                                                                       ║
║  ┌──────────┐  ┌──────────┐  ┌──────────┐                          ║
║  │Grid      │  │CPUX      │  │Sequential│                          ║
║  │Lookout   │  │Thread    │  │Resolver  │                          ║
║  │1Cell=1P  │  │Progressor│  │MultiSens │                          ║
║  └──────────┘  └──────────┘  └──────────┘                          ║
╚═══════════════════════════════╪═══════════════════════════════════════╝
                                 │
                                 ↓
╔══════════════════════════════════════════════════════════════════════╗
║                    TAPSTREAM APPLICATION (Rust)                       ║
║                                                                       ║
║  COOK (Producer):                 NETWORK FIELD:                     ║
║  ┌─────────────────┐             ┌─────────────────┐                ║
║  │[Camera DN]      │             │  WiFi Network   │                ║
║  │   ↓ Capture     │             │  as Dynamic     │                ║
║  │[Encoder Object] │──Emit──────→│  Pulse Field    │                ║
║  │   ↓ Encode      │             │                 │                ║
║  │[Preview GL]     │             │  Pulses:        │                ║
║  │   ↓ Display     │             │  - frame_123    │                ║
║  │ 3x3 Grid        │             │  - camera_frame │                ║
║  │ 9 Pulses        │             │  - encoded_frame│                ║
║  └─────────────────┘             └────────┬────────┘                ║
║                                            │                         ║
║  DINER (Consumer):                         │                         ║
║  ┌─────────────────┐                       │                         ║
║  │[Decoder Object] │←──Absorb─────────────┘                         ║
║  │   ↓ Decode      │                                                 ║
║  │[Display DN]     │                                                 ║
║  │   ↓ Render      │                                                 ║
║  │[Stream GL]      │                                                 ║
║  │   ↓ Interact    │                                                 ║
║  │ 4x4 Grid        │                                                 ║
║  │ 16 Pulses       │                                                 ║
║  └─────────────────┘                                                 ║
╚═══════════════════════════════╪═══════════════════════════════════════╝
                                 │
                                 ↓
╔══════════════════════════════════════════════════════════════════════╗
║                    EMBEDDED HTTP SERVER (Rust + Warp)                 ║
║  ┌────────────────────────────────────────────────────────────────┐  ║
║  │ POST /cook/register      - Register as producer                │  ║
║  │ POST /cook/stream        - Start streaming                     │  ║
║  │ POST /diner/register     - Register as consumer                │  ║
║  │ POST /diner/watch        - Start watching                      │  ║
║  │ POST /diner/interact     - Handle interaction                  │  ║
║  │ GET  /stats              - Network statistics                  │  ║
║  │ GET  /cpux/state         - CPUX debugging                      │  ║
║  │ GET  /field/pulses       - Field pulses (debug)                │  ║
║  └────────────────────────────────────────────────────────────────┘  ║
║  Runs on: http://192.168.43.1:8080 (WiFi hotspot)                    ║
╚═══════════════════════════════╪═══════════════════════════════════════╝
                                 │
                                 ↓
╔══════════════════════════════════════════════════════════════════════╗
║                        SLINT UI LAYER (Declarative)                   ║
║                                                                       ║
║  Cook Preview (3x3):          Diner Stream (4x4):                    ║
║  ┌───┬───┬───┐                ┌───┬───┬───┬───┐                     ║
║  │ P │ P │ P │                │ Video Display │                     ║
║  ├───┼───┼───┤                ├───┼───┼───┼───┤                     ║
║  │ P │ P │ P │                │ P │ P │ P │ P │ Metadata            ║
║  ├───┼───┼───┤                ├───┼───┼───┼───┤                     ║
║  │ P │ P │ P │                │ ❤️│💬 │🔗 │🔖 │ Buttons             ║
║  └───┴───┴───┘                ├───┼───┼───┼───┤                     ║
║  Each cell = 1 Pulse          │ P │ P │ P │ P │ Status              ║
║                                └───┴───┴───┴───┘                     ║
║                                Each cell = 1 Pulse                    ║
║                                                                       ║
║  WiFi Setup:                                                          ║
║  ┌────────────────────────────────────┐                              ║
║  │  📡 TapStream WiFi                 │                              ║
║  │  SSID: TapStream_xyz               │                              ║
║  │  Password: tapstream123            │                              ║
║  │  Server: http://192.168.43.1:8080  │                              ║
║  │  ┌──────────────────────┐          │                              ║
║  │  │   QR Code Here       │          │                              ║
║  │  └──────────────────────┘          │                              ║
║  └────────────────────────────────────┘                              ║
╚═══════════════════════════════╪═══════════════════════════════════════╝
                                 │
                                 ↓
╔══════════════════════════════════════════════════════════════════════╗
║                    ANDROID NATIVE LAYER (Minimal JNI)                 ║
║  ┌────────────────────────────────────────────────────────────────┐  ║
║  │ MainActivity.java (~10 lines):                                 │  ║
║  │                                                                 │  ║
║  │   static { System.loadLibrary("tapstream"); }                  │  ║
║  │                                                                 │  ║
║  │   protected void onCreate(Bundle state) {                      │  ║
║  │       super.onCreate(state);                                   │  ║
║  │       runApp(); // Native call                                 │  ║
║  │   }                                                             │  ║
║  └────────────────────────────────────────────────────────────────┘  ║
║  Binary: libtapstream.so (~3 MB per arch)                            ║
╚══════════════════════════════════════════════════════════════════════╝
```

## CPUX Flow Detail

```
Cook's CPUX Thread (progressor_delta = 1ms):
┌──────────────────────────────────────────────────────────────┐
│                                                              │
│  Time:    0ms      1ms      2ms      3ms      4ms      5ms  │
│           │        │        │        │        │        │     │
│  Step 1:  [Camera DN]────→ Intention                        │
│           Capture frame    (compute!)                        │
│           │                                                  │
│  Step 2:  └─────→ [Encoder Object]────→ Intention           │
│                   Static mapping        (transfer)           │
│                   │                                          │
│  Step 3:          └─────→ [Network Field]────→ Intention    │
│                           WiFi emit          (pulse added)   │
│                           │                                  │
│  Step 4:                  └─────→ [Preview GL]────→ Intention│
│                                   Display     (update cells) │
│                                   │                          │
│  Loop:                            └──────────┐               │
│                                              │               │
│                       Back to Camera DN ←────┘               │
│                                                              │
└──────────────────────────────────────────────────────────────┘

Diner's CPUX Thread (progressor_delta = 1ms):
┌──────────────────────────────────────────────────────────────┐
│                                                              │
│  Step 1:  [Network Field]────→ Intention                    │
│           WiFi absorb         (pulse received)               │
│           │                                                  │
│  Step 2:  └─────→ [Decoder Object]────→ Intention           │
│                   Static mapping       (transfer)            │
│                   │                                          │
│  Step 3:          └─────→ [Display DN]────→ Intention       │
│                           Render        (compute!)           │
│                           │                                  │
│  Step 4:                  └─────→ [Stream GL]────→ Intention │
│                                   Update grid (16 cells)     │
│                                   │                          │
│  Human:                           └─→ Tap cell               │
│                                      │                       │
│  Sequential:                         └─→ Queue interaction   │
│                                         │                    │
│  Resolve:                               └─→ Process (1ms)    │
│                                            Emit intention    │
│                                                              │
└──────────────────────────────────────────────────────────────┘
```

## Pulse Flow Through Network

```
Cook Device:                Network (WiFi):         Diner Device:
┌─────────────┐            ┌─────────────┐         ┌─────────────┐
│             │            │   FIELD     │         │             │
│ Camera DN   │            │  (Dynamic   │         │ Decoder Obj │
│    ↓        │            │   Pulses)   │         │    ↓        │
│ frame_data  │──Emit─────→│             │─Absorb→ │ decode      │
│    ↓        │            │ ┌─────────┐ │         │    ↓        │
│ Encoder Obj │            │ │frame_123│ │         │ Display DN  │
│    ↓        │            │ │  TV: Y  │ │         │    ↓        │
│ encoded     │──Emit─────→│ │response:│ │─Absorb→ │ render      │
│    ↓        │            │ │  <data> │ │         │    ↓        │
│ Preview GL  │            │ └─────────┘ │         │ Stream GL   │
│  3x3 Grid   │            │             │         │  4x4 Grid   │
│  9 pulses   │            │ Multiple    │         │  16 pulses  │
│             │            │ subscribers │         │             │
└─────────────┘            └─────────────┘         └─────────────┘

Response Immutability:
┌──────────────────────────────────────────────────────────────┐
│ Intention created:                                           │
│   Response { pulse_ref: "camera_frame", value: "xyz", ... } │
│                                                              │
│ After Object mapping:                                        │
│   Response { pulse_ref: "encoded_frame", value: "xyz", ... }│
│                          ↑                        ↑          │
│                    CHANGED                  UNCHANGED        │
│                                                              │
│ Physics law: Only pulse_ref changes, value stays immutable! │
└──────────────────────────────────────────────────────────────┘
```

## Grid Lookout Structure

```
Cook's 3x3 Preview Grid Lookout:
┌─────────────────────────────────────────────────────────┐
│ Layer 0 (z-index: 0)                                    │
│                                                          │
│  Row 0:  ┌─────────┬─────────┬─────────┐               │
│          │Camera   │Frame    │Quality  │               │
│          │ready    │counter  │HD       │               │
│          │Pulse 1  │Pulse 2  │Pulse 3  │               │
│          └─────────┴─────────┴─────────┘               │
│                                                          │
│  Row 1:  ┌─────────┬─────────┬─────────┐               │
│          │Viewers  │Bitrate  │FPS      │               │
│          │0        │2Mbps    │30       │               │
│          │Pulse 4  │Pulse 5  │Pulse 6  │               │
│          └─────────┴─────────┴─────────┘               │
│                                                          │
│  Row 2:  ┌─────────┬─────────┬─────────┐               │
│          │Audio    │Flash    │Focus    │               │
│          │On       │Off      │Auto     │               │
│          │Pulse 7  │Pulse 8  │Pulse 9  │               │
│          └─────────┴─────────┴─────────┘               │
│                                                          │
│  Each cell = ONE PULSE (fundamental physics!)           │
│  Cell position: (layer, row, col)                       │
│  Pulse: { id, name, tv, response }                      │
│  Interaction DN: Optional<Arc<DesignNode>>              │
│                                                          │
│  Exposed Field: All 9 pulses accessible                 │
└─────────────────────────────────────────────────────────┘

Diner's 4x4 Stream Grid Lookout:
┌─────────────────────────────────────────────────────────┐
│ Layer 0 (z-index: 0)                                    │
│                                                          │
│  Row 0:  ┌─────────────────────────────────────────┐   │
│          │      Video Display (Main Stream)        │   │
│          │      Pulse 1 (large cell)               │   │
│          └─────────────────────────────────────────┘   │
│                                                          │
│  Row 1:  ┌────────┬────────┬────────┬────────┐        │
│          │Dish    │Stage   │Temp    │Timer   │        │
│          │name    │cooking │98°C    │8:00    │        │
│          │Pulse 5 │Pulse 6 │Pulse 7 │Pulse 8 │        │
│          └────────┴────────┴────────┴────────┘        │
│                                                          │
│  Row 2:  ┌────────┬────────┬────────┬────────┐        │
│          │❤️ Like │💬 Cmt  │🔗 Share│🔖 Save │        │
│          │P9 +DN  │P10 +DN │P11 +DN │P12 +DN │        │
│          └────────┴────────┴────────┴────────┘        │
│                                                          │
│  Row 3:  ┌────────┬────────┬────────┬────────┐        │
│          │Viewers │Status  │Quality │Latency │        │
│          │12      │Live    │Auto    │15ms    │        │
│          │Pulse 13│Pulse 14│Pulse 15│Pulse 16│        │
│          └────────┴────────┴────────┴────────┘        │
│                                                          │
│  Buttons (Row 2) have interaction DNs attached          │
│  On tap: DN executes → emits intention → field absorbs  │
└─────────────────────────────────────────────────────────┘
```

## Sequential Interaction Resolution

```
Human Multi-Sensory Input:
┌──────────────────────────────────────────────────────────┐
│                                                          │
│  Time:     t0      t0+5ms   t0+12ms  t0+20ms           │
│           │        │         │        │                 │
│  Tap ────→│        │         │        │                 │
│  See ─────┼──────→ │         │        │                 │
│  Hear ────┼────────┼───────→ │        │                 │
│  Hold ────┼────────┼─────────┼──────→ │                 │
│                                                          │
│  Queue: [Tap@t0, See@t0+5, Hear@t0+12, Hold@t0+20]     │
│                                                          │
└──────────────────────────────────────────────────────────┘
                         ↓
┌──────────────────────────────────────────────────────────┐
│  Sequential Resolution (progressor_delta = 1ms):         │
│                                                          │
│  Process 1: [Tap]   ──1ms──→ Emit intention → Field     │
│  Process 2: [See]   ──1ms──→ Emit intention → Field     │
│  Process 3: [Hear]  ──1ms──→ Emit intention → Field     │
│  Process 4: [Hold]  ──1ms──→ Emit intention → Field     │
│                                                          │
│  Total: 4ms to process all interactions                 │
│  Minimal delta ensures proper ordering                  │
│                                                          │
└──────────────────────────────────────────────────────────┘

Physics Law: Multi-dimensional inputs MUST resolve sequentially
            with progressor delta time between each.
```

## File Size Comparison

```
Traditional Android App:
├─ Java/Kotlin code:    ~15 MB
├─ Android runtime:     ~20 MB
├─ Dependencies:        ~15 MB
├─ Resources:           ~5 MB
└─ Total:              ~55 MB

TapStream (Intention Space):
├─ Rust native lib:     ~3 MB (per arch)
├─ Slint runtime:       ~1 MB
├─ Minimal JNI:         ~100 KB
├─ Resources:           ~500 KB
└─ Total:              ~5 MB

Savings: 50 MB (90% reduction!)
```

## Build Time Comparison

```
Traditional Android App:
├─ Gradle setup:        ~30s
├─ Java compilation:    ~45s
├─ Dex compilation:     ~25s
├─ APK assembly:        ~10s
└─ Total:              ~110s

TapStream (Intention Space):
├─ Rust compilation:    ~90s (first build)
│                       ~5s (incremental)
├─ Slint compilation:   ~10s
├─ APK assembly:        ~5s
└─ Total:              ~105s (first)
                       ~20s (incremental)

Incremental builds: 5x faster!
```

---

**Visual Summary: Physics meets practice in every layer**

*From IPTP shell orchestration down to Android native, the same physics apply*
