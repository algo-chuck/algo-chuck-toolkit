//! # Schwab API Toolkit
//!
//! A unified Rust library for interacting with the Charles Schwab Developer API.
//! This facade crate provides a single entry point for all Schwab API functionality.
//!
//! ## Features
//!
//! - **Types**: Core data structures and types for Schwab API
//! - **OAuth**: Authentication and authorization flows  
//! - **Market Data**: Real-time and historical market data
//! - **Trading**: Order placement and account management
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use schwab_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // TODO: Example usage once core types are implemented
//!     println!("Schwab API Toolkit v{}", schwab_api::VERSION);
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! By default, all major features are enabled. You can opt into specific functionality:
//!
//! ```toml
//! [dependencies]
//! schwab-api = { version = "0.1", default-features = false, features = ["market-data"] }
//! ```

#![deny(missing_docs)]
#![warn(clippy::all)]

// Re-export core crates as modules
pub use schwab_api_core as core;
pub use schwab_api_types as types;

// Feature-gated re-exports
#[cfg(feature = "oauth")]
pub use schwab_api_oauth as oauth;

#[cfg(feature = "marketdata")]
pub use schwab_api_marketdata as marketdata;

#[cfg(feature = "trader")]
pub use schwab_api_trader as trader;

/// Convenience prelude for common imports
///
/// This module will be populated as the individual crates are implemented
pub mod prelude {
    // Will re-export common types and traits once they're implemented
    // pub use crate::marketdata::*;
    // pub use crate::core::*;
}

/// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_available() {
        assert!(!VERSION.is_empty());
        println!("Schwab API Toolkit version: {}", VERSION);
    }

    #[test]
    fn modules_are_accessible() {
        // Test that we can access the re-exported modules
        // These will contain actual functionality once implemented
        // let _ = core::add(1, 1); // Using the placeholder function for now
        // let _ = marketdata::add(1, 1); // Using the placeholder function for now
    }
}
