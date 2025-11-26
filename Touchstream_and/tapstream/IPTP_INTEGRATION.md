# IPTP Shell Integration with TapStream

This document shows how IPTP shell (from IS_ITTP_Shell repo) orchestrates the TapStream application lifecycle, enabling rapid prototyping and deployment.

## Overview

```
┌─────────────────────────────────────────────────────────────┐
│                     IPTP Shell (Go)                         │
│  • State management (processes, pulses, intentions)         │
│  • Directory navigation (goto, jump, back)                  │
│  • Command orchestration                                    │
└──────────────────────┬──────────────────────────────────────┘
                       │
                       ├──→ Build Commands
                       ├──→ Deploy Commands  
                       ├──→ Monitor Commands
                       │
┌──────────────────────┴──────────────────────────────────────┐
│                TapStream App (Rust + Slint)                 │
│  • Intention Space physics engine                           │
│  • Cook producer / Diner consumer                           │
│  • Embedded HTTP server                                     │
│  • Grid-based UI                                            │
└─────────────────────────────────────────────────────────────┘
```

## IPTP Shell Commands for TapStream

### 1. Project Setup

```bash
# Start IPTP shell
iptp

# Name your shell with intention
iptp> name "developing tapstream streaming app"
✓ Shell named: tapstream_streaming_app
  Intention: developing tapstream streaming app

# Navigate to project
iptp> goto ~/projects/tapstream
✓ Changed to: /home/user/projects/tapstream

# Save state
iptp> save
✓ Saved state for: tapstream_streaming_app @ /home/user/projects/tapstream
```

### 2. Build Commands (Extended IPTP)

Add these to IPTP shell's `shell.go`:

```go
// Extended commands for TapStream development
case "tapstream":
    sh.cmdTapStream(args)
```

