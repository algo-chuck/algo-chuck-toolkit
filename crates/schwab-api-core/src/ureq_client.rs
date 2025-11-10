//! Sync/blocking HTTP client implementation using ureq.
//!
//! This module provides [`SyncHttpClient`] trait implementations for `ureq` types,
//! enabling synchronous/blocking HTTP requests to the Schwab API.
//!
//! # Supported Types
//!
//! This module implements [`SyncHttpClient`] for:
//! - `ureq::Agent` (owned)
//! - `&ureq::Agent` (borrowed reference)
//! - `Arc<ureq::Agent>` (thread-safe shared reference)
//!
//! # Error Handling
//!
//! All implementations automatically:
//! - Parse Schwab API error responses (4xx/5xx) into structured [`crate::SchwabError`] types
//! - Convert network/transport errors into [`HttpError::NetworkError`]
//! - Handle malformed responses gracefully
//!
//! # Examples
//!
//! ```ignore
//! use schwab_api_core::{ApiClient, ApiConfig};
//! use ureq::Agent;
//!
//! // Define your API configuration
//! struct MyApiConfig;
//! impl ApiConfig for MyApiConfig {
//!     fn base_url() -> &'static str {
//!         "https://api.schwabapi.com/trader/v1"
//!     }
//! }
//!
//! // Create client with owned ureq::Agent
//! let http_client = Agent::new();
//! let api_client = ApiClient::<_, MyApiConfig>::new(http_client);
//!
//! // Or use Arc for sharing across threads
//! let http_client = std::sync::Arc::new(Agent::new());
//! let api_client = ApiClient::<_, MyApiConfig>::new(http_client);
//! ```

use http::{Request, Response};

use crate::{HttpError, SyncHttpClient};

/// Executes HTTP requests using a `ureq::Agent`.
///
/// This helper function handles the conversion between `http` types and `ureq` types,
/// executes the request synchronously, and processes the response including error handling.
///
/// # Error Handling
///
/// - **Transport errors**: Connection failures, timeouts, DNS errors → [`HttpError::NetworkError`]
/// - **4xx/5xx responses**: Parsed into structured [`crate::SchwabError`] → [`HttpError::Api`]
/// - **Malformed responses**: Invalid HTTP structure → [`HttpError::RequestFailed`]
///
/// # Arguments
///
/// * `agent` - The ureq agent to use for the request
/// * `request` - The HTTP request to execute (using `http` crate types)
///
/// # Returns
///
/// * `Ok(Response<String>)` - Successful response (2xx status) with body as String
/// * `Err(HttpError)` - Any error during request execution or response processing
fn execute_with_ureq(
    agent: &ureq::Agent,
    request: Request<String>,
) -> Result<Response<String>, HttpError> {
    // Deconstruct http::Request
    let (parts, body_ref) = request.into_parts();

    // Convert Method and URI
    let method = parts.method.as_str();
    let url = parts.uri.to_string();

    // Build ureq request
    let mut req = agent.request(method, &url);

    // Add headers
    for (name, value) in parts.headers.iter() {
        if let Ok(value_str) = value.to_str() {
            req = req.set(name.as_str(), value_str);
        }
    }

    // Send request with optional body
    let response = if !body_ref.is_empty() {
        req.send_string(&body_ref)
    } else {
        req.call()
    };

    // Handle ureq::Error which can be network error or HTTP error
    let resp = match response {
        Ok(resp) => resp,
        Err(ureq::Error::Status(code, resp)) => {
            // HTTP error status (4xx, 5xx) - still has response body
            let body_text = resp.into_string().map_err(|e| {
                HttpError::NetworkError(format!("Failed to read error response body: {}", e))
            })?;

            // Parse the API error using the centralized error parser
            let status =
                http::StatusCode::from_u16(code).unwrap_or(http::StatusCode::INTERNAL_SERVER_ERROR);
            let parsed = crate::parse_api_error(status, &body_text);
            return Err(HttpError::Api(parsed));
        }
        Err(ureq::Error::Transport(e)) => {
            // Network/transport error
            return Err(HttpError::NetworkError(e.to_string()));
        }
    };

    // Extract status and headers
    let status = resp.status();
    let mut builder = http::Response::builder().status(status);

    // Copy headers
    for header_name in resp.headers_names() {
        if let Some(header_value) = resp.header(&header_name) {
            builder = builder.header(header_name.as_str(), header_value);
        }
    }

    // Extract body
    let body_text = resp
        .into_string()
        .map_err(|e| HttpError::NetworkError(format!("Failed to read response body: {}", e)))?;

    // Build http::Response
    let response = builder
        .body(body_text)
        .map_err(|e| HttpError::RequestFailed(format!("Failed to build final Response: {}", e)))?;

    Ok(response)
}

/// Implementation of [`SyncHttpClient`] for owned `ureq::Agent`.
///
/// This allows you to pass a `ureq::Agent` directly to [`crate::ApiClient`].
///
/// # Example
///
/// ```ignore
/// use schwab_api_core::ApiClient;
/// let client = ApiClient::new(ureq::Agent::new());
/// ```
impl SyncHttpClient for ureq::Agent {
    type Error = HttpError;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_ureq(self, request)
    }
}

/// Implementation of [`SyncHttpClient`] for borrowed `ureq::Agent`.
///
/// This allows you to pass a reference to a `ureq::Agent`, useful when
/// the agent is owned elsewhere and you want to avoid cloning.
///
/// # Example
///
/// ```ignore
/// use schwab_api_core::ApiClient;
/// let http_client = ureq::Agent::new();
/// let client = ApiClient::new(&http_client);
/// ```
impl SyncHttpClient for &ureq::Agent {
    type Error = HttpError;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_ureq(self, request)
    }
}

/// Implementation of [`SyncHttpClient`] for `Arc<ureq::Agent>`.
///
/// This allows you to share a `ureq::Agent` across multiple threads
/// using `Arc`, which is useful in multi-threaded applications.
///
/// # Example
///
/// ```ignore
/// use std::sync::Arc;
/// use schwab_api_core::ApiClient;
///
/// let http_client = Arc::new(ureq::Agent::new());
/// let client1 = ApiClient::new(http_client.clone());
/// let client2 = ApiClient::new(http_client.clone());
/// ```
impl SyncHttpClient for std::sync::Arc<ureq::Agent> {
    type Error = HttpError;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_ureq(self.as_ref(), request)
    }
}
