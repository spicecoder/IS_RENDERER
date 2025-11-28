# 🔄 Objects Reflecting Different Intentions

## A Critical Intention Space Pattern

---

## 🎯 The Core Concept

**Objects map intentions based on payload examination, but NEVER change the payload itself.**

This is a fundamental pattern in Intention Space physics:
- **Objects** examine the payload and map to different intention NAMES
- **Design Nodes** are the ONLY components that can modify payload data
- **Objects** are pure routing/reflection functions with zero data transformation

---

## ⚠️ CRITICAL RULE: Payload Immutability in Objects

```rust
// ✅ CORRECT: Object examines but doesn't change payload
ObjectMapping::new("reflect_validation_result", |pulse| {
    let result = pulse.response.first().unwrap_or(&String::new());
    
    if result.starts_with("error_") {
        FieldPulse::new("validation_error")
            .with_response(pulse.response.clone())  // SAME payload!
            .with_tv(TV::N)
    } else {
        FieldPulse::new("validation_success")
            .with_response(pulse.response.clone())  // SAME payload!
            .with_tv(TV::Y)
    }
});

// ❌ WRONG: Object modifying payload
ObjectMapping::new("bad_example", |pulse| {
    let text = pulse.response[0].trim();  // NO! Modification!
    FieldPulse::new("modified")
        .with_response(vec![text.to_string()])  // Changed payload!
});
```

**The Principle:**
- **Objects** change: intention NAME, TV value
- **Objects** preserve: payload (response data)
- **Design Nodes** can change: everything including payload

---

## 🔍 The Problem

In traditional programming, you branch with if/else:

```rust
// Traditional approach
if validate(input) {
    update_state(input);
} else {
    handle_error();
}
```

But in Intention Space, we need to maintain the flow pattern while still enabling different execution paths.

---

## ✨ The Solution: Objects Reflect Different Intentions

Objects examine the output from Design Nodes and create **different intention names** based on the result:

```rust
// Intention Space approach
let object = ObjectMapping::new("reflect_validation_result", |pulse| {
    let result = pulse.response.first().unwrap_or(&String::new());
    
    if result.starts_with("error_") {
        // DIFFERENT intention for error
        FieldPulse::new("validation_error")
            .with_response(vec![result.clone()])
            .with_tv(TV::N)  // Failed
    } else {
        // DIFFERENT intention for success
        FieldPulse::new("validation_success")
            .with_response(vec![result.clone()])
            .with_tv(TV::Y)  // Succeeded
    }
});
```

---

---

## 🎨 Objects vs Design Nodes: Clear Separation of Concerns

This is THE most important distinction in Intention Space physics.

### Design Nodes (COMPUTATION) - Can Change Everything

```rust
DesignNode::new("validate_poem", |inputs| {
    let text = inputs[0].response[0].clone();
    
    // ✅ Can trim whitespace (modifies payload)
    let validated = text.trim().to_string();
    
    // ✅ Can check length (computation)
    if validated.len() > 10000 {
        // ✅ Can return different payload
        vec!["error_too_long".to_string()]
    } else if validated.is_empty() {
        vec!["error_empty".to_string()]
    } else {
        // ✅ Can return modified payload
        vec![validated]
    }
})
```

**Design Node Powers:**
- ✅ Validate data
- ✅ Transform data (trim, format, convert)
- ✅ Compute new values
- ✅ Perform I/O (database, files, network)
- ✅ Change payload completely
- ✅ Add/remove data
- ✅ Complex logic and algorithms

### Objects (ROUTING/MAPPING) - Cannot Change Payload

```rust
ObjectMapping::new("reflect_validation_result", |pulse| {
    let result = pulse.response.first().unwrap_or(&String::new());
    
    // ✅ Can READ payload
    if result.starts_with("error_") {
        // ✅ Can map to different intention names
        // ❗ MUST preserve payload exactly
        FieldPulse::new("validation_error")
            .with_response(pulse.response.clone())  // UNCHANGED!
            .with_tv(TV::N)
    } else {
        FieldPulse::new("validation_success")
            .with_response(pulse.response.clone())  // UNCHANGED!
            .with_tv(TV::Y)
    }
})
```

