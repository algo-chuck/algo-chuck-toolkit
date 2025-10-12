//! Minimal example using only core types
//!
//! This example demonstrates using the facade crate with
//! default features disabled for minimal dependencies.
//!
//! Run with: cargo run --example minimal --no-default-features

use schwab_api::{VERSION, core, types};

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
    let core_demo = core::add(1, 2);
    let types_demo = types::add(3, 4);

    println!("🔧 Core example: {}", core_demo);
    println!("📊 Types example: {}", types_demo);
    println!();
    println!("💡 This minimal setup is perfect for:");
    println!("   • Building custom implementations");
    println!("   • Library authors extending the API");
    println!("   • WASM or embedded targets");
    println!("   • Minimal dependency requirements");
}
