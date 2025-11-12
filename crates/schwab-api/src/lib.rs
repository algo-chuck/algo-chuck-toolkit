//! # Schwab API Toolkit
//!
//! A unified Rust library for interacting with the Charles Schwab Developer API.
//!
//! ## Quick Start
//!
//! ### Async Trading Client
//!
//! ```rust,no_run
//! use schwab_api::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let oauth_config = OAuthConfig::new(
//!         "client_id".into(),
//!         "client_secret".into(),
//!         "redirect_uri".into(),
//!     );
//!     
//!     let oauth = AsyncOAuthClient::new(reqwest::Client::new(), oauth_config);
//!     // ... authenticate and get access_token ...
//!     
//!     let trader = AsyncTraderClient::new(reqwest::Client::new(), access_token);
//!     let accounts = trader.get_accounts().await?;
//!     println!("Accounts: {:?}", accounts);
//!     Ok(())
//! }
//! ```
//!
//! ### Sync Market Data Client
//!
//! ```rust,no_run
//! use schwab_api::prelude::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = SyncMarketdataClient::new(ureq::Agent::new(), access_token);
//!     let quote = client.get_quote("AAPL", None)?;
//!     println!("Quote: {:?}", quote);
//!     Ok(())
//! }
//! ```
//!
//! ## Feature Flags
//!
//! Control which APIs and HTTP clients are included:
//!
//! ### API Selection
//!
//! ```toml
//! # Only trader API (no market data)
//! schwab-api = { version = "0.1", default-features = false, features = ["trader", "oauth", "async-only"] }
//!
//! # Only market data (no trading)
//! schwab-api = { version = "0.1", default-features = false, features = ["marketdata", "oauth", "sync-only"] }
//!
//! # Everything (default)
//! schwab-api = "0.1"
//! ```
//!
//! ### HTTP Client Selection
//!
//! - `reqwest-client` - Async HTTP client (requires tokio)
//! - `ureq-client` - Sync/blocking HTTP client (no async runtime)
//! - `async-only` - Convenience: reqwest + both APIs
//! - `sync-only` - Convenience: ureq + both APIs
//!
//! ### Available Features
//!
//! - `trader` - Trading API (accounts, orders, transactions)
//! - `marketdata` - Market data API (quotes, options, price history)
//! - `oauth` - OAuth authentication
//! - `reqwest-client` - Async HTTP support
//! - `ureq-client` - Sync HTTP support
//! - `default` - Everything enabled
//! - `full` - Explicit alias for default
//!
//! ## Module Organization
//!
//! - [`types`] - Core data structures (always available)
//! - [`core`] - HTTP client traits and errors (always available)
//! - [`oauth`] - OAuth authentication (feature: `oauth`)
//! - [`trader`] - Trading API client (feature: `trader`)
//! - [`marketdata`] - Market data API client (feature: `marketdata`)
//! - [`prelude`] - Common imports for convenience

#![deny(missing_docs)]
#![warn(clippy::all)]

// Re-export core crates as modules (always available)
pub use schwab_api_core as core;
pub use schwab_api_types as types;

// Feature-gated re-exports
#[cfg(feature = "oauth")]
pub use schwab_api_oauth as oauth;

#[cfg(feature = "marketdata")]
pub use schwab_api_marketdata as marketdata;

#[cfg(feature = "trader")]
pub use schwab_api_trader as trader;

/// Convenience prelude that re-exports commonly used types
pub mod prelude {
    pub use crate::core::{ApiClient, HttpError, SchwabError};
    pub use crate::types;

    #[cfg(feature = "oauth")]
    pub use crate::oauth::{OAuthConfig, OAuthError, TokenResponse};

    #[cfg(all(feature = "oauth", feature = "reqwest-client"))]
    pub use crate::oauth::AsyncOAuthClient;

    #[cfg(all(feature = "oauth", feature = "ureq-client"))]
    pub use crate::oauth::SyncOAuthClient;

    #[cfg(all(feature = "trader", feature = "reqwest-client"))]
    pub use crate::trader::AsyncTraderClient;

    #[cfg(all(feature = "trader", feature = "ureq-client"))]
    pub use crate::trader::SyncTraderClient;

    #[cfg(all(feature = "marketdata", feature = "reqwest-client"))]
    pub use crate::marketdata::AsyncMarketdataClient;

    #[cfg(all(feature = "marketdata", feature = "ureq-client"))]
    pub use crate::marketdata::SyncMarketdataClient;
}

/// Version of the schwab-api crate
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
