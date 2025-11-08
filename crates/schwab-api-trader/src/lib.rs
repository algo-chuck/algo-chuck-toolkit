//! # Schwab Trader API Client
//!
//! This crate provides both async and sync clients for interacting with the Schwab Trader API.
//!
//! ## Async Usage
//!
//! ```rust,no_run
//! use schwab_api_trader::AsyncTraderClient;
//!
//! async fn example() {
//!     let http_client = reqwest::Client::new();
//!     let trader = AsyncTraderClient::new(http_client);
//!     
//!     let token = "your_access_token";
//!     let accounts = trader.get_account_numbers(token).await.unwrap();
//!     println!("{:#?}", accounts);
//! }
//! ```
//!
//! ## Sync Usage
//!
//! ```rust,no_run
//! use schwab_api_trader::SyncTraderClient;
//!
//! fn example() {
//!     let http_client = ureq::Agent::new();
//!     let trader = SyncTraderClient::new(http_client);
//!     
//!     let token = "your_access_token";
//!     let accounts = trader.get_account_numbers(token).unwrap();
//!     println!("{:#?}", accounts);
//! }
//! ```

mod async_client;
mod params;
mod sync_client;

pub use schwab_api_core::ApiConfig;

/// Configuration for Schwab Trader API
pub struct TraderConfig;

impl ApiConfig for TraderConfig {
    fn base_url() -> &'static str {
        "https://api.schwabapi.com/trader/v1"
    }
}

/// Re-export async and sync client types
pub use async_client::AsyncTraderClient;
pub use sync_client::SyncTraderClient;

/// Re-export TraderParams for advanced users who want direct parameter access
pub use params::TraderParams;
