# 🎯 CRITICAL CORRECTION: Objects Never Change Payload

## The Golden Rule of Intention Space

---

## ⚠️ The Fundamental Principle

```
┌─────────────────────────────────────────────────────────┐
│  Objects map intentions based on payload examination,   │
│  but NEVER change the payload itself.                   │
└─────────────────────────────────────────────────────────┘
```

---

## 🔍 What This Means

### Objects Can:
✅ **Read/examine** the payload  
✅ **Map to different** intention names  
✅ **Change** TV values (Y/N/U)  
✅ **Route** based on payload content  

### Objects Cannot:
❌ **Modify** the payload data  
❌ **Transform** strings (trim, format, etc.)  
❌ **Compute** new values  
❌ **Add/remove** data from payload  
❌ **Perform I/O** operations  

---

## 📊 The Correct Pattern

### Object: Examines and Routes (Payload Unchanged)

```rust
ObjectMapping::new("reflect_validation_result", |pulse| {
    // ✅ Read the payload
    let result = pulse.response.first().unwrap_or(&String::new());
    
    // ✅ Examine content
    if result.starts_with("error_") {
        // ✅ Map to different intention
        // ❗ CRITICAL: Clone payload unchanged
        FieldPulse::new("validation_error")
            .with_response(pulse.response.clone())  // SAME!
            .with_tv(TV::N)
    } else {
        FieldPulse::new("validation_success")
            .with_response(pulse.response.clone())  // SAME!
            .with_tv(TV::Y)
    }
})
```

**What Changed:**
- Intention name: `validation_error` vs `validation_success`
- TV value: `N` vs `Y`

**What Did NOT Change:**
- Payload: `pulse.response.clone()` - IDENTICAL to input

---

### Design Node: Computes and Transforms (Payload Can Change)

```rust
DesignNode::new("validate_poem", |inputs| {
    // Get input payload
    let text = inputs[0].response[0].clone();
    
    // ✅ Transform data (ONLY Design Nodes can do this)
    let validated = text.trim().to_string();
    
    // ✅ Compute result
    if validated.len() > 10000 {
        // Returns DIFFERENT payload
        vec!["error_too_long".to_string()]
    } else {
        // Returns MODIFIED payload (trimmed)
        vec![validated]
    }
})
```

**What Changed:**
- Payload: Trimmed whitespace, or returned error message

---

## 🔄 Complete Flow Showing Payload Tracking

```
User Input: "  Hello  " (with spaces)

Intention 1: user_input_poem
  Payload: ["  Hello  "]
  
Object 1: route_to_validation
  Payload: ["  Hello  "] ← UNCHANGED
  
Intention 2: ready_for_validation
  Payload: ["  Hello  "] ← STILL THE SAME
  
Design Node 1: validate_poem
  Payload: ["Hello"] ← CHANGED (trimmed)
  
Intention 3: validation_result
  Payload: ["Hello"] ← NEW VALUE FROM DN
  
Object 2: reflect_validation_result
  Payload: ["Hello"] ← UNCHANGED (cloned from Intention 3)
  
Intention 4: validation_success
  Payload: ["Hello"] ← STILL THE SAME
  
Design Node 2: update_poem
  Payload: ["Hello"] ← MAY CHANGE AGAIN
```

---

## 🎨 Why This Rule Exists

### 1. **Clear Separation of Concerns**
- Objects = Routing logic
- Design Nodes = Computation logic

### 2. **Testability**
```rust
// Test Object: Verify routing only
#[test]
fn test_object_preserves_payload() {
    let obj = create_reflect_object();
    let input = FieldPulse::new("test")
        .with_response(vec!["data".to_string()]);
    
    let output = obj.apply(&input);
    
    // Payload must be identical
    assert_eq!(input.response, output.response);
    // Only name/TV should differ
    assert_ne!(input.name, output.name);
}

// Test Design Node: Verify computation
#[test]
fn test_design_node_transforms() {
    let dn = create_validate_node();
    let input = FieldPulse::new("test")
        .with_response(vec!["  Hello  ".to_string()]);
    
    let output = dn.execute(&[input]);
    
    // Payload should be transformed
    assert_eq!(output.response[0], "Hello");
}
```

### 3. **Debuggability**
```rust
// Can trace exactly where data changes
let field = cpux.network_field.lock().unwrap();
for pulse in field.iter() {
    println!("{}: {:?}", pulse.name, pulse.response);
}

// Output clearly shows:
// user_input_poem: ["  Hello  "]
// ready_for_validation: ["  Hello  "]  ← Same
// validation_result: ["Hello"]  ← Changed (by DN)
// validation_success: ["Hello"]  ← Same
```

### 4. **Performance**
- Objects are fast (just routing)
- Heavy computation isolated to Design Nodes
- Easy to optimize specific Design Nodes

---

## ❌ Common Mistakes

### Mistake 1: Object Modifying Payload

```rust
// ❌ WRONG
ObjectMapping::new("bad_example", |pulse| {
    let text = pulse.response[0].trim();  // Modifying!
    FieldPulse::new("trimmed")
        .with_response(vec![text.to_string()])
});
```

**Fix:** Move trimming to Design Node
```rust
// ✅ CORRECT
DesignNode::new("trim_text", |inputs| {
    let text = inputs[0].response[0].trim();
    vec![text.to_string()]
})
```

### Mistake 2: Design Node Routing Only

```rust
// ❌ WRONG (waste of Design Node)
DesignNode::new("just_routing", |inputs| {
    // No actual computation, just routing
    inputs[0].response.clone()
})
```

**Fix:** Use Object for routing
```rust
// ✅ CORRECT
ObjectMapping::new("route_to_next", |pulse| {
    FieldPulse::new("next_stage")
        .with_response(pulse.response.clone())
})
```

---

## 🎓 Quick Reference

### When to Use Object
- Examining payload content
- Routing to different intentions
- Setting TV based on payload
- Simple if/match logic
- **Payload must be IDENTICAL**

### When to Use Design Node
- Transforming data (trim, format, convert)
- Validating and computing
- Database/file/network I/O
- Complex algorithms
- **Payload CAN be different**

---

## 📝 Code Review Checklist

When reviewing Objects:
- [ ] Does it clone `pulse.response`?
- [ ] Does it only change intention name?
- [ ] Does it only change TV value?
- [ ] Is there NO data transformation?
- [ ] Is there NO computation?
- [ ] Is there NO I/O?

When reviewing Design Nodes:
- [ ] Does it perform actual computation?
- [ ] Does it transform or validate data?
- [ ] Is this logic testable in isolation?
- [ ] Could this be split into smaller DNs?

---

## 🎯 Summary

```
OBJECTS:
  Input Payload:  ["data"]
  Output Payload: ["data"] ← MUST BE IDENTICAL
  Changes:        intention name, TV value

DESIGN NODES:
  Input Payload:  ["data"]
  Output Payload: ["transformed"] ← CAN BE DIFFERENT
  Changes:        anything and everything
```

---

**This is THE fundamental rule that separates Objects from Design Nodes in Intention Space physics.**

**Violating this rule breaks the entire computational model.**

✅ **Objects = Pure Routing**  
✅ **Design Nodes = Computation**  
✅ **Payload preservation in Objects is NON-NEGOTIABLE**

---

*Updated: All code and documentation now correctly implement this rule.*  
*See: src/main.rs, OBJECTS_REFLECTING_INTENTIONS.md, ARCHITECTURE.md*