**Object Powers:**
- ✅ Examine/read payload
- ✅ Map to different intention names
- ✅ Set TV values (Y/N/U)
- ✅ Simple conditional logic (if/match)
- ❌ **NEVER modify payload**
- ❌ **NEVER do computation**
- ❌ **NEVER interact with I/O**
- ❌ **NEVER transform data**

### The Golden Rule

```
┌─────────────────────────────────────────────┐
│  If it changes DATA → Design Node          │
│  If it changes ROUTING → Object             │
└─────────────────────────────────────────────┘
```

---

## 📊 Complete Flow Example



### Poem Writer Validation Flow

```
User types "Hello"
        ↓
┌─────────────────────┐
│ INTENTION 1         │  Name: "user_input_poem"
│ TV: U               │  Payload: ["Hello"]
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ OBJECT 1            │  Function: route_to_validation
│ (Routing Only)      │  Examines: payload
│                     │  Changes: intention name, TV
│                     │  Payload: ["Hello"] ← UNCHANGED!
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ INTENTION 2         │  Name: "ready_for_validation"
│ TV: U               │  Payload: ["Hello"] ← SAME!
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ DESIGN NODE 1       │  Function: validate_poem
│ (COMPUTATION)       │  Logic: trim, check length
│                     │  CAN modify payload
│                     │  Result: "Hello" or "error_too_long"
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ INTENTION 3         │  Name: "validation_result"
│ TV: Y/N             │  Payload: ["Hello"] or ["error_too_long"]
│                     │  ← PAYLOAD CHANGED by DN
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ OBJECT 2            │  ** ROUTING PATTERN **
│ (Map Intentions)    │  Examines: payload content
│                     │  Creates: DIFFERENT intention names
│                     │  Payload: PRESERVED (clone)
│                     │  
│ If "Hello":         │  If "error_...":
│   Name: validation_ │    Name: validation_error
│         success     │    TV: N
│   TV: Y             │    Payload: ["error_..."] ← SAME!
│   Payload: ["Hello"]│
│         ← SAME!     │
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ INTENTION 4         │  Name: "validation_success" OR
│                     │        "validation_error"
│ Different names!    │  TV: Y or N
│ Same payload!       │  Payload: UNCHANGED from Intention 3
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ DESIGN NODE 2       │  Function: update_poem
│ (COMPUTATION)       │  Condition: Only if TV == Y
│                     │  Logic: Save to database
│                     │  CAN modify payload
└──────────┬──────────┘
           ↓
┌─────────────────────┐
│ INTENTION 5         │  Name: "update_poem_result"
│ TV: Y               │  Payload: May differ from Intention 4
└─────────────────────┘

KEY INSIGHT:
Objects (1 & 2): Payload never changes
Design Nodes (1 & 2): Payload CAN change
```

---

## 🎨 Real Code Implementation

### Design Node: Performs Validation

```rust
DesignNode::new("validate_poem", "Validate Poem Text", |inputs| {
    let text = inputs[0].response[0].clone();
    
    // COMPUTATION: Actual validation logic
    let validated = text.trim().to_string();
    
    if validated.len() > 10000 {
        // Return error indicator
        vec!["error_too_long".to_string()]
    } else if validated.is_empty() {
        vec!["error_empty".to_string()]
    } else {
        // Return validated text
        vec![validated]
    }
})
```

### Object: Reflects Different Intentions (PRESERVES PAYLOAD)

```rust
ObjectMapping::new("reflect_validation_result", |pulse| {
    let result = pulse.response.first().unwrap_or(&String::new());
    
    // KEY LOGIC: Examine payload and map to different intentions
    // CRITICAL: Payload remains UNCHANGED
    if result.starts_with("error_") {
        // Path 1: Error - create "validation_error" intention
        FieldPulse::new("validation_error")
            .with_response(pulse.response.clone())  // SAME payload!
            .with_tv(TV::N)  // N = Failed
    } else {
        // Path 2: Success - create "validation_success" intention
        FieldPulse::new("validation_success")
            .with_response(pulse.response.clone())  // SAME payload!
            .with_tv(TV::Y)  // Y = Success
    }
})
```

