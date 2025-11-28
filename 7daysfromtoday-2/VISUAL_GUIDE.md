# Visual Guide: Intention Space Flow in "7 Days from Today"

## 🎨 Complete Visual Walkthrough

### The Big Picture

```
╔═══════════════════════════════════════════════════════════════════════════╗
║                    INTENTION SPACE PHYSICS                                ║
║                    I-O-I-DN-I-GL-I Pattern                               ║
║                                                                           ║
║  Like a factory assembly line - each station has ONE job                 ║
╚═══════════════════════════════════════════════════════════════════════════╝

          YOU                    APP                     RESULT
           ↓                      ↓                        ↓
    ┌──────────┐           ┌──────────┐           ┌──────────┐
    │  Action  │  →  →  →  │   Flow   │  →  →  →  │  Display │
    │  Click   │           │ Pipeline  │           │  Update  │
    └──────────┘           └──────────┘           └──────────┘
```

---

## 📱 Example 1: Adding a Todo

### What You See

```
┌─────────────────────────────────────────────────────┐
│  Wednesday, Nov 27, 2025        [Add Todo Section]  │
│  ┌───────────────────────────────────────────────┐  │
│  │  [   Buy groceries          ] [➕]           │  │
│  └───────────────────────────────────────────────┘  │
│                                                      │
│  Existing todos:                                     │
│  ☐ Review project                                   │
│  ☐ Call dentist                                     │
└─────────────────────────────────────────────────────┘

You type "Buy groceries" and press Enter
```

### What Happens Behind the Scenes

