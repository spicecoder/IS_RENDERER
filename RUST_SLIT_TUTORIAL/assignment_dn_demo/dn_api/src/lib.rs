// dn_api/src/lib.rs
use serde_json::Value;
use std::fs::OpenOptions;
use std::io::Write;

/// A Pulse is an arbitrary JSON value (variable / payload)
pub type Pulse = Value;

/// Intention carries an id and list of pulses
#[derive(Debug, Clone)]
pub struct Intention {
    pub id: String,
    pub pulses: Vec<Pulse>,
}

impl Intention {
    pub fn new(id: &str, pulses: Vec<Pulse>) -> Self {
        Self { id: id.to_string(), pulses }
    }
}

/// Very small NDJSON ledger append helper (public so DNs and macros can use it)
/// Make sure this is `pub` so other crates can call `dn_api::emit_ledger`.
pub fn emit_ledger(entry: &serde_json::Value) {
    let path = "ledger.ndjson";
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .expect("open ledger");
    f.write_all(entry.to_string().as_bytes()).expect("write");
    f.write_all(b"\n").expect("write newline");
}

// Helper that always returns a JSON value using Debug formatting.
// This avoids requiring Serialize on arbitrary types and guarantees compilation.
#[doc(hidden)]
pub fn _to_json_value_debug<T: std::fmt::Debug>(v: &T) -> serde_json::Value {
    serde_json::json!(format!("{:?}", v))
}

#[macro_export]
macro_rules! assign_val {
    ($name:ident, $expr:expr) => {{
        let val = $expr;
        let value_json = $crate::_to_json_value_debug(&val);
        let entry_put = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:PutValue",
            "pulses": { "value": value_json, "location": stringify!($name) }
        });
        $crate::emit_ledger(&entry_put);
        let name = val;
        let placed_value_json = $crate::_to_json_value_debug(&name);
        let entry_placed = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:ValuePlaced",
            "pulses": { "location": stringify!($name), "value": placed_value_json }
        });
        $crate::emit_ledger(&entry_placed);
        name
    }};
}

#[macro_export]
macro_rules! move_val {
    ($name:ident, $expr:expr) => {{
        let val = $expr;
        let value_json = $crate::_to_json_value_debug(&val);
        let entry_move = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:MoveValue",
            "pulses": { "from": "(expr)", "to": stringify!($name), "value": value_json }
        });
        $crate::emit_ledger(&entry_move);
        let name = val;
        let placed_value_json = $crate::_to_json_value_debug(&name);
        let entry_placed = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:ValuePlaced",
            "pulses": { "location": stringify!($name), "value": placed_value_json }
        });
        $crate::emit_ledger(&entry_placed);
        name
    }};
}

#[macro_export]
macro_rules! borrow_ref {
    ($r:ident, $x:ident) => {{
        let r = &$x;
        let entry_borrow = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:BorrowValue",
            "pulses": { "location": stringify!($x), "borrow_kind": "shared" }
        });
        $crate::emit_ledger(&entry_borrow);
        let entry_issued = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:BorrowIssued",
            "pulses": { "loan_token": format!("loan_{}", stringify!($r)), "location": stringify!($x) }
        });
        $crate::emit_ledger(&entry_issued);
        r
    }};
}

#[macro_export]
macro_rules! borrow_mut {
    ($r:ident, $x:ident) => {{
        let r = &mut $x;
        let entry_borrow = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:BorrowMutValue",
            "pulses": { "location": stringify!($x), "borrow_kind": "exclusive" }
        });
        $crate::emit_ledger(&entry_borrow);
        let entry_issued = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:BorrowIssued",
            "pulses": { "loan_token": format!("loan_{}", stringify!($r)), "location": stringify!($x), "mut": true }
        });
        $crate::emit_ledger(&entry_issued);
        r
    }};
}

#[macro_export]
macro_rules! rc_clone {
    ($target:ident, $src:expr) => {{
        let target = std::rc::Rc::clone(&$src);
        let inner_json = $crate::_to_json_value_debug(&*target);
        let entry = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:RefcountChanged",
            "pulses": { "src": stringify!($src), "target": stringify!($target), "inner": inner_json }
        });
        $crate::emit_ledger(&entry);
        target
    }};
}

#[macro_export]
macro_rules! drop_val {
    ($x:ident) => {{
        let entry = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:DropScheduled",
            "pulses": { "from": stringify!($x) }
        });
        $crate::emit_ledger(&entry);
        std::mem::drop($x);
        let entry2 = serde_json::json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:ValueDropped",
            "pulses": { "from": stringify!($x) }
        });
        $crate::emit_ledger(&entry2);
    }};
}
