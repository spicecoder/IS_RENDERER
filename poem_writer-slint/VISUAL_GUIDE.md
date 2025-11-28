# 🎨 Poem Writer - Visual Guide

## UI Mockups and Flow Diagrams

---

## 📱 Main Application Interface

### Desktop/Android Screen Layout

```
┌─────────────────────────────────────────────────────────────────┐
│  ✍️ Poem Writer                         ◀ [Page 1 / 3] ▶ [+]   │ ← Header
├─────────────────────────────────────────────────────────────────┤
│                             │                                    │
│  📝 Your Poem              │  💭 Notes & Emotions               │
│  Compose your verses here  │  Capture feelings, ideas, themes   │
│ ────────────────────────── │ ────────────────────────────────── │
│                             │                                    │
│  In silicon dreams we code, │  Theme: Technology & creativity    │
│  Intentions flow like       │                                    │
│  rivers,                    │  Emotion: Wonder, curiosity        │
│  Pulses dance through       │                                    │
│  nodes,                     │  Imagery:                          │
│  Computing delivers.        │  - Rivers (flow)                   │
│                             │  - Dancing (movement)              │
│  [Cursor here...]           │  - Silicon (technology)            │
│                             │                                    │
│                             │  Rhyme pattern: ABCB               │
│                             │                                    │
│                             │  Word choices:                     │
│                             │  - "Intentions" (key concept)      │
│                             │  - "Pulses" (technical term)       │
│                             │                                    │
│                             │  Next lines ideas:                 │
│                             │  - Physics metaphor                │
│                             │  - Grid/cell imagery               │
│                             │                                    │
├─────────────────────────────┴────────────────────────────────────┤
│          [💾 Save]    [📤 Export]    [🎨 Theme]                 │ ← Footer
└─────────────────────────────────────────────────────────────────┘
```

---

## 🔄 User Interactions

### Typing in Poem Editor

```
User types "Hello"
        ↓
┌─────────────────────┐
│  Keyboard Event     │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  Slint UI Callback  │
│  on_poem_text_      │
│  changed()          │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  CPUX Method        │
│  update_poem_text() │
└──────────┬──────────┘
           ↓
     [I-O-I-DN-I-GL-I Flow]
           ↓
┌─────────────────────┐
│  State Updated      │
│  PoemPage.poem_text │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  UI Refreshes       │
│  Text appears       │
└─────────────────────┘
```

### Adding New Page

```
User clicks [+]
        ↓
┌─────────────────────┐
│  Button Click       │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  on_add_page()      │
│  callback           │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  CPUX.add_page()    │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  Create PoemPage    │
│  page_number = 4    │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│  Update UI          │
│  Page 4 / 4         │
│  Blank editors      │
└─────────────────────┘
```

### Navigation Between Pages

```
         [◀]               [▶]
          ↓                 ↓
    prev_page()        next_page()
          ↓                 ↓
    ┌──────────┐      ┌──────────┐
    │ Page 1   │      │ Page 3   │
    │ Roses... │      │ Stars... │
    └──────────┘      └──────────┘
          ↑                 ↑
          └─── Current ─────┘
                Page 2
           ┌──────────┐
           │ Rivers...│
           └──────────┘
```

---

## 🧩 Component Breakdown

### Header Bar Components

```
┌─────────────────────────────────────────────────────────────────┐
│ [Icon] ✍️ Poem Writer        [◀] [Page 2/5] [▶] [+]           │
│        └─Title                └─Nav Controls                     │
│                                   │    │     │   │               │
│                                   │    │     │   └─Add Page      │
│                                   │    │     └─Next Page         │
│                                   │    └─Page Indicator          │
│                                   └─Prev Page                    │
└─────────────────────────────────────────────────────────────────┘
```

### Split Content Area

```
┌──────────────────────────┬──────────────────────────┐
│  LEFT HALF (50%)         │  RIGHT HALF (50%)        │
│                          │                          │
│  ┌────────────────────┐ │ ┌────────────────────┐  │
│  │ Section Header     │ │ │ Section Header     │  │
│  │ 📝 Your Poem       │ │ │ 💭 Notes & Emotions│  │
│  └────────────────────┘ │ └────────────────────┘  │
│                          │                          │
│  ┌────────────────────┐ │ ┌────────────────────┐  │
│  │                    │ │ │                    │  │
│  │  Scrollable Text   │ │ │  Scrollable Text   │  │
│  │  Editor            │ │ │  Editor            │  │
│  │                    │ │ │                    │  │
│  │  [Poem content]    │ │ │  [Notes content]   │  │
│  │                    │ │ │                    │  │
│  │                    │ │ │                    │  │
│  └────────────────────┘ │ └────────────────────┘  │
│                          │                          │
└──────────────────────────┴──────────────────────────┘
```

