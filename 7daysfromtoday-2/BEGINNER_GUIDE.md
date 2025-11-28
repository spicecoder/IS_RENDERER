# Understanding Intention Space Physics in "7 Days from Today"

## 🎯 For Complete Beginners

Think of Intention Space as a **new way to organize how apps work** - like how physics describes how the real world works, but for software.

## 🌟 The Simple Version

### Traditional App (What Most Apps Do)

```
You click button → Code runs → Screen updates
```

**Problem**: Everything is mixed together. Hard to understand, debug, or improve.

### Intention Space App (What We Do)

```
You click button → Signal created → Transformed → Computed → Displayed → Ready for next click
```

**Benefit**: Clear stages. Each stage has ONE job. Easy to understand and fix.

## 🔤 Breaking Down I-O-I-DN-I-GL-I

Let's use a **real example** from the app: Adding a todo.

### What Happens When You Type "Buy groceries" and Press Enter?

```
┌─────────────────────────────────────────────────────────────────┐
│                     THE FULL JOURNEY                             │
│                  I-O-I-DN-I-GL-I Pattern                        │
└─────────────────────────────────────────────────────────────────┘
```

### **Step 1: I (Intention)**
**"Something needs to happen"**

```
You press Enter
    ↓
App creates an "Intention Pulse"
    ↓
Pulse says: "User wants to add 'Buy groceries' to Wednesday"
```

**What is it?** A signal carrying:
- What happened: "add_todo"
- The data: "Buy groceries", "Wednesday"
- Status: "Undecided" (not done yet)

**Real code:**
```rust
FieldPulse {
    name: "add_todo",
    tv: U,  // Undecided - not processed yet
    response: ["Buy groceries", "day_index:2"]
}
```

**Think of it like:** Writing a note that says "Please add this task"

---

### **Step 2: O (Object)**
**"Transform the signal"**

```
Intention arrives
    ↓
Object looks at it and says: "This needs to become a data operation"
    ↓
Object transforms it to: "Insert todo into Wednesday's list"
```

**What is it?** A transformer that takes one format and converts to another.

**Real code:**
```rust
// Object's mapping function
fn map_add_todo(input: &FieldPulse) -> FieldPulse {
    // Transform "add_todo" intention into database format
    FieldPulse {
        name: "insert_todo_db",
        tv: Y,  // Yes - ready to process
        response: ["wednesday_2025_11_27", "Buy groceries"]
    }
}
```

**Think of it like:** A translator converting English to French

---

### **Step 3: I (Intention)**
**"Pass the transformed signal"**

```
Object finishes transforming
    ↓
Creates new Intention with transformed data
    ↓
Sends it to the next stage
```

**What is it?** Another signal, but now in the right format for processing.

**Think of it like:** Handing the translated note to someone who speaks French

---

### **Step 4: DN (Design Node)**
**"Actually DO the work"**

```
Intention arrives at Design Node
    ↓
Design Node says: "Time to compute!"
    ↓
Design Node CREATES the todo:
    - Generates unique ID
    - Adds timestamp
    - Sets completed = false
    - Saves to Wednesday's list
    - Saves to JSON file
```

**What is it?** The ONLY place where real work happens. Computation is LOCAL here.

**Real code:**
```rust
// Design Node's compute function
fn compute_add_todo(inputs: &[FieldPulse]) -> Vec<String> {
    // Extract data
    let date = inputs[0].response[0];  // "wednesday_2025_11_27"
    let text = inputs[0].response[1];  // "Buy groceries"
    
    // COMPUTE: Create the todo
    let todo = TodoItem::new(text);
    
    // COMPUTE: Save it
    store.get_or_create_entry(date).add_todo(todo);
    store.save();
    
    // Return result
    vec![todo.id, "success"]
}
```

**Think of it like:** The chef actually cooking the meal (not just reading the recipe)

---

### **Step 5: I (Intention)**
**"Signal the result"**

```
Design Node finishes
    ↓
Creates Intention with result
    ↓
Pulse says: "Todo added successfully! Here's the ID: abc123"
```

