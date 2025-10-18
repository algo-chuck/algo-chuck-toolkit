//! Basic example demonstrating the Schwab API facade crate
//!
//! This example shows how to use the unified `schwab-api` crate
//! and access the individual modules.
//!
//! Run with: cargo run --example basic

use schwab_api::{self as schwab, VERSION};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Schwab API Toolkit Basic Example");
    println!("📦 Version: {}", VERSION);
    println!();

    // Demonstrate accessing the facade crate modules
    println!("📋 Available modules:");

    // Core module (always available)
    println!("  ✅ schwab::core - Core functionality");
    // let core_result = schwab::core::add(2, 3);
    // println!("     Example: core::add(2, 3) = {}", core_result);

    // Types module (always available)
    println!("  ✅ schwab::types - API data structures");
    let types_demo = schwab::types::marketdata::Candle {
        open: Some(100.0),
        high: Some(110.0),
        low: Some(90.0),
        close: Some(105.0),
        volume: Some(1000),
        datetime: Some(1625247600),
        datetime_iso8601: None,
    };

    println!("     Example: marketdata::Candle = {:#?}", types_demo);

    // Feature-gated modules - only try to use them if the features are enabled
    #[cfg(feature = "oauth")]
    {
        println!("  ✅ schwab::oauth - Authentication (feature enabled)");
        // Note: Individual modules will have actual functionality once implemented
        let oauth_result = schwab::oauth::add(5, 7);
        println!("     Example: oauth::add(5, 7) = {}", oauth_result);
    }
    #[cfg(not(feature = "oauth"))]
    {
        println!("  ⚠️  schwab::oauth - Authentication (feature disabled)");
    }

    #[cfg(feature = "marketdata")]
    {
        println!("  ✅ schwab::marketdata - Market data (feature enabled)");
        // Note: Individual modules will have actual functionality once implemented
        let market_data_result = schwab::marketdata::add(5, 7);
        println!(
            "     Example: marketdata::add(5, 7) = {}",
            market_data_result
        );
    }
    #[cfg(not(feature = "marketdata"))]
    {
        println!("  ⚠️  schwab::marketdata - Market data (feature disabled)");
    }

    #[cfg(feature = "trader")]
    {
        println!("  ✅ schwab::trader - Trader API (feature enabled)");
        // Note: Individual modules will have actual functionality once implemented
        // let trader_result = schwab::trader::add(5, 7);
        // println!("     Example: trader::add(5, 7) = {}", trader_result);
    }
    #[cfg(not(feature = "trader"))]
    {
        println!("  ⚠️  schwab::trader - Trader API (feature disabled)");
    }

    println!();
    println!("🎯 Feature Configuration:");
    println!("  To run with specific features:");
    println!("    cargo run --example basic --features oauth");
    println!("    cargo run --example basic --features marketdata");
    println!("    cargo run --example basic --features trader");
    println!("    cargo run --example basic --features full");
    println!();
    println!("  To run with minimal features:");
    println!("    cargo run --example basic --no-default-features");

    Ok(())
}
