
use dn_api::*;
use dn_impl::AssignmentDn;
use serde_json::json;
use std::rc::Rc;

fn main() {
    // 1) simple copy assignment
    let x = assign_val!(x, 42);
    println!("x = {}", x);

    // 2) move assignment (vector)
    let v = vec![1,2,3];
    let y = move_val!(y, v);
    println!("y = {:?}", y);
    // note: `v` is moved; using v now would be a compile error (as expected)

    // 3) borrow shared
    let a = assign_val!(a, 100);
    let r = borrow_ref!(r, a);
    println!("r (shared borrow) = {}", r);

    // 4) mutable borrow
    let mut m = assign_val!(m, 10);
    {
        let rm = borrow_mut!(rm, m);
        *rm += 5;
        println!("rm (mut borrow) = {}", rm);
    }
    println!("m after mut borrow = {}", m);

    // 5) Rc clone / refcount demonstration
    let rc1 = assign_val!(rc1, Rc::new(vec![9,8,7]));
    let rc2 = rc_clone!(rc2, rc1);
    println!("rc1 len {}, rc2 len {}", rc1.len(), rc2.len());

    // 6) drop demonstration
    let z = assign_val!(z, String::from("to be dropped"));
    drop_val!(z);
    // z is moved into drop; cannot use further

    // 7) using AssignmentDn API directly
    let dn = AssignmentDn::new();
    dn.handle_put_value("direct_loc", json!({"hello":"world"}));
    println!("demo complete - ledger.ndjson updated with intents/pulses.");
}