```
╔════════════════════════════════════════════════════════════════════╗
║  STAGE 1: I (INTENTION) - "Something needs to happen"             ║
╚════════════════════════════════════════════════════════════════════╝

    👤 USER ACTION: Press Enter
         ↓
    ┌────────────────────────────────────────┐
    │  Intention Pulse Created               │
    │  ─────────────────────────────────     │
    │  ID: "pulse_001"                       │
    │  Name: "add_todo"                      │
    │  TV: U (Undecided)                     │
    │  Data: ["Buy groceries", "day_2"]      │
    │  Time: 2025-11-27 14:30:00             │
    └────────────────────────────────────────┘
         ↓
    📨 Signal: "Please add this todo"


╔════════════════════════════════════════════════════════════════════╗
║  STAGE 2: O (OBJECT) - "Transform the message"                    ║
╚════════════════════════════════════════════════════════════════════╝

    Object receives Intention
         ↓
    ┌────────────────────────────────────────┐
    │  Mapping Function                      │
    │  ─────────────────────────────────     │
    │  Input: "add_todo" intention           │
    │    ↓                                    │
    │  Transform to DB format:                │
    │  - Extract text: "Buy groceries"       │
    │  - Extract day: Wednesday (2025-11-27) │
    │  - Prepare for storage                 │
    │    ↓                                    │
    │  Output: "insert_todo_db" intention    │
    └────────────────────────────────────────┘
         ↓
    ┌────────────────────────────────────────┐
    │  Transformed Intention                 │
    │  ─────────────────────────────────     │
    │  ID: "pulse_002"                       │
    │  Name: "insert_todo_db"                │
    │  TV: Y (Yes - ready to process)        │
    │  Data: ["2025-11-27", "Buy groceries"] │
    └────────────────────────────────────────┘
         ↓
    🔄 Signal: "Ready to save this data"


╔════════════════════════════════════════════════════════════════════╗
║  STAGE 3: I (INTENTION) - "Pass it along"                         ║
╚════════════════════════════════════════════════════════════════════╝

    Transformed signal travels to computation
         ↓
    📦 Carrying: Date + Todo text in DB format


╔════════════════════════════════════════════════════════════════════╗
║  STAGE 4: DN (DESIGN NODE) - "DO THE WORK!"                       ║
╚════════════════════════════════════════════════════════════════════╝

    Design Node receives Intention
         ↓
    ┌────────────────────────────────────────┐
    │  COMPUTATION HAPPENS HERE              │
    │  ═══════════════════════════════════   │
    │                                         │
    │  Step 1: Generate unique ID             │
    │    → id: "550e8400-..."                │
    │                                         │
    │  Step 2: Create TodoItem                │
    │    → text: "Buy groceries"              │
    │    → completed: false                   │
    │    → created_at: now()                  │
    │                                         │
    │  Step 3: Get day entry                  │
    │    → date: 2025-11-27 (Wednesday)      │
    │                                         │
    │  Step 4: Add todo to day                │
    │    → day.todos.push(new_todo)          │
    │                                         │
    │  Step 5: Save to JSON                   │
    │    → Write to data.json                 │
    │    → Disk write complete ✓              │
    └────────────────────────────────────────┘
         ↓
    💾 TODO CREATED & SAVED!


╔════════════════════════════════════════════════════════════════════╗
║  STAGE 5: I (INTENTION) - "Signal the result"                     ║
╚════════════════════════════════════════════════════════════════════╝

    Design Node creates result Intention
         ↓
    ┌────────────────────────────────────────┐
    │  Result Intention (IMMUTABLE)          │
    │  ─────────────────────────────────     │
    │  ID: "pulse_003"                       │
    │  Name: "todo_added"                    │
    │  TV: Y (Yes - success)                 │
    │  Data: [                               │
    │    "550e8400-...",                     │
    │    "Buy groceries",                    │
    │    "success"                           │
    │  ]                                     │
    └────────────────────────────────────────┘
         ↓
    ✅ Signal: "Todo added successfully"


╔════════════════════════════════════════════════════════════════════╗
║  STAGE 6: GL (GRID LOOKOUT) - "Show the result"                   ║
╚════════════════════════════════════════════════════════════════════╝

    Grid Lookout for Wednesday receives result
         ↓
    ┌────────────────────────────────────────┐
    │  UI Update                              │
    │  ─────────────────────────────────     │
    │  Get updated day data                   │
    │    ↓                                    │
    │  Convert to UI format                   │
    │    ↓                                    │
    │  Update Slint model                     │
    │    ↓                                    │
    │  Trigger re-render                      │
    └────────────────────────────────────────┘
         ↓
    🎨 SCREEN UPDATES!


╔════════════════════════════════════════════════════════════════════╗
║  STAGE 7: I (INTENTION) - "Ready for next"                        ║
╚════════════════════════════════════════════════════════════════════╝

    System creates "ready" Intention
         ↓
    ⏳ Waiting for next user action...
```

### What You See After

```
┌─────────────────────────────────────────────────────┐
│  Wednesday, Nov 27, 2025        [Add Todo Section]  │
│  ┌───────────────────────────────────────────────┐  │
│  │  [                      ] [➕]               │  │
│  └───────────────────────────────────────────────┘  │
│                                                      │
│  Existing todos:                                     │
│  ☐ Review project                                   │
│  ☐ Call dentist                                     │
│  ☐ Buy groceries          ← NEW! Just appeared     │
└─────────────────────────────────────────────────────┘
```

---

## 📱 Example 2: Checking Off a Todo

### Visual Flow

```
USER CLICKS CHECKBOX
        ↓
┌───────────────────┐
│   I: Intention    │  "toggle_todo" signal
│   TV: U           │  Data: todo_id, day_index
└────────┬──────────┘
         ↓
┌───────────────────┐
│   O: Object       │  Map to: "update_completion"
│   TV: Y           │  Data: todo_id, new_state
└────────┬──────────┘
         ↓
┌───────────────────┐
│   I: Intention    │  Pass mapped signal
└────────┬──────────┘
         ↓
┌───────────────────┐
│   DN: Design Node │  COMPUTE:
│                   │  1. Find todo by ID
│   ⚙️  WORK HERE   │  2. Flip completed flag
│                   │  3. Save to JSON
└────────┬──────────┘
         ↓
┌───────────────────┐
│   I: Intention    │  "todo_toggled" result
│   TV: Y           │  Data: todo_id, "success"
└────────┬──────────┘
         ↓
┌───────────────────┐
│   GL: Grid Look   │  Update display:
│   🎨 DISPLAY      │  - Show as checked
│                   │  - Gray out text
└────────┬──────────┘
         ↓
┌───────────────────┐
│   I: Intention    │  Ready for next
└───────────────────┘
```