**What Changed:**
- Intention NAME: "validation_error" vs "validation_success"
- TV value: N vs Y

**What Did NOT Change:**
- Payload (response): Exactly the same as input pulse

### Conditional Execution Based on Intention

```rust
// Apply the object that reflects different intentions
let intention4 = object.apply(&intention3);

// Now we can branch based on the intention name or TV
if intention4.name == "validation_success" && intention4.tv == TV::Y {
    // Continue to next Design Node
    let dn2 = design_nodes.get("update_poem");
    let intention5 = dn2.execute(&[intention4]);
    
    // Success path
    collection.update_poem(text);
    network_field.push(intention5);
} else {
    // Error path - just store the error
    network_field.push(intention4);
    // Could also trigger error handling Design Node
}
```

---

## 🌟 Why This Matters

### Maintains Flow Pattern

```
✅ Still follows I-O-I-DN-I-O-I-DN-I-GL-I
✅ Clear stages at every step
✅ Testable at each point
✅ Debuggable with intention names
```

### Enables Branching

```
✅ Different intentions = different paths
✅ TV values indicate success/failure
✅ Can route to different Design Nodes
✅ Error handling is explicit
```

### Preserves Immutability

```
✅ Objects don't modify data
✅ They create NEW intentions
✅ Original pulses unchanged
✅ Full history in Network Field
```

---

## 📚 Multiple Reflection Patterns

### Pattern 1: Success/Error

```rust
if result.is_ok() {
    FieldPulse::new("operation_success")
        .with_tv(TV::Y)
} else {
    FieldPulse::new("operation_error")
        .with_tv(TV::N)
}
```

### Pattern 2: Multi-way Routing

```rust
match result.as_str() {
    "type_a" => FieldPulse::new("route_to_handler_a"),
    "type_b" => FieldPulse::new("route_to_handler_b"),
    "type_c" => FieldPulse::new("route_to_handler_c"),
    _ => FieldPulse::new("route_to_default"),
}
```

### Pattern 3: Priority Levels

```rust
let priority = calculate_priority(&result);

match priority {
    Priority::High => FieldPulse::new("high_priority_task"),
    Priority::Medium => FieldPulse::new("medium_priority_task"),
    Priority::Low => FieldPulse::new("low_priority_task"),
}
```

---

## 🎯 Design Principles

### 1. Objects Are Pure Functions

```rust
// ✅ GOOD: Pure mapping
|pulse| {
    if pulse.response[0] == "valid" {
        FieldPulse::new("success")
    } else {
        FieldPulse::new("error")
    }
}

// ❌ BAD: Side effects in Object
|pulse| {
    database.save(pulse.response[0]); // NO!
    FieldPulse::new("saved")
}
```

### 2. Design Nodes Do Computation

```rust
// ✅ GOOD: Computation in Design Node
DesignNode::new("validate", |inputs| {
    let text = &inputs[0].response[0];
    if text.len() > 10000 {
        vec!["error_too_long".to_string()]
    } else {
        vec![text.clone()]
    }
})

// ❌ BAD: Computation in Object
ObjectMapping::new("bad_example", |pulse| {
    // This complex logic belongs in Design Node!
    let result = expensive_computation(pulse);
    FieldPulse::new("result").with_response(vec![result])
})
```

### 3. Intentions Have Clear Names

```rust
// ✅ GOOD: Descriptive intention names
"validation_success"
"validation_error"
"authentication_required"
"data_ready_for_processing"

// ❌ BAD: Vague names
"result"
"done"
"next"
"temp"
```

---

## 🔬 Testing Pattern

### Test the Design Node

