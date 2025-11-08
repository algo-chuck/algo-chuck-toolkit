//! # Schwab Trader API Client
//!
//! This crate provides a client for interacting with the Schwab Trader API.
//!
//! ## Usage Patterns
//!
//! ### CLI or Single-Use (Owned Client)
//! ```rust,no_run
//! use schwab_api_trader::TraderClient;
//!
//! async fn example() {
//!     let http_client = reqwest::Client::new();
//!     let trader = TraderClient::new(http_client);
//!     // Use trader...
//! }
//! ```
//!
//! ### Server or Shared Client (Borrowed)
//! ```rust,no_run
//! use schwab_api_trader::TraderClient;
//! use std::sync::LazyLock;
//!
//! // Shared client for the entire application
//! static HTTP: LazyLock<reqwest::Client> = LazyLock::new(|| {
//!     reqwest::Client::builder()
//!         .timeout(std::time::Duration::from_secs(30))
//!         .build()
//!         .unwrap()
//! });
//!
//! async fn handle_request() {
//!     // Borrow the shared client - zero cost!
//!     let trader = TraderClient::new(&*HTTP);
//!     // Use trader...
//! }
//! ```
//!
//! ### Explicit Arc Sharing
//! ```rust,no_run
//! use schwab_api_trader::TraderClient;
//! use std::sync::Arc;
//!
//! async fn example() {
//!     let http_client = Arc::new(reqwest::Client::new());
//!     let trader = TraderClient::new(http_client.clone());
//!     // Use trader...
//! }
//! ```

mod asynchronous;
mod client;
mod params;
mod synchronous;

pub use client::TraderClient;
