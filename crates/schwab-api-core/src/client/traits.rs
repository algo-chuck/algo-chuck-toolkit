//! HTTP client traits for async and sync implementations.

use async_trait::async_trait;
use http::{Request, Response};

/// Trait for asynchronous HTTP clients.
///
/// This trait defines the interface for async HTTP client implementations,
/// such as those using `reqwest` or other async HTTP libraries.
///
/// # Examples
///
/// ```ignore
/// use schwab_api_core::AsyncHttpClient;
/// use async_trait::async_trait;
///
/// struct MyAsyncClient {
///     client: reqwest::Client,
/// }
///
/// #[async_trait]
/// impl AsyncHttpClient for MyAsyncClient {
///     type Error = reqwest::Error;
///
///     async fn execute(&self, request: http::Request<String>)
///         -> Result<http::Response<String>, Self::Error>
///     {
///          // Implementation here
///     }
/// }
/// ```
#[async_trait]
pub trait AsyncHttpClient: Send + Sync {
    /// The error type returned by this client
    type Error: std::error::Error + Send + Sync + 'static;

    /// Execute an HTTP request asynchronously.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to execute
    ///
    /// # Returns
    ///
    /// A `Result` containing the HTTP response or an error
    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error>;
}

/// Trait for synchronous/blocking HTTP clients.
///
/// This trait defines the interface for blocking HTTP client implementations,
/// such as those using `ureq` or other synchronous HTTP libraries.
///
/// # Examples
///
/// ```ignore
/// use schwab_api_core::SyncHttpClient;
///
/// struct MySyncClient {
///     agent: ureq::Agent,
/// }
///
/// impl SyncHttpClient for MySyncClient {
///     type Error = ureq::Error;
///
///     fn execute(&self, request: http::Request<String>)
///         -> Result<http::Response<String>, Self::Error>
///     {
///         // Implementation here
///     }
/// }
/// ```
pub trait SyncHttpClient: Send + Sync {
    /// The error type returned by this client
    type Error: std::error::Error + Send + Sync + 'static;

    /// Execute an HTTP request synchronously.
    ///
    /// # Arguments
    ///
    /// * `request` - The HTTP request to execute
    ///
    /// # Returns
    ///
    /// A `Result` containing the HTTP response or an error
    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error>;
}
