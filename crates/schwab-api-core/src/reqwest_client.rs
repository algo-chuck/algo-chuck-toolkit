//! Async HTTP client implementation using reqwest.
//!
//! This module provides [`AsyncHttpClient`] trait implementations for `reqwest` types,
//! enabling async HTTP requests to the Schwab API.
//!
//! # Supported Types
//!
//! This module implements [`AsyncHttpClient`] for:
//! - `reqwest::Client` (owned)
//! - `&reqwest::Client` (borrowed reference)
//! - `Arc<reqwest::Client>` (thread-safe shared reference)
//!
//! # Error Handling
//!
//! All implementations automatically:
//! - Parse Schwab API error responses (4xx/5xx) into structured [`crate::SchwabError`] types
//! - Convert network errors into [`HttpError::NetworkError`]
//! - Handle malformed responses gracefully
//!
//! # Examples
//!
//! ```ignore
//! use schwab_api_core::{ApiClient, ApiConfig};
//! use reqwest::Client;
//!
//! // Define your API configuration
//! struct MyApiConfig;
//! impl ApiConfig for MyApiConfig {
//!     fn base_url() -> &'static str {
//!         "https://api.schwabapi.com/trader/v1"
//!     }
//! }
//!
//! // Create client with owned reqwest::Client
//! let http_client = Client::new();
//! let api_client = ApiClient::<_, MyApiConfig>::new(http_client);
//!
//! // Or use Arc for sharing across tasks
//! let http_client = std::sync::Arc::new(Client::new());
//! let api_client = ApiClient::<_, MyApiConfig>::new(http_client);
//! ```

use async_trait::async_trait;
use http::{Request, Response, method};

use crate::{AsyncHttpClient, HttpError};

/// Executes HTTP requests using a `reqwest::Client`.
///
/// This helper function handles the conversion between `http` types and `reqwest` types,
/// executes the request, and processes the response including error handling.
///
/// # Error Handling
///
/// - **Network errors**: Connection failures, timeouts, DNS errors → [`HttpError::NetworkError`]
/// - **4xx/5xx responses**: Parsed into structured [`crate::SchwabError`] → [`HttpError::Api`]
/// - **Malformed responses**: Invalid HTTP structure → [`HttpError::RequestFailed`]
///
/// # Arguments
///
/// * `client` - The reqwest client to use for the request
/// * `request` - The HTTP request to execute (using `http` crate types)
///
/// # Returns
///
/// * `Ok(Response<String>)` - Successful response (2xx status) with body as String
/// * `Err(HttpError)` - Any error during request execution or response processing
async fn execute_with_reqwest(
    client: &reqwest::Client,
    request: Request<String>,
) -> Result<Response<String>, HttpError> {
    // Deconstruct http::Request
    let (parts, body_ref) = request.into_parts();

    // Convert Method and URI
    let req_method = parts
        .method
        .as_str()
        .parse()
        .map_err(|e: method::InvalidMethod| {
            HttpError::RequestFailed(format!("Invalid HTTP method: {}", e))
        })?;

    let url = parts.uri.to_string();

    // Build reqwest request
    let mut rb = client.request(req_method, url);
    rb = rb.headers(parts.headers);

    if !body_ref.is_empty() {
        rb = rb.body(body_ref);
    }

    // Send request
    let resp = rb
        .send()
        .await
        .map_err(|e| HttpError::NetworkError(e.to_string()))?;

    let status = resp.status();
    let headers = resp.headers().clone();

    // Extract body text
    let body_text = resp
        .text()
        .await
        .map_err(|e| HttpError::NetworkError(format!("Failed to read response body: {}", e)))?;

    // Handle API failure using the helper function
    if !status.is_success() {
        return Err(HttpError::UnparsedApiError {
            status,
            body: body_text,
        });
    }

    // Build http::Response
    let mut builder = Response::builder().status(status);

    for (name, value) in headers.iter() {
        builder = builder.header(name, value);
    }

    let response = builder
        .body(body_text)
        .map_err(|e| HttpError::RequestFailed(format!("Failed to build final Response: {}", e)))?;

    Ok(response)
}

/// Implementation of [`AsyncHttpClient`] for owned `reqwest::Client`.
///
/// This allows you to pass a `reqwest::Client` directly to [`crate::ApiClient`].
///
/// # Example
///
/// ```ignore
/// use schwab_api_core::ApiClient;
/// let client = ApiClient::new(reqwest::Client::new());
/// ```
#[async_trait]
impl AsyncHttpClient for reqwest::Client {
    type Error = HttpError;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_reqwest(self, request).await
    }
}

/// Implementation of [`AsyncHttpClient`] for borrowed `reqwest::Client`.
///
/// This allows you to pass a reference to a `reqwest::Client`, useful when
/// the client is owned elsewhere and you want to avoid cloning.
///
/// # Example
///
/// ```ignore
/// use schwab_api_core::ApiClient;
/// let http_client = reqwest::Client::new();
/// let client = ApiClient::new(&http_client);
/// ```
#[async_trait]
impl AsyncHttpClient for &reqwest::Client {
    type Error = HttpError;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_reqwest(self, request).await
    }
}

/// Implementation of [`AsyncHttpClient`] for `Arc<reqwest::Client>`.
///
/// This allows you to share a `reqwest::Client` across multiple tasks or threads
/// using `Arc`, which is common in async applications.
///
/// # Example
///
/// ```ignore
/// use std::sync::Arc;
/// use schwab_api_core::ApiClient;
///
/// let http_client = Arc::new(reqwest::Client::new());
/// let client1 = ApiClient::new(http_client.clone());
/// let client2 = ApiClient::new(http_client.clone());
/// ```
#[async_trait]
impl AsyncHttpClient for std::sync::Arc<reqwest::Client> {
    type Error = HttpError;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_reqwest(self.as_ref(), request).await
    }
}
