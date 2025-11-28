# Assignment DN Demo (Rust)

This workspace demonstrates modelling Rust assignment/ownership/borrow semantics
as an Assignment DN with Intention + Pulse traces.

Crates:
- dn_api: shared API (Intention type, emit_ledger) and instrumentation macros:
  - assign_val!(name, expr)
  - move_val!(name, expr)
  - borrow_ref!(r, x)
  - borrow_mut!(r, x)
  - rc_clone!(target, src)
  - drop_val!(x)
- dn_impl: AssignmentDn with a programmatic handle example
- assignment_demo: binary that uses the macros and the DN

Run:
```bash
# from workspace root
cargo run -p assignment_demo
```

This will append NDJSON lines to `ledger.ndjson` in the current working directory.
I also included `ledger_samples.ndjson` showing example traces for reference.