```go
// cmdTapStream handles TapStream-specific commands
func (sh *Shell) cmdTapStream(args []string) {
    if len(args) == 0 {
        fmt.Println("TapStream commands:")
        fmt.Println("  build-desktop    - Build for desktop testing")
        fmt.Println("  build-android    - Build APK for Android")
        fmt.Println("  serve            - Start HTTP server")
        fmt.Println("  deploy DEVICE    - Deploy to Android device")
        fmt.Println("  cpux-state       - Show CPUX state")
        fmt.Println("  field-pulses     - Show network field pulses")
        fmt.Println("  watch            - Watch and rebuild on changes")
        return
    }

    cmd := args[0]
    cmdArgs := args[1:]

    switch cmd {
    case "build-desktop":
        sh.tapstreamBuildDesktop()
    case "build-android":
        sh.tapstreamBuildAndroid()
    case "serve":
        sh.tapstreamServe(cmdArgs)
    case "deploy":
        sh.tapstreamDeploy(cmdArgs)
    case "cpux-state":
        sh.tapstreamCPUXState()
    case "field-pulses":
        sh.tapstreamFieldPulses()
    case "watch":
        sh.tapstreamWatch()
    default:
        fmt.Printf("Unknown tapstream command: %s\n", cmd)
    }
}

func (sh *Shell) tapstreamBuildDesktop() {
    fmt.Println("🔨 Building TapStream for desktop...")
    
    pulse := Pulse{
        Name:     "build started",
        TV:       "Y",
        Response: "desktop",
    }
    
    sh.state.SetProcess(sh.currentProcess, "Building TapStream", "")
    sh.state.Save()
    
    // Execute build
    cmd := exec.Command("cargo", "build", "--release")
    output, err := cmd.CombinedOutput()
    
    if err != nil {
        pulse.TV = "N"
        pulse.Response = string(output)
        fmt.Printf("❌ Build failed:\n%s\n", output)
    } else {
        pulse.TV = "Y"
        fmt.Println("✅ Build successful!")
        fmt.Println("   Binary: target/release/tapstream")
    }
}

func (sh *Shell) tapstreamBuildAndroid() {
    fmt.Println("📱 Building TapStream APK...")
    
    // Execute build script
    cmd := exec.Command("./build_android.sh")
    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    
    if err := cmd.Run(); err != nil {
        fmt.Printf("❌ Android build failed: %v\n", err)
    } else {
        fmt.Println("✅ APK built: output/TapStream.apk")
    }
}

func (sh *Shell) tapstreamServe(args []string) {
    port := "8080"
    if len(args) > 0 {
        port = args[0]
    }
    
    fmt.Printf("🚀 Starting TapStream server on port %s...\n", port)
    
    // Run in background
    cmd := exec.Command("cargo", "run", "--release")
    cmd.Env = append(os.Environ(), fmt.Sprintf("TAPSTREAM_PORT=%s", port))
    
    if err := cmd.Start(); err != nil {
        fmt.Printf("❌ Failed to start server: %v\n", err)
        return
    }
    
    fmt.Printf("✅ Server started (PID: %d)\n", cmd.Process.Pid)
    fmt.Printf("   Access at: http://localhost:%s\n", port)
    
    // Save PID to state
    pulse := Pulse{
        Name:     "server running",
        TV:       "Y",
        Response: fmt.Sprintf("PID:%d PORT:%s", cmd.Process.Pid, port),
    }
    sh.state.Processes[sh.currentProcess].Pulses = append(
        sh.state.Processes[sh.currentProcess].Pulses,
        pulse,
    )
    sh.state.Save()
}

func (sh *Shell) tapstreamDeploy(args []string) {
    if len(args) == 0 {
        fmt.Println("Usage: tapstream deploy DEVICE")
        fmt.Println("   e.g., tapstream deploy pixel6")
        return
    }
    
    device := args[0]
    fmt.Printf("📱 Deploying to %s...\n", device)
    
    // Check if APK exists
    if _, err := os.Stat("output/TapStream.apk"); os.IsNotExist(err) {
        fmt.Println("❌ APK not found. Run 'tapstream build-android' first.")
        return
    }
    
    // Use adb to install
    cmd := exec.Command("adb", "-s", device, "install", "-r", "output/TapStream.apk")
    output, err := cmd.CombinedOutput()
    
    if err != nil {
        fmt.Printf("❌ Deployment failed:\n%s\n", output)
    } else {
        fmt.Println("✅ Deployed successfully!")
        fmt.Printf("   Device: %s\n", device)
    }
}

func (sh *Shell) tapstreamCPUXState() {
    fmt.Println("📊 Querying CPUX state...")
    
    resp, err := http.Get("http://localhost:8080/cpux/state")
    if err != nil {
        fmt.Printf("❌ Failed to connect: %v\n", err)
        return
    }
    defer resp.Body.Close()
    
    var state map[string]interface{}
    if err := json.NewDecoder(resp.Body).Decode(&state); err != nil {
        fmt.Printf("❌ Failed to parse response: %v\n", err)
        return
    }
    
    // Pretty print
    data, _ := json.MarshalIndent(state, "", "  ")
    fmt.Println(string(data))
}

func (sh *Shell) tapstreamFieldPulses() {
    fmt.Println("🌐 Network field pulses...")
    
    resp, err := http.Get("http://localhost:8080/field/pulses")
    if err != nil {
        fmt.Printf("❌ Failed to connect: %v\n", err)
        return
    }
    defer resp.Body.Close()
    
    var pulses []Pulse
    if err := json.NewDecoder(resp.Body).Decode(&pulses); err != nil {
        fmt.Printf("❌ Failed to parse response: %v\n", err)
        return
    }
    
    fmt.Printf("\nFound %d pulses:\n\n", len(pulses))
    for _, p := range pulses {
        fmt.Printf("  • %s [%s] = %s\n", p.Name, p.TV, p.Response)
    }
}

func (sh *Shell) tapstreamWatch() {
    fmt.Println("👀 Watching for changes...")
    fmt.Println("   Press Ctrl+C to stop")
    
    // Use fswatch or similar
    // This is a simplified version
    cmd := exec.Command("cargo", "watch", "-x", "build")
    cmd.Stdout = os.Stdout
    cmd.Stderr = os.Stderr
    cmd.Run()
}
```

## Usage Examples

### Complete Development Workflow

```bash
# 1. Start and name shell
iptp> name "tapstream rapid prototyping"
✓ Shell named: tapstream_rapid_prototyping

# 2. Navigate to project
iptp> goto ~/projects/tapstream
✓ Changed to: /home/user/projects/tapstream

# 3. Build desktop version for testing
iptp> tapstream build-desktop
🔨 Building TapStream for desktop...
✅ Build successful!
   Binary: target/release/tapstream

# 4. Start server
iptp> tapstream serve
🚀 Starting TapStream server on port 8080...
✅ Server started (PID: 12345)
   Access at: http://localhost:8080

# 5. Check CPUX state
iptp> tapstream cpux-state
📊 Querying CPUX state...
{
  "cooks": [
    {
      "cook_id": "cook_1234567890",
      "cpux_id": "cook_cook_1234567890_cpux",
      "progressor_delta_ns": 1000000,
      "field_pulses": 12
    }
  ],
  "diners": [
    {
      "diner_id": "diner_0987654321",
      "cpux_id": "diner_diner_0987654321_cpux",
      "progressor_delta_ns": 1000000,
      "field_pulses": 8
    }
  ],
  "network_field_pulses": 24
}

# 6. View network field pulses
iptp> tapstream field-pulses
🌐 Network field pulses...

Found 24 pulses:

  • stream_frame [Y] = {"frame_id":123,...}
  • camera_frame [Y] = frame_captured_1234567890
  • encoded_frame [Y] = ...
  • decoded_frame [Y] = ...
  ...

# 7. Build APK
iptp> tapstream build-android
📱 Building TapStream APK...
🔨 Building Rust libraries...
   Building for arm64-v8a...
   ✓ Built arm64-v8a
   Building for armeabi-v7a...
   ✓ Built armeabi-v7a
📦 Building APK...
✅ APK built successfully!
   Output: output/TapStream.apk
   Size: 5.2M

# 8. Deploy to device
iptp> tapstream deploy pixel6
📱 Deploying to pixel6...
✅ Deployed successfully!
   Device: pixel6

# 9. Save current state
iptp> save
✓ Saved state for: tapstream_rapid_prototyping @ /home/user/projects/tapstream

# 10. Jump back later
iptp> jump tapstream_rapid_prototyping
✓ Jumped to tapstream_rapid_prototyping @ /home/user/projects/tapstream
```