### Footer Action Bar

```
┌─────────────────────────────────────────────────────────────────┐
│  [        💾 Save        ] [      📤 Export      ] [🎨 Theme]  │
│   └─Primary Action          └─Secondary Action     └─Utility    │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🎨 Color Theme

### Current Theme (Dark)

```
┌─────────────────────────────────────────────────────────┐
│  Component          Color       Hex       Use          │
├─────────────────────────────────────────────────────────┤
│  Background         Dark Navy   #1a1a2e   Main bg      │
│  Surface            Navy        #16213e   Cards        │
│  Surface Light      Blue Navy   #0f3460   Accents      │
│  Primary            Red         #e94560   CTA          │
│  Secondary          Purple      #533483   Alt actions  │
│  Text Primary       Light Gray  #eaeaea   Main text    │
│  Text Secondary     Gray        #a0a0a0   Subtle text  │
│  Divider            Dark Gray   #2d2d44   Separators   │
│  Accent             Orange      #f39c12   Highlights   │
└─────────────────────────────────────────────────────────┘
```

### Visual Color Palette

```
Background:    ████████████  #1a1a2e
Surface:       ████████████  #16213e
Primary:       ████████████  #e94560
Text:          ████████████  #eaeaea
```

---

## 📊 State Diagram

### Application State Flow

```
          ┌──────────────┐
          │   INITIAL    │
          │   STATE      │
          │  Page 1, 0/0 │
          └──────┬───────┘
                 │
                 ↓
          ┌──────────────┐
          │   EDITING    │
          │  User types  │
          └──────┬───────┘
                 │
        ┌────────┼────────┐
        ↓        ↓        ↓
   ┌────────┐ ┌────────┐ ┌────────┐
   │ POEM   │ │ NOTES  │ │  NAV   │
   │ TYPING │ │ TYPING │ │ ACTION │
   └───┬────┘ └───┬────┘ └───┬────┘
       │          │          │
       └──────────┴──────────┘
                 │
                 ↓
          ┌──────────────┐
          │    SAVED     │
          │  Auto-save   │
          └──────┬───────┘
                 │
                 ↓
          ┌──────────────┐
          │   DISPLAY    │
          │  Show result │
          └──────────────┘
```

---

## 🔄 Data Flow Visualization

### Complete I-O-I-DN-I-O-I-DN-I-GL-I Path

```
┌─────────────────────────────────────────────────────────────────┐
│                        USER ACTION                              │
│                    (Types "Hello")                              │
└───────────────────────────┬─────────────────────────────────────┘
                            ↓
                   ┌────────────────┐
                   │  INTENTION 1   │  Immutable data created
                   │  user_input_   │  TV: U (Undecided)
                   │  poem          │  response: ["Hello"]
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │   OBJECT 1     │  Static routing
                   │  route_to_     │  Pure function
                   │  update        │  No side effects
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │  INTENTION 2   │  Routed data
                   │  ready_for_    │  TV: Y (Ready)
                   │  update        │  response: ["Hello"]
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │ DESIGN NODE 1  │  ** COMPUTATION **
                   │  validate_poem │  - Trim whitespace
                   │                │  - Check length
                   │  [COMPUTE]     │  - Validate format
                   │                │  - Return result/error
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │  INTENTION 3   │  Validation result
                   │  validation_   │  TV: Y/N (depends)
                   │  result        │  response: ["Hello" or "error"]
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │   OBJECT 2     │  ** REFLECTS DIFFERENT **
                   │  reflect_val_  │  ** INTENTIONS **
                   │  result        │  Examines result
                   │                │  Creates different intentions
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │  INTENTION 4   │  Different intentions!
                   │ "validation_   │  If success: TV: Y
                   │  success" OR   │  If error: TV: N
                   │ "validation_   │  
                   │  error"        │  
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │ DESIGN NODE 2  │  ** COMPUTATION **
                   │ update_poem    │  (Only if TV: Y)
                   │                │  - Update state
                   │  [COMPUTE]     │  - Save to DB
                   │                │  - Mark timestamp
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │  INTENTION 5   │  Result data
                   │  update_poem_  │  TV: Y (Success)
                   │  result        │  response: ["Hello"]
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │ NETWORK FIELD  │  History storage
                   │  Store pulse   │  For replay/undo
                   │  [STORAGE]     │  Debugging
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │ GRID LOOKOUT   │  UI binding
                   │  Bind to cell  │  ONE CELL = ONE PULSE
                   │  [DISPLAY]     │  Automatic update
                   └────────┬───────┘
                            ↓
                   ┌────────────────┐
                   │  INTENTION 6   │  Display intention
                   │  ui_update     │  TV: Y (Success)
                   │                │  Visual refresh
                   └────────┬───────┘
                            ↓
