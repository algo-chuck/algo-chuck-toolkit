//! Individual crate usage example
//!
//! This example shows how to use individual workspace crates
//! directly instead of the facade crate.
//!
//! Run with: cargo run --example individual_crates

use schwab_api_core as core;
// use schwab_api_marketdata::add;

fn main() {
    println!("🔧 Individual Crates Example");
    println!();

    println!("📦 Using crates directly:");
    println!("  • schwab_api_core");
    println!("  • schwab_api_types");
    println!();

    // Use crates directly
    let core_result = core::add(10, 20);
    // let types_demo = Candle {
    //     open: Some(100.0),
    //     high: Some(110.0),
    //     low: Some(90.0),
    //     close: Some(105.0),
    //     volume: Some(1000),
    //     datetime: Some(1625247600),
    //     datetime_iso8601: None,
    // };

    println!("🔧 schwab_api_core::add(10, 20) = {}", core_result);
    // println!("📊 schwab_api_marketdata::Candle = {:#?}", types_demo);
    println!();

    println!("💡 Direct crate usage is useful when:");
    println!("   • You only need specific functionality");
    println!("   • You want explicit control over dependencies");
    println!("   • Building libraries that extend specific parts");
    println!("   • Maximum performance (no re-export overhead)");
}
