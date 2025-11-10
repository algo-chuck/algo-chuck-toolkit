//! HTTP client infrastructure for Schwab API.
//!
//! This module provides the client abstractions for making HTTP requests
//! to the Schwab API, supporting both async and sync execution models.

pub mod api_client;
pub mod params;
pub mod traits;

use http::{Request, Response};

pub use api_client::ApiClient;
pub use params::RequestParams;
pub use traits::{AsyncHttpClient, SyncHttpClient};

/// Generic HTTP client wrapper that works with either sync or async implementations.
///
/// This type wraps an underlying HTTP client implementation and provides a
/// consistent interface for both async and sync operations.
///
/// # Type Parameters
///
/// * `C` - The underlying client type (e.g., `reqwest::Client` or `ureq::Agent`)
///
/// # Examples
///
/// ```ignore
/// use schwab_api_core::HttpClient;
///
/// // With a sync client
/// let agent = ureq::Agent::new();
/// let client = HttpClient::new(agent);
///
/// // With an async client
/// let reqwest_client = reqwest::Client::new();
/// let async_client = HttpClient::new(reqwest_client);
/// ```
pub struct HttpClient<C> {
    client: C,
}

impl<C> HttpClient<C> {
    /// Create a new HTTP client wrapper.
    ///
    /// # Arguments
    ///
    /// * `client` - The underlying HTTP client implementation
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Get a reference to the inner HTTP client.
    ///
    /// This is useful when you need direct access to the underlying client
    /// for advanced configuration or custom requests.
    pub fn inner(&self) -> &C {
        &self.client
    }
}

impl<C: AsyncHttpClient> HttpClient<C> {
    /// Execute an HTTP request asynchronously.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to execute
    ///
    /// # Returns
    ///
    /// A `Result` containing the HTTP response or an error
    pub async fn execute(&self, request: Request<String>) -> Result<Response<String>, C::Error> {
        self.client.execute(request).await
    }
}

impl<C: SyncHttpClient> HttpClient<C> {
    /// Execute an HTTP request synchronously.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to execute
    ///
    /// # Returns
    ///
    /// A `Result` containing the HTTP response or an error
    pub fn execute_sync(&self, request: Request<String>) -> Result<Response<String>, C::Error> {
        self.client.execute(request)
    }
}