```rust
#[test]
fn test_validation_logic() {
    let dn = create_validate_poem_node();
    
    // Test success case
    let input = FieldPulse::new("test").with_response(vec!["Hello".to_string()]);
    let result = dn.execute(&[input]);
    assert_eq!(result.response[0], "Hello");
    
    // Test error case
    let long_input = FieldPulse::new("test")
        .with_response(vec!["x".repeat(20000)]);
    let result = dn.execute(&[long_input]);
    assert_eq!(result.response[0], "error_too_long");
}
```

### Test the Object Reflection

```rust
#[test]
fn test_intention_reflection() {
    let obj = create_reflect_validation_result();
    
    // Test success reflection
    let success_pulse = FieldPulse::new("validation_result")
        .with_response(vec!["Hello".to_string()]);
    let reflected = obj.apply(&success_pulse);
    assert_eq!(reflected.name, "validation_success");
    assert_eq!(reflected.tv, TV::Y);
    
    // Test error reflection
    let error_pulse = FieldPulse::new("validation_result")
        .with_response(vec!["error_too_long".to_string()]);
    let reflected = obj.apply(&error_pulse);
    assert_eq!(reflected.name, "validation_error");
    assert_eq!(reflected.tv, TV::N);
}
```

### Test the Full Flow

```rust
#[test]
fn test_full_validation_flow() {
    let cpux = PoemWriterCPUX::new();
    
    // Test valid input
    cpux.update_poem_text("Hello".to_string());
    let page = cpux.get_current_page().unwrap();
    assert_eq!(page.poem_text, "Hello");
    
    // Test invalid input (too long)
    cpux.update_poem_text("x".repeat(20000));
    let page = cpux.get_current_page().unwrap();
    assert_ne!(page.poem_text, "x".repeat(20000)); // Should not update
}
```

---

## 🚀 Advanced Use Cases

### Cascading Reflections

```rust
// Object 1: Reflect by type
Object1: result → "type_a" or "type_b"

// Design Node 1: Process type
DN1: "type_a" → processing...

// Object 2: Reflect by outcome
Object2: processing → "success" or "retry" or "failed"

// Design Node 2: Handle outcome
DN2: Different handlers for each
```

### Network Field Analysis

```rust
// You can trace the full decision path
let field = cpux.network_field.lock().unwrap();

for pulse in field.iter() {
    println!("{}: TV={:?}", pulse.name, pulse.tv);
}

// Output:
// user_input_poem: TV=U
// ready_for_update: TV=Y
// validation_result: TV=Y
// validation_success: TV=Y  ← Object reflected success
// update_poem_result: TV=Y
```

---

## 📖 Comparison with Traditional Patterns

### Traditional If/Else

```rust
if validate(input) {
    update(input);
} else {
    handle_error();
}

// Problems:
// - Mixed concerns
// - Hard to test stages
// - No history
// - Unclear flow
```

### Strategy Pattern

```rust
match input_type {
    TypeA => strategy_a.execute(input),
    TypeB => strategy_b.execute(input),
}

// Better, but:
// - Still mixed concerns
// - No clear data flow
// - Limited history
```

### Intention Space Pattern

```rust
Intention → Object → Intention (different!) → Design Node

// Benefits:
// - Clear stages
// - Testable isolation
// - Full history
// - Explicit flow
// - Debuggable names
```

---

## 🎓 Key Takeaways

1. **Objects examine results** from Design Nodes and reflect DIFFERENT intention names
2. **Intention names and TV values** enable branching while maintaining flow
3. **Objects remain pure** - they don't compute, they just reflect/route
4. **Design Nodes compute** - validation, processing, decisions
5. **Pattern is testable** at every stage
6. **Network Field captures** the full decision tree
7. **Flow remains clear** despite branching logic

---

## 🔄 The Pattern in Summary

```
I → O → I → DN → I → O → I
               ↑       ↑
               |       |
          Computes  Reflects
          Result    Different
                   Intentions
```

**This is how Intention Space handles branching logic while maintaining the flow pattern!**

---

*Objects don't just transform - they reflect the reality of what happened.*  
*Different results → Different intentions → Different paths*  
*The flow continues: I-O-I-DN-I-O-I-DN-I-GL-I ...*
