//! Sync HTTP client implementation using ureq
//!
//! This module provides `SyncHttpClient` implementations for ureq types.

use http::{Request, Response};

use crate::{HttpError, SyncHttpClient};

/// Helper function to execute HTTP requests with ureq::Agent
/// Shared by all SyncHttpClient implementations for ureq
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
            // HTTP error status (4xx, 5xx) - still has response
            // We need to extract body and build Response
            let body_text = resp.into_string().map_err(|e| {
                HttpError::NetworkError(format!("Failed to read error response body: {}", e))
            })?;

            let builder = http::Response::builder().status(code);

            let response = builder.body(body_text).map_err(|e| {
                HttpError::RequestFailed(format!("Failed to build error Response: {}", e))
            })?;

            return Ok(response);
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

// SyncHttpClient for owned ureq::Agent
impl SyncHttpClient for ureq::Agent {
    type Error = HttpError;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_ureq(self, request)
    }
}

// SyncHttpClient for borrowed ureq::Agent
impl SyncHttpClient for &ureq::Agent {
    type Error = HttpError;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_ureq(self, request)
    }
}

// SyncHttpClient for Arc-wrapped ureq::Agent
impl SyncHttpClient for std::sync::Arc<ureq::Agent> {
    type Error = HttpError;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_ureq(self.as_ref(), request)
    }
}
