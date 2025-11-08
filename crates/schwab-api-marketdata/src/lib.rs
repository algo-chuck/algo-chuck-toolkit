//! # Schwab Market Data API Client
//!
//! This crate provides both async and sync clients for interacting with the Schwab Market Data API.
//!
//! ## Async Usage
//!
//! ```rust,no_run
//! use schwab_api_marketdata::AsyncMarketdataClient;
//!
//! async fn example() {
//!     let http_client = reqwest::Client::new();
//!     let marketdata = AsyncMarketdataClient::new(http_client);
//!     
//!     let token = "your_access_token";
//!     let quotes = marketdata.get_quote(token, "AAPL", None).await.unwrap();
//!     println!("{:#?}", quotes);
//! }
//! ```
//!
//! ## Sync Usage
//!
//! ```rust,no_run
//! use schwab_api_marketdata::SyncMarketdataClient;
//!
//! fn example() {
//!     let http_client = ureq::Agent::new();
//!     let marketdata = SyncMarketdataClient::new(http_client);
//!     
//!     let token = "your_access_token";
//!     let quotes = marketdata.get_quote(token, "AAPL", None).unwrap();
//!     println!("{:#?}", quotes);
//! }
//! ```

mod async_client;
mod params;
mod sync_client;

pub use schwab_api_core::ApiConfig;

/// Configuration for Schwab Market Data API
pub struct MarketdataConfig;

impl ApiConfig for MarketdataConfig {
    fn base_url() -> &'static str {
        "https://api.schwabapi.com/marketdata/v1"
    }
}

/// Re-export async and sync client types
pub use async_client::AsyncMarketdataClient;
pub use sync_client::SyncMarketdataClient;

/// Re-export MarketdataParams for advanced users who want direct parameter access
pub use params::MarketdataParams;
