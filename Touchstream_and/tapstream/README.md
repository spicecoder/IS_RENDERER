# TapStream - Intention Space Live Streaming

**Cook → Network Field → Diner**

A lightweight Android streaming app built on **Intention Space physics** using Rust + Slint, bypassing Java/Kotlin layers entirely.

## 🎯 Core Concept

TapStream demonstrates Intention Space computing in a real-world use case: live streaming cooking processes from cook to diner over local WiFi (hotel, restaurant, hospital, etc.).

### Intention Space Physics

```
Cook's CPUX:    [Camera DN] → [Encoder O] → [Network Field] → [Preview GL]
                     ↓              ↓               ↓              ↓
                   Frame      Encode Frame    WiFi Pulses    Cook Preview
                     
Diner's CPUX:   [Network Field] → [Decoder O] → [Display DN] → [Stream GL]
                      ↓               ↓              ↓              ↓
                 WiFi Pulses    Decode Frame    Render      Diner Interface
```

### The Flow: I-O-I-DN-I-GL-I

```
Intention → Object → Intention → Design Node → Intention → Grid Lookout → Intention
    ↓          ↓          ↓            ↓             ↓            ↓            ↓
  Signal    Mapping   Transfer    COMPUTE      Transfer    Display     Human
                                 (Physics!)                           Interaction
```

## 🏗️ Architecture

### 1. **Intention Space Physics Engine** (`intention_space.rs`)

Core implementation of IS physics:
- **Intentions**: Immutable response carriers
- **Design Nodes (DN)**: Where computation happens
- **Objects (O)**: Static mapping, sync with progressor
- **Fields**: Dynamic pulse collections (WiFi network = Field!)
- **Grid Lookout (GL)**: ONE CELL = ONE PULSE
- **CPUX Threads**: Execute I-O-I-DN-I-GL-I flow
- **Sequential Interaction Resolver**: Multi-sensory → sequential with progressor delta

### 2. **TapStream Application** (`tapstream.rs`)

Application-specific logic:
- **Cook Producer**: Camera DN → Encoder O → Network Field
- **Diner Consumer**: Network Field → Decoder O → Display DN → GL
- **Network Field**: WiFi as dynamic pulse collection
- **Stream Frames**: Data carried as pulses

### 3. **Embedded HTTP Server** (`server.rs`)

Runs on Android device's WiFi hotspot:
- REST API for cook/diner registration
- Stream control endpoints
- Field pulse inspection (debugging)
- CPUX state monitoring

### 4. **Slint UI** (`ui/tapstream.slint`)

Grid-based responsive UI:
- **Cook Preview**: 3x3 grid (9 pulses)
- **Diner Stream**: 4x4 grid (16 pulses)
- **WiFi Setup**: QR code & credentials
- Direct mapping: Grid Cell ↔ Pulse

## 📱 Use Cases

### Hotel/Restaurant
- Cook streams preparation in real-time
- Diners watch their meal being prepared
- No internet required (local WiFi)

### Surgery/Medical
- Surgeon streams procedure
- Medical students watch from observation room
- Low latency over hospital WiFi

### Workshop/Training
- Expert demonstrates technique
- Trainees watch on their devices
- Interactive feedback via grid interactions

## 🚀 Building

### Desktop (Development)
```bash
cargo build --release
cargo run
```

### Android APK (Production)
```bash
# Install Android NDK
rustup target add aarch64-linux-android

# Build for Android
cargo ndk -t arm64-v8a -o app/src/main/jniLibs build --release

# Create APK (uses minimal JNI wrapper)
./build_android.sh
```

### APK Size
- Rust binary: ~3 MB
- Slint runtime: ~1 MB
- Minimal JNI wrapper: ~100 KB
- **Total: ~5 MB** (vs 20-50 MB typical Android app)

## 🎮 Usage

### As Cook (Producer)
1. Launch app → "I'm the Cook"
2. Start WiFi hotspot
3. Show QR code to diners
4. Start streaming
5. Preview shows on 3x3 grid

### As Diner (Consumer)
1. Connect to cook's WiFi
2. Launch app → "I'm a Diner"
3. Scan QR or enter cook ID
4. Watch stream on 4x4 grid
5. Interact: ❤️ Like, 💬 Comment, 🔗 Share, 🔖 Save

## 🔬 Intention Space Physics Details