┌─────────────────────────────────────────────────────────────────┐
│                     VISUAL UPDATE                               │
│                 (User sees "Hello")                             │
└─────────────────────────────────────────────────────────────────┘
```

**Key Concept:** Object 2 reflects DIFFERENT intentions (validation_success vs validation_error) based on the computation result from Design Node 1. This enables branching in the flow while maintaining the pattern.

---

## 🎯 Pulse Timeline

### Temporal View of Field Pulses

```
Time ──────────────────────────────────────────────────────────→

T0:   [user_input_poem]         TV: U
         ↓
T1:   [validated_poem]           TV: Y
         ↓
T2:   [update_poem_result]       TV: Y
         ↓
T3:   [ui_update]                TV: Y
         ↓
T4:   [user_input_poem]          TV: U  (Next keystroke)
         ↓
T5:   [validated_poem]           TV: Y
         ↓
...

Network Field stores ALL pulses:
┌──────────────────────────────────────┐
│ Pulse History                        │
├──────────────────────────────────────┤
│ 1. user_input_poem (T0)             │
│ 2. validated_poem (T1)              │
│ 3. update_poem_result (T2)          │
│ 4. ui_update (T3)                   │
│ 5. user_input_poem (T4)             │
│ 6. validated_poem (T5)              │
│ ...                                  │
└──────────────────────────────────────┘
```

---

## 📏 Layout Dimensions

### Header Bar (70px height)

```
┌─────────────────────────────────────────┐
│  ↕ 16px padding                         │
│  ┌─────────────────────────────────┐   │
│  │ App Title + Nav (38px)          │   │ ← 38px content
│  └─────────────────────────────────┘   │
│  ↕ 16px padding                         │
└─────────────────────────────────────────┘
Total: 70px
```

### Content Area (flexible)

```
←──────── 50% ────────→←──────── 50% ────────→
┌───────────────────────┬───────────────────────┐
│ Poem Section          │ Notes Section         │
│ ↕ Flexible height     │ ↕ Flexible height     │
│                       │                       │
│ [Scrollable]          │ [Scrollable]          │
│                       │                       │
└───────────────────────┴───────────────────────┘
```

### Footer Bar (60px height)

```
┌─────────────────────────────────────────┐
│  ↕ 12px padding                         │
│  ┌─────────────────────────────────┐   │
│  │ Action Buttons (36px)           │   │ ← 36px buttons
│  └─────────────────────────────────┘   │
│  ↕ 12px padding                         │
└─────────────────────────────────────────┘
Total: 60px
```

---

## 🖱️ Interaction Zones

### Touch/Click Targets

```
┌─────────────────────────────────────────────────────────────────┐
│  [44x44] [120x40] [44x44] [44x44]    ← Minimum 44px touch size │
│     ◀      Page 1/3    ▶      +                                 │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────────┬──────────────────────────┐
│  ┌────────────────────┐ │ ┌────────────────────┐  │
│  │ Full area          │ │ │ Full area          │  │ ← Entire area
│  │ clickable          │ │ │ clickable          │  │   is editable
│  │ (Text Editor)      │ │ │ (Text Editor)      │  │
│  └────────────────────┘ │ └────────────────────┘  │
└──────────────────────────┴──────────────────────────┘

┌─────────────────────────────────────────────────────────────────┐
│  [Full Width Button] [Full Width] [Compact]    ← 48px min height│
│       💾 Save            📤 Export    🎨 Theme                   │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🎭 States and Animations

### Button States

```
Normal:     [  💾 Save  ]  ← Default appearance
Hover:      [  💾 Save  ]  ← Slight highlight (desktop)
Pressed:    [  💾 Save  ]  ← Darker background
Disabled:   [  💾 Save  ]  ← Grayed out (if applicable)
```

### Page Transition

```
Page 1 (visible)           Page 2 (hidden)
┌───────────────┐         ┌───────────────┐
│ Content...    │  ────→  │ Content...    │
└───────────────┘         └───────────────┘
     Fade out                  Fade in
     (0.2s)                    (0.2s)
```

