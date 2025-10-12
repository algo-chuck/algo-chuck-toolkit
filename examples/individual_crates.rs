//! Individual crate usage example
//!
//! This example shows how to use individual workspace crates
//! directly instead of the facade crate.
//!
//! Run with: cargo run --example individual_crates

use schwab_api_core as core;
use schwab_api_types as types;

fn main() {
    println!("🔧 Individual Crates Example");
    println!();
    
    println!("📦 Using crates directly:");
    println!("  • schwab_api_core");
    println!("  • schwab_api_types");
    println!();
    
    // Use crates directly
    let core_result = core::add(10, 20);
    let types_result = types::add(30, 40);
    
    println!("🔧 schwab_api_core::add(10, 20) = {}", core_result);
    println!("📊 schwab_api_types::add(30, 40) = {}", types_result);
    println!();
    
    println!("💡 Direct crate usage is useful when:");
    println!("   • You only need specific functionality");
    println!("   • You want explicit control over dependencies");
    println!("   • Building libraries that extend specific parts");
    println!("   • Maximum performance (no re-export overhead)");
}