### Parallel Development Contexts

```bash
# Terminal 1: Backend work
iptp> name "tapstream backend fixes"
iptp> goto ~/projects/tapstream/src
iptp> # Edit intention_space.rs
iptp> tapstream build-desktop
iptp> tapstream serve 8080

# Terminal 2: UI work
iptp> name "tapstream ui improvements"  
iptp> goto ~/projects/tapstream/ui
iptp> # Edit tapstream.slint
iptp> tapstream build-desktop
iptp> cargo run

# Terminal 3: Android deployment
iptp> name "tapstream android testing"
iptp> goto ~/projects/tapstream
iptp> tapstream build-android
iptp> tapstream deploy pixel6

# List all active contexts
iptp> list
=== Available Processes ===
  → tapstream_backend_fixes: /home/user/projects/tapstream/src (PID: 11111)
  → tapstream_ui_improvements: /home/user/projects/tapstream/ui (PID: 22222)
  → tapstream_android_testing: /home/user/projects/tapstream (PID: 33333)
```

## Pattern Language Scripts

Create reusable patterns for common workflows:

```bash
# File: tapstream_dev.iptp
intention "complete tapstream development cycle"
name tapstream-dev

# Navigate to project
goto ~/projects/tapstream

# Build sequence
pulse "desktop_build" tv=U response=""
pulse "android_build" tv=U response=""
pulse "deployment" tv=U response=""

# Design chunks (commands)
chunk build_desktop {
  precondition: desktop_build.tv == "U"
  action: tapstream build-desktop
  postcondition: desktop_build.tv = "Y"
}

chunk build_android {
  precondition: desktop_build.tv == "Y" && android_build.tv == "U"
  action: tapstream build-android
  postcondition: android_build.tv = "Y"
}

chunk deploy_device {
  precondition: android_build.tv == "Y" && deployment.tv == "U"
  action: tapstream deploy pixel6
  postcondition: deployment.tv = "Y"
}

# CPUX flow
cpux dev_flow {
  members: [build_desktop, build_android, deploy_device]
  progressor_delta: 1s
}

# Execute
run cpux dev_flow
```

Usage:
```bash
iptp> source tapstream_dev.iptp
✓ Loaded pattern: tapstream-dev
✓ Starting CPUX: dev_flow

🔨 Building desktop...
✅ desktop_build [Y]

📱 Building Android...
✅ android_build [Y]

📱 Deploying...
✅ deployment [Y]

✓ CPUX complete!
```

## State Persistence

IPTP shell maintains full state across sessions:

```json
{
  "processes": {
    "tapstream_rapid_prototyping": {
      "intention": "tapstream rapid prototyping",
      "current_dir": "/home/user/projects/tapstream",
      "history": [
        "/home/user/projects",
        "/home/user"
      ],
      "pid": 12345,
      "timestamp": "2024-01-15T10:30:00Z",
      "pulses": [
        {
          "name": "process named",
          "TV": "Y",
          "response": "tapstream_rapid_prototyping"
        },
        {
          "name": "directory saved",
          "TV": "Y",
          "response": "/home/user/projects/tapstream"
        },
        {
          "name": "server running",
          "TV": "Y",
          "response": "PID:12345 PORT:8080"
        },
        {
          "name": "build completed",
          "TV": "Y",
          "response": "desktop,android"
        }
      ]
    }
  }
}
```

## Integration Benefits

1. **Unified State Management**: IPTP tracks all dev contexts
2. **Intention-Driven**: Every action has explicit intention
3. **Pulse Tracking**: Build status, server state, all tracked
4. **Quick Context Switching**: `jump` between different tasks
5. **History Navigation**: `back` through directory history
6. **Pattern Reuse**: Save and replay workflows
7. **Cross-Session Persistence**: Resume exactly where you left off

## Future Extensions

- **Remote deployment**: Deploy to multiple devices in parallel
- **CI/CD integration**: IPTP as CI/CD orchestrator
- **Live reload**: Auto-rebuild and hot-reload on Android
- **CPUX monitoring dashboard**: Real-time pulse visualization
- **Distributed builds**: Parallel builds across machines
- **Team collaboration**: Shared IPTP state for team

---

**IPTP Shell + TapStream = Rapid Intention Space Development**

*Where your development workflow follows the same physics as your application*