---

## 📱 Responsive Breakpoints

### Phone (< 480px)

```
┌─────────────────┐
│  Header         │
├─────────────────┤
│                 │
│  Poem (100%)    │  ← Stacked layout
│                 │
├─────────────────┤
│                 │
│  Notes (100%)   │
│                 │
├─────────────────┤
│  Footer         │
└─────────────────┘
```

### Tablet (480px - 768px)

```
┌─────────────────────────────┐
│  Header                     │
├─────────────────────────────┤
│           │                 │
│  Poem     │  Notes          │  ← Side by side
│  (50%)    │  (50%)          │     (original)
│           │                 │
├─────────────────────────────┤
│  Footer                     │
└─────────────────────────────┘
```

### Desktop (> 768px)

```
┌──────────────────────────────────────┐
│  Header                              │
├──────────────────────────────────────┤
│              │                       │
│  Poem        │  Notes                │  ← Optimized
│  (40%)       │  (60%)                │     ratios
│              │                       │
├──────────────────────────────────────┤
│  Footer                              │
└──────────────────────────────────────┘
```

---

## 🎨 Typography Scale

### Text Hierarchy

```
┌─────────────────────────────────────────┐
│  Title (20px, Bold)                     │  ← App title
│  ✍️ Poem Writer                         │
├─────────────────────────────────────────┤
│  Subtitle (11px, Italic)                │  ← Secondary info
│  Intention Space Physics                │
├─────────────────────────────────────────┤
│  Section Header (18px, Bold)            │  ← Section titles
│  📝 Your Poem                           │
├─────────────────────────────────────────┤
│  Body Text (16px, Normal)               │  ← Poem content
│  In silicon dreams we code,             │     (Serif font)
│  Intentions flow like rivers...         │
├─────────────────────────────────────────┤
│  Notes Text (14px, Normal)              │  ← Notes content
│  Theme: Technology & creativity         │     (Sans-serif)
├─────────────────────────────────────────┤
│  Button Text (16px, Semibold)           │  ← Action buttons
│  💾 Save                                │
├─────────────────────────────────────────┤
│  Caption (12px, Normal)                 │  ← Small labels
│  Page 1 / 3                             │
└─────────────────────────────────────────┘
```

---

## 🔍 Zoom Levels

### Standard View (100%)

```
┌────────────────────────────────────┐
│  Normal text size                  │
│  Comfortable reading               │
│  16px poem, 14px notes             │
└────────────────────────────────────┘
```

### Zoomed In (125%)

```
┌────────────────────────────────────┐
│  Larger text                       │
│  Better accessibility              │
│  20px poem, 17.5px notes           │
└────────────────────────────────────┘
```

### Zoomed Out (75%)

```
┌────────────────────────────────────┐
│  Smaller text, more content visible│
│  12px poem, 10.5px notes           │
└────────────────────────────────────┘
```

---

## 📐 Grid System (Internal)

### UI Grid (ONE CELL = ONE PULSE)

```
┌─────────────────────────────────────────┐
│  Grid Cell 1         Grid Cell 2        │
│  (Poem Editor)       (Notes Editor)     │
│       ↕                   ↕             │
│    Pulse 1             Pulse 2          │
│  "poem_text"        "notes_text"        │
│       ↕                   ↕             │
│  PoemPage.          PoemPage.           │
│  poem_text          notes_text          │
└─────────────────────────────────────────┘

Each cell binds to exactly ONE pulse
No conflicts, clear ownership
```

---

## 🎯 Visual Summary

```
┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
┃  ✍️ POEM WRITER                          ┃
┃  Intention Space Android App             ┃
┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

Features:
├─ 📝 Split-screen poem + notes
├─ 📄 Multi-page management
├─ 💾 Auto-save
├─ 🎨 Beautiful dark theme
├─ 🔄 I-O-I-DN-I-GL-I physics
└─ 📱 Android native

Tech Stack:
├─ 🦀 Rust (logic)
├─ 🎨 Slint (UI)
├─ 🤖 Android NDK
└─ ⚙️ Native Activity

Architecture:
├─ Field Pulses (immutable data)
├─ Design Nodes (computation)
├─ Objects (transformations)
└─ Grid Lookouts (UI binding)
```

---

**This visual guide provides a complete picture of the Poem Writer interface, interactions, and architecture.**

**Use these mockups to understand layout, flow, and design decisions.**

✨ **Ready to build beautiful poetry apps!** ✍️