### Immutability of Responses
```rust
// Responses are IMMUTABLE
let intention = Intention::with_responses(
    "frame_123",
    vec![Response {
        pulse_ref: PulseRef { pulse_id: "camera_frame" },
        value: "frame_data_xyz",  // IMMUTABLE
        tv: TV::Y,                // IMMUTABLE
    }]
);

// Only pulse references change
let transferred = intention.transfer_to_pulse(
    "camera_frame",    // from
    "encoded_frame"    // to
);
// value "frame_data_xyz" stays the same!
```

### Object vs Field
```rust
// Object: STATIC collection, configured mapping
let encoder = Object::new(
    "encoder",
    vec![/* predefined pulses */],
    Arc::new(|i| i.transfer_to_pulse("in", "out"))  // Fixed mapping
);

// Field: DYNAMIC collection, updated by emissions
let network_field = Field::new("wifi");
network_field.absorb_intention(&intention);  // Adds/updates pulses
```

### One Cell = One Pulse
```rust
struct GridCell {
    position: CellPosition,
    pulse: Pulse,              // Single pulse!
    interaction_dn: Option<DN>, // DN that emits I from this cell
}
```

### Sequential Interaction Resolution
```rust
// Human: click + see + hear (multi-sensory)
// System: processes sequentially with progressor delta time
resolver.queue_interaction(InteractionEvent {
    interaction_type: InteractionType::Tap,
    cell_position: CellPosition { layer: 0, row: 2, col: 0 },
    timestamp_ns: now(),
});

// Progressor ensures minimal delta between processing
loop {
    sleep(Duration::from_nanos(progressor_delta_ns));
    let intention = process_next_interaction();
    field.absorb_intention(&intention);
}
```

## 🌐 Network as Field

The WiFi network itself is an **Intention Space Field**:

```rust
pub struct NetworkField {
    pub field: Arc<Field>,  // Field with dynamic pulses
    pub stream_channel: broadcast::Sender<StreamFrame>,
}

// When cook emits frame
network_field.publish_frame(frame);  // Pulse added to field

// Diners absorb from field
let mut receiver = network_field.subscribe();
let frame = receiver.recv().await;   // Pulse absorbed from field
```

## 🔧 Integration with IPTP Shell

TapStream can be orchestrated by IPTP shell:

```bash
# In IPTP shell
iptp> name "tapstream development"
iptp> goto ~/tapstream

# Start TapStream server
iptp> tapstream serve --port 8080

# Monitor CPUX state
iptp> tapstream cpux-state

# Deploy to device
iptp> tapstream deploy --device pixel6
```

## 📊 Why This Architecture?

### ✅ Advantages

1. **No Java/Kotlin**: Direct native compilation
2. **Small APK**: 5 MB vs 20-50 MB
3. **Physics-Based**: Intention Space guarantees correct flow
4. **Rapid Prototyping**: IPTP shell orchestration
5. **Cross-Platform**: Same code for Android/iOS/Desktop
6. **Grid-Based UI**: Universal responsive layout
7. **Sequential Interactions**: Proper human-machine physics

### 🎯 Intention Space Benefits

1. **Immutable Responses**: No state corruption
2. **Pulse Traceability**: Every cell = one pulse
3. **CPUX Flow**: Clear execution model (I-O-I-DN-I-GL-I)
4. **Field Dynamics**: Network naturally modeled as field
5. **Progressor Timing**: Proper sequential resolution
6. **Design-Time Pulses**: Predetermined at design time

## 📖 Learn More

- [Intention Space Paper](../Intention_Space_and_PnR_Computing.docx)
- [Grid Layout Spec](../grid_layout_)
- [IPTP Shell](https://github.com/spicecoder/IS_ITTP_Shell)

## 🤝 Contributing

TapStream is a demonstration of Intention Space physics. Contributions welcome:

1. Additional use cases (surgery, workshops, etc.)
2. Camera integration (real video capture)
3. Audio streaming
4. Quality adaptation
5. Multi-cook support
6. Recording/playback

## 📄 License

MIT License - See LICENSE file

---

**Built with Intention Space Physics**  
*Where computation follows physical laws*

```
I-O-I-DN-I-O-I-DN-I-GL-I-O-I-DN-I...
└─────────────────┬─────────────────┘
            CPUX Thread
        (Progressor-driven)
```
