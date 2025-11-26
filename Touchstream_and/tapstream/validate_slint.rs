#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! slint-build = "1.8"
//! ```

// Standalone Slint File Validator
// Usage: rust-script validate_slint.rs <path/to/file.slint>

use std::env;
use std::path::PathBuf;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: {} <slint-file>", args[0]);
        eprintln!("Example: {} ui/tapstream.slint", args[0]);
        process::exit(1);
    }
    
    let slint_file = PathBuf::from(&args[1]);
    
    if !slint_file.exists() {
        eprintln!("❌ File not found: {}", slint_file.display());
        process::exit(1);
    }
    
    println!("🔍 Validating Slint file: {}", slint_file.display());
    println!();
    
    // Compile the Slint file
    match slint_build::compile(&slint_file) {
        Ok(_) => {
            println!("✅ Slint file is valid!");
            println!("   No syntax errors found");
            println!("   Ready to build");
        }
        Err(e) => {
            eprintln!("❌ Slint compilation failed:");
            eprintln!();
            eprintln!("{}", e);
            eprintln!();
            eprintln!("Common issues:");
            eprintln!("  - Unknown element (check component names)");
            eprintln!("  - Property errors (in vs in-out)");
            eprintln!("  - Callback signature mismatch");
            eprintln!("  - Two-way binding issues (use <=>)");
            process::exit(1);
        }
    }
}
