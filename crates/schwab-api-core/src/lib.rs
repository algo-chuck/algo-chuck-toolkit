//! # Schwab API Core
//!
//! Core HTTP client infrastructure for interacting with Charles Schwab API.
//!
//! This crate provides:
//! - Async and sync HTTP client traits
//! - Generic API client implementation
//! - Error types for API interactions
//! - Response parsing utilities
//!
//! ## Features
//!
//! - `reqwest-client` - Enable async HTTP client using reqwest (default)
//! - `ureq-client` - Enable sync/blocking HTTP client using ureq
//! - `trader` - Include trader API error types (default)
//! - `marketdata` - Include market data API error types (default)
//! - `default` - Enables `reqwest-client`, `trader`, and `marketdata`
//!
//! ## Examples
//!
//! ### Synchronous Client
//!
//! ```ignore
//! use schwab_api_core::{ApiClient, ApiConfig, RequestParams};
//!
//! pub struct TraderConfig;
//!
//! impl ApiConfig for TraderConfig {
//!     fn base_url() -> &'static str {
//!         "https://api.schwabapi.com/trader/v1"
//!     }
//! }
//!
//! let http_client = ureq::Agent::new();
//! let client = ApiClient::<_, TraderConfig>::new(http_client);
//! ```
//!
//! ### Asynchronous Client
//!
//! ```ignore
//! use schwab_api_core::{ApiClient, ApiConfig, RequestParams};
//!
//! pub struct TraderConfig;
//!
//! impl ApiConfig for TraderConfig {
//!     fn base_url() -> &'static str {
//!         "https://api.schwabapi.com/trader/v1"
//!     }
//! }
//!
//! let http_client = reqwest::Client::new();
//! let client = ApiClient::<_, TraderConfig>::new(http_client);
//! ```

mod client;
mod config;
mod error;
mod response;

// Feature-gated HTTP client implementations
#[cfg(feature = "reqwest-client")]
mod reqwest_client;

#[cfg(feature = "ureq-client")]
mod ureq_client;

// Re-export public API
pub use client::{ApiClient, AsyncHttpClient, HttpClient, RequestParams, SyncHttpClient};
pub use config::ApiConfig;
pub use error::{HttpError, Result, SchwabError, parse_api_error};
pub use response::{HttpResponse, SchwabSuccess};
