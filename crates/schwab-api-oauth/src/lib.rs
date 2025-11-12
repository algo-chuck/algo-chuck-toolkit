//! # Schwab OAuth2 Client
//!
//! This crate provides both async and sync OAuth2 clients for Schwab API authentication.
//!
//! ## Features
//!
//! - `reqwest-client` - Enable async OAuth client using reqwest (default)
//! - `ureq-client` - Enable sync OAuth client using ureq (default)
//! - `default` - Enables both `reqwest-client` and `ureq-client`
//!
//! ## Async Usage
//!
//! ```rust,no_run
//! use schwab_api_oauth::{AsyncOAuthClient, OAuthConfig};
//!
//! async fn example() {
//!     let config = OAuthConfig::new(
//!         "client_id".to_string(),
//!         "client_secret".to_string(),
//!         "https://localhost:8182/callback".to_string(),
//!     );
//!     
//!     let client = AsyncOAuthClient::new(reqwest::Client::new(), config);
//!     let auth_url = client.build_auth_url("random_state").unwrap();
//!     // ... redirect user to auth_url, get code from callback ...
//!     let tokens = client.exchange_code_for_token("auth_code").await.unwrap();
//!     println!("Access token: {}", tokens.access_token);
//! }
//! ```
//!
//! ## Sync Usage
//!
//! ```rust,no_run
//! use schwab_api_oauth::{SyncOAuthClient, OAuthConfig};
//!
//! fn example() {
//!     let config = OAuthConfig::new(
//!         "client_id".to_string(),
//!         "client_secret".to_string(),
//!         "https://localhost:8182/callback".to_string(),
//!     );
//!     
//!     let client = SyncOAuthClient::new(ureq::Agent::new(), config);
//!     let auth_url = client.build_auth_url("random_state").unwrap();
//!     // ... redirect user to auth_url, get code from callback ...
//!     let tokens = client.exchange_code_for_token("auth_code").unwrap();
//!     println!("Access token: {}", tokens.access_token);
//! }
//! ```

#[cfg(feature = "reqwest-client")]
mod async_client;
#[cfg(feature = "ureq-client")]
mod sync_client;

pub mod config;
pub mod error;

use serde::{Deserialize, Serialize};

/// Response structure from Schwab OAuth2 token endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
    pub id_token: String,
}

pub use config::OAuthConfig;
pub use error::{OAuthError, Result};

#[cfg(feature = "reqwest-client")]
pub use async_client::AsyncOAuthClient;

#[cfg(feature = "ureq-client")]
pub use sync_client::SyncOAuthClient;