---

## 🗺️ The Complete System Map

### The 7-Day Grid

```
┌══════════════════════════════════════════════════════════════════════┐
║                        7 DAYS GRID                                    ║
║                   ONE CELL = ONE PULSE                               ║
╚══════════════════════════════════════════════════════════════════════╝

    Grid Layer (What you see):
    ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
    │ MON │ TUE │ WED │ THU │ FRI │ SAT │ SUN │
    │ 🌸  │ 🦋  │ 🌻  │ 🦜  │ 🌺  │ 🦩  │ 🌞  │
    │Cell0│Cell1│Cell2│Cell3│Cell4│Cell5│Cell6│
    └─────┴─────┴─────┴─────┴─────┴─────┴─────┘
       ↕     ↕     ↕     ↕     ↕     ↕     ↕
    Grid Lookout Layer (Display binding):
    ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
    │ GL0 │ GL1 │ GL2 │ GL3 │ GL4 │ GL5 │ GL6 │
    └─────┴─────┴─────┴─────┴─────┴─────┴─────┘
       ↕     ↕     ↕     ↕     ↕     ↕     ↕
    Field Pulse Layer (Data):
    ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
    │day_0│day_1│day_2│day_3│day_4│day_5│day_6│
    └─────┴─────┴─────┴─────┴─────┴─────┴─────┘
       ↕     ↕     ↕     ↕     ↕     ↕     ↕
    Data Layer (Storage):
    ┌─────┬─────┬─────┬─────┬─────┬─────┬─────┐
    │JSON │JSON │JSON │JSON │JSON │JSON │JSON │
    └─────┴─────┴─────┴─────┴─────┴─────┴─────┘

    Each vertical stack = ONE complete path from display to storage
```

### Data Flow Through The System

```
╔════════════════════════════════════════════════════════════════════╗
║                    SYSTEM ARCHITECTURE                              ║
╚════════════════════════════════════════════════════════════════════╝

    ┌─────────────────────────────────────────────────┐
    │             USER INTERFACE (Slint)              │
    │  ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐ ┌────┐    │
    │  │Mon │ │Tue │ │Wed │ │Thu │ │Fri │ │Sat │    │
    │  │🌸  │ │🦋  │ │🌻  │ │🦜  │ │🌺  │ │🦩  │    │
    │  └────┘ └────┘ └────┘ └────┘ └────┘ └────┘    │
    └──────────────────┬──────────────────────────────┘
                       ↓ Callbacks (I)
    ┌──────────────────────────────────────────────────┐
    │         INTENTION SPACE LAYER                     │
    │                                                   │
    │  ┌──────────────────────────────────────────┐   │
    │  │  Space Loop (Orchestrator)                │   │
    │  │  ├─ CPUX 1: Day Management               │   │
    │  │  ├─ CPUX 2: Todo Operations              │   │
    │  │  └─ CPUX 3: Note Operations              │   │
    │  └──────────────────────────────────────────┘   │
    │                                                   │
    │  ┌──────────────────────────────────────────┐   │
    │  │  Network Field (Pulse Storage)            │   │
    │  │  ├─ day_0, day_1, ... day_6              │   │
    │  │  ├─ add_todo, toggle_todo, remove_todo   │   │
    │  │  └─ update_note, load_day               │   │
    │  └──────────────────────────────────────────┘   │
    │                                                   │
    │  ┌──────┐  ┌──────┐  ┌──────┐  ┌──────┐        │
    │  │  I   │→ │  O   │→ │  DN  │→ │  GL  │        │
    │  └──────┘  └──────┘  └──────┘  └──────┘        │
    └──────────────────┬───────────────────────────────┘
                       ↓
    ┌──────────────────────────────────────────────────┐
    │         DATA STORAGE LAYER                        │
    │  ┌──────────────────────────────────────────┐   │
    │  │  SevenDaysStore                           │   │
    │  │  ├─ HashMap<Date, DayEntry>              │   │
    │  │  └─ JSON persistence                     │   │
    │  └──────────────────────────────────────────┘   │
    │                                                   │
    │  📄 data.json                                    │
    │  {                                                │
    │    "2025-11-27": {                               │
    │      "todos": [...],                             │
    │      "notes": [...]                              │
    │    }                                             │
    │  }                                               │
    └──────────────────────────────────────────────────┘
```