**What is it?** A signal carrying the RESULT (immutable - can't be changed)

**Real code:**
```rust
FieldPulse {
    name: "todo_added",
    tv: Y,  // Yes - operation succeeded
    response: ["abc123", "Buy groceries", "success"]
}
```

**Think of it like:** Chef saying "Your order is ready!"

---

### **Step 6: GL (Grid Lookout)**
**"Show it to the user"**

```
Intention with result arrives
    ↓
Grid Lookout for Wednesday updates
    ↓
You SEE the new todo appear in the list
```

**What is it?** The display layer - shows what happened.

**Real code:**
```rust
// Grid Lookout binds pulse to UI
grid_lookout.bind_pulse("todo_added");
    ↓
// Slint UI updates automatically
ui.set_days(updated_days);  // Wednesday now has new todo
```

**Think of it like:** The waiter bringing your food to the table

---

### **Step 7: I (Intention)**
**"Ready for next action"**

```
UI updated
    ↓
System creates "ready" intention
    ↓
Waiting for your next click/action
```

**What is it?** The cycle completes, ready to start again.

**Think of it like:** Waiter asking "Anything else?"

---

## 🎬 The Complete Flow (Visual)

```
YOU TYPE & PRESS ENTER
        ↓
    ┌───────────────────┐
    │  I: Intention     │  "add_todo" signal created
    │  "Add Buy groc"   │
    └────────┬──────────┘
             ↓
    ┌───────────────────┐
    │  O: Object        │  Transform to database format
    │  Maps signal      │
    └────────┬──────────┘
             ↓
    ┌───────────────────┐
    │  I: Intention     │  Pass transformed signal
    │  "insert_todo_db" │
    └────────┬──────────┘
             ↓
    ┌───────────────────┐
    │  DN: Design Node  │  COMPUTE: Create todo, save to file
    │  Actually creates │  (Only place work happens!)
    └────────┬──────────┘
             ↓
    ┌───────────────────┐
    │  I: Intention     │  Signal with result
    │  "todo_added"     │
    └────────┬──────────┘
             ↓
    ┌───────────────────┐
    │  GL: Grid Lookout │  Update Wednesday's display
    │  Shows new todo   │
    └────────┬──────────┘
             ↓
    ┌───────────────────┐
    │  I: Intention     │  Ready for next action
    │  "ready"          │
    └───────────────────┘
             ↓
    WAITING FOR YOUR NEXT CLICK
```

## 🔍 Why Is This Better?

### Traditional Approach
```javascript
// Everything mixed together
function addTodo(text, day) {
    // Transform data? Compute? Display? All in one place!
    let id = generateId();
    database.insert(id, text, day);
    updateUI(day);
    setupNextAction();
}
```

**Problems:**
- ❌ Hard to test (everything coupled)
- ❌ Hard to debug (where did it break?)
- ❌ Hard to change (touching one thing breaks others)
- ❌ Hard to understand (what does this function do?)

### Intention Space Approach
```rust
// Each stage has ONE job
I  → Signal created         (just a message)
O  → Signal transformed     (just mapping)
I  → Signal passed          (just transfer)
DN → Work done              (just computation)
I  → Result signaled        (just the result)
GL → Display updated        (just showing)
I  → Ready for next         (just waiting)
```

**Benefits:**
- ✅ Easy to test (test each stage separately)
- ✅ Easy to debug (see which stage failed)
- ✅ Easy to change (change one stage, others unaffected)
- ✅ Easy to understand (each stage has clear purpose)

## 📊 Real Examples in the App

### Example 1: Toggle Todo Completion

```
Click checkbox
    ↓
I: "toggle_todo" intention
    ↓
O: Map to database update
    ↓
I: Pass mapped signal
    ↓
DN: COMPUTE - flip completed flag, save JSON
    ↓
I: "todo_toggled" result
    ↓
GL: Update display (show checked/unchecked)
    ↓
I: Ready for next
```

### Example 2: Write Note

```
Type in note area
    ↓
I: "update_note" intention
    ↓
O: Map to note update
    ↓
I: Pass mapped signal
    ↓
DN: COMPUTE - save note content, update timestamp
    ↓
I: "note_updated" result
    ↓
GL: Show "Auto-saved" message
    ↓
I: Ready for next
```

### Example 3: Delete Todo

```
Click trash icon
    ↓
I: "remove_todo" intention
    ↓
O: Map to delete operation
    ↓
I: Pass mapped signal
    ↓
DN: COMPUTE - remove from list, save JSON
    ↓
I: "todo_removed" result
    ↓
GL: Remove from display
    ↓
I: Ready for next
```

## 🎯 Key Principles

### 1. **Immutable Responses**
Once a result is created, it NEVER changes.

```rust
// This pulse is created once and never modified
let pulse = FieldPulse {
    name: "todo_added",
    response: ["abc123", "success"]
};
// Can't change pulse.response later!
```

**Why?** Like a receipt - once printed, it's a record of what happened.

### 2. **Computation Locality**
All real work happens ONLY in Design Nodes.

```rust
// Intention: just carries data
let intention = create_signal("add_todo", data);

// Object: just transforms
let transformed = map_to_db_format(intention);

// Design Node: ONLY place that actually does work
let result = actually_create_and_save_todo(transformed);
```

**Why?** Like a kitchen - all cooking happens in the kitchen, not in the dining room.

### 3. **Static Mapping**
Objects always transform the same way.

```rust
// Object always maps "add_todo" → "insert_todo_db"
// Never changes based on time of day, user, etc.
fn map_add_todo(input) -> output {
    // Same input always gives same output
    transform(input)
}
```

**Why?** Like a translator - "hello" always becomes "bonjour", predictably.

### 4. **ONE CELL = ONE PULSE**
Each grid cell (day card) is bound to ONE pulse.

```
Monday Card    = Grid Lookout 0 = Pulse "day_0"
Tuesday Card   = Grid Lookout 1 = Pulse "day_1"
Wednesday Card = Grid Lookout 2 = Pulse "day_2"
...
```

**Why?** Clear relationship. Each day cell displays exactly one data pulse.

## 🧪 How to Verify It's Working

### 1. Add a Todo
- **I**: Signal created when you press Enter
- **O**: Transformed to database operation
- **DN**: Todo actually created and saved
- **GL**: Appears in the list immediately
- **Check**: Look in JSON file - it's there!

### 2. Toggle Completion
- **I**: Signal created when you click checkbox
- **O**: Mapped to update operation
- **DN**: Completion flag flipped
- **GL**: Todo appearance changes (grayed out)
- **Check**: JSON updated with completed=true

### 3. View Previous Weeks
- **I**: Signal when you click "Previous"
- **O**: Mapped to historical query
- **DN**: Database queried for past Wednesdays
- **GL**: Previous week cards displayed
- **Check**: See up to 7 previous weeks

## 💡 The Big Idea

**Traditional apps**: Everything is mixed up in a big bowl.

**Intention Space apps**: Everything flows through clear stages, like an assembly line.

```
Traditional:
┌─────────────────────────────────┐
│  EVERYTHING HAPPENS IN HERE     │
│  (hard to understand)            │
└─────────────────────────────────┘

Intention Space:
┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐
│  I   │ → │  O   │ → │  DN  │ → │  GL  │ → │  I   │
│Signal│   │ Map  │   │Compute│   │Display│   │Ready │
└──────┘   └──────┘   └──────┘   └──────┘   └──────┘
  (clear stages - easy to understand)
```

## 🎓 Summary for Newcomers

**Intention Space physics (I-O-I-DN-I-GL-I) means:**

1. **I (Intention)**: User does something → Signal created
2. **O (Object)**: Signal transformed to right format
3. **I (Intention)**: Transformed signal passed along
4. **DN (Design Node)**: Real work happens (COMPUTE!)
5. **I (Intention)**: Result carried forward
6. **GL (Grid Lookout)**: User sees the result
7. **I (Intention)**: Ready for next action

**Every user action follows this flow. Every. Single. Time.**

**Benefits:**
- Predictable behavior
- Easy to debug
- Easy to extend
- Easy to understand
- Professional quality

## 🚀 Try It Yourself

1. **Run the app**: `cargo run`
2. **Add a todo** and watch mentally:
   - "I created an intention"
   - "O is mapping it"
   - "DN is computing"
   - "GL is displaying"
3. **Open the code** and find:
   - Intentions in `main.rs` (callbacks)
   - Objects in `intention_space.rs`
   - Design Nodes in `intention_space.rs`
   - Grid Lookouts in `seven_days.slint`

## 🎯 The Bottom Line

**Intention Space isn't magic** - it's just a really good way to organize code so that:
- Each part has ONE clear job
- Data flows in ONE direction
- Changes are predictable
- Everything is testable

**It's physics for software** - like Newton's laws give structure to motion, I-O-I-DN-I-GL-I gives structure to computation.

---

**Now you understand Intention Space!** 🎉

Every time you use the app, you're seeing I-O-I-DN-I-GL-I in action.

**Welcome to physics-based application development!**
