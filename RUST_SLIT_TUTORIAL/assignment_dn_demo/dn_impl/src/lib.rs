use dn_api::{emit_ledger};
use serde_json::json;

/// A light Assignment DN that can be called programmatically.
/// In this demo the macros in dn_api also emit intentions directly;
/// this struct is a demonstration of a reusable DN component.
pub struct AssignmentDn;

impl AssignmentDn {
    pub fn new() -> Self { AssignmentDn }
    pub fn handle_put_value(&self, location: &str, value: serde_json::Value) {
        let entry = json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:PutValue",
            "pulses": { "location": location, "value": value }
        });
        emit_ledger(&entry);
        let entry2 = json!({
            "ts": chrono::Utc::now().to_rfc3339(),
            "dc": "dc_assignment",
            "intention": "I:ValuePlaced",
            "pulses": { "location": location }
        });
        emit_ledger(&entry2);
    }
}