---

## 🔄 Compare: Traditional vs Intention Space

### Traditional App Flow

```
┌─────────────────────────────────────────────────────────┐
│  USER CLICKS BUTTON                                      │
└────────────────┬─────────────────────────────────────────┘
                 ↓
       ┌─────────────────────┐
       │  EVERYTHING HAPPENS  │
       │  IN ONE BIG BLOB     │
       │                      │
       │  - Transform data    │
       │  - Validate input    │
       │  - Save to database  │
       │  - Update UI         │
       │  - Log event         │
       │  - Send analytics    │
       │  - Check permissions │
       │  ... and more        │
       │                      │
       │  ALL MIXED TOGETHER! │
       └──────────┬───────────┘
                  ↓
       ┌─────────────────────┐
       │  SCREEN UPDATES      │
       │  (hopefully)         │
       └─────────────────────┘

Problems:
❌ Hard to debug (where did it break?)
❌ Hard to test (need to test everything)
❌ Hard to change (changing one thing breaks others)
❌ Hard to understand (what does this do?)
```

### Intention Space Flow

```
┌─────────────────────────────────────────────────────────┐
│  USER CLICKS BUTTON                                      │
└────────────────┬─────────────────────────────────────────┘
                 ↓
    ┌────────────────────┐
    │   I: Intention     │  Create signal
    │   Clear purpose    │
    └─────────┬──────────┘
              ↓
    ┌────────────────────┐
    │   O: Object        │  Transform signal
    │   ONE job          │
    └─────────┬──────────┘
              ↓
    ┌────────────────────┐
    │   I: Intention     │  Pass along
    │   Clean transfer   │
    └─────────┬──────────┘
              ↓
    ┌────────────────────┐
    │   DN: Design Node  │  Do computation
    │   ONLY place for   │  - Validate
    │   actual work      │  - Save database
    └─────────┬──────────┘  - Log
              ↓
    ┌────────────────────┐
    │   I: Intention     │  Carry result
    │   Immutable        │
    └─────────┬──────────┘
              ↓
    ┌────────────────────┐
    │   GL: Grid Lookout │  Display result
    │   ONE job          │
    └─────────┬──────────┘
              ↓
    ┌────────────────────┐
    │   I: Intention     │  Ready for next
    └────────────────────┘
              ↓
       ┌─────────────────────┐
       │  SCREEN UPDATED      │
       │  (guaranteed)        │
       └─────────────────────┘

Benefits:
✅ Easy to debug (pinpoint exact stage)
✅ Easy to test (test each stage separately)
✅ Easy to change (stages are independent)
✅ Easy to understand (each stage has clear purpose)
```

---

## 🎯 Key Visual Concepts

### 1. Field Pulses are Like Mail

