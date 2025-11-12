//! Minimal example using only core types
//!
//! This example demonstrates using the facade crate with
//! default features disabled for minimal dependencies.
//!
//! Run with: cargo run --example minimal --no-default-features

use schwab_api::{VERSION, types::marketdata::Candle};

fn main() {
    println!("🎯 Minimal Schwab API Example");
    println!("📦 Version: {}", VERSION);
    println!();

    println!("📋 Available with no default features:");
    println!("  ✅ Core functionality");
    println!("  ✅ Type definitions");
    println!("  ❌ OAuth (disabled)");
    println!("  ❌ Market Data (disabled)");
    println!("  ❌ Trader (disabled)");
    println!();

    // Only core and types are available
    // let core_demo = core::add(1, 2);
    let types_demo = Candle {
        open: Some(100.0),
        high: Some(110.0),
        low: Some(90.0),
        close: Some(105.0),
        volume: Some(1000),
        datetime: Some(1625247600),
        datetime_iso8601: None,
    };

    // println!("🔧 Core example: {}", core_demo);
    println!("📊 Types example: {:#?}", types_demo);
    println!();
    println!("💡 This minimal setup is perfect for:");
    println!("   • Building custom implementations");
    println!("   • Library authors extending the API");
    println!("   • WASM or embedded targets");
    println!("   • Minimal dependency requirements");
}