```
┌─────────────────────────────┐
│  FROM: User Action          │
│  TO: Design Node            │
│  SUBJECT: Add Todo          │
│  ────────────────────────   │
│  Dear System,               │
│                             │
│  Please add this todo:      │
│  "Buy groceries"            │
│  to Wednesday.              │
│                             │
│  Thank you,                 │
│  User                       │
└─────────────────────────────┘

Properties:
✉️  Has sender and receiver
✉️  Carries a message
✉️  Can't be modified once sent
✉️  Has timestamp
✉️  Has status (Y/N/U)
```

### 2. Objects are Like Translators

```
    ENGLISH              TRANSLATOR              FRENCH
       ↓                     ↓                      ↓
  "Add Buy         →    [Translation]    →    "Ajouter acheter
   groceries"                                  l'épicerie"

    UI Format            Object Maps         Database Format
       ↓                     ↓                      ↓
  add_todo          →    [Mapping]       →    insert_todo_db
  "Buy groceries"        Function             date, text, id
  day_index: 2                                completed: false
```

### 3. Design Nodes are Like Chefs

```
    ORDER               CHEF                   MEAL
      ↓                  ↓                      ↓
  "Make pasta"   →   [Cooking]         →   🍝 Pasta
                     - Boil water
                     - Cook noodles
                     - Make sauce
                     - Combine
                     
    Intention         Design Node           Result
      ↓                  ↓                      ↓
  "Add todo"     →   [Computing]        →   ✅ Todo created
                     - Generate ID
                     - Create object
                     - Save to DB
                     - Return result
```

### 4. Grid Lookouts are Like Display Windows

```
    WHAT HAPPENED           DISPLAY WINDOW          WHAT YOU SEE
         ↓                       ↓                        ↓
    Todo added      →      [Grid Lookout]     →     ☐ Buy groceries
    (data change)          binds pulse to UI        appears on screen
                           updates display
```

---

## 🎬 Complete Animation Script

```
SCENE: User adds a todo

[Frame 1] User types "Buy groceries"
    │
    │  Input box: [Buy groceries_]
    │
    
[Frame 2] User presses Enter
    │
    │  🎬 ACTION: Keyboard event triggered
    │
    
[Frame 3] I - Intention created
    │
    │  📦 Pulse: { name: "add_todo", data: [...] }
    │
    
[Frame 4] O - Object receives and transforms
    │
    │  🔄 Mapping: "add_todo" → "insert_todo_db"
    │
    
[Frame 5] I - Transformed intention travels
    │
    │  📨 Signal moving through system
    │
    
[Frame 6] DN - Design Node computes
    │
    │  ⚙️  Creating todo object...
    │  💾 Saving to JSON...
    │  ✅ Done!
    │
    
[Frame 7] I - Result intention created
    │
    │  📬 Result: { status: "success", id: "abc123" }
    │
    
[Frame 8] GL - Grid Lookout updates
    │
    │  🎨 Rendering new todo...
    │
    
[Frame 9] Todo appears on screen
    │
    │  ☐ Buy groceries  ← NEW!
    │
    
[Frame 10] I - System ready for next
    │
    │  ⏳ Waiting for user...
```

---

## 📚 Quick Reference

### The 7 Stages (Simplified)

```
1. I  → 💭 "User wants something"
2. O  → 🔄 "Transform it"
3. I  → 📨 "Pass it along"
4. DN → ⚙️  "DO THE WORK"
5. I  → ✅ "Here's the result"
6. GL → 🎨 "Show it"
7. I  → ⏳ "Ready for next"
```

### Visual Symbols Used

```
I   = 📦 Intention / Signal
O   = 🔄 Object / Transformer
DN  = ⚙️  Design Node / Computer
GL  = 🎨 Grid Lookout / Display
TV  = 🚦 Trivalent (Y/N/U)
```

---

**This is how Intention Space works visually!** 🎉

Every action in the app follows these exact stages, every single time.

**Welcome to physics-based visual thinking!**
