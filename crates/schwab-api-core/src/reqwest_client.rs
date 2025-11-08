//! Async HTTP client implementation using reqwest
//!
//! This module provides `AsyncHttpClient` implementations for reqwest types.

use async_trait::async_trait;
use http::{Request, Response};

use crate::{AsyncHttpClient, HttpError};

/// Helper function to execute HTTP requests with reqwest::Client
/// Shared by all AsyncHttpClient implementations for reqwest
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
        .map_err(|e: http::method::InvalidMethod| {
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
        let parsed = crate::parse_api_error(status, &body_text);
        return Err(HttpError::Api(parsed));
    }

    // Build http::Response
    let mut builder = http::Response::builder().status(status);

    for (name, value) in headers.iter() {
        builder = builder.header(name, value);
    }

    let response = builder
        .body(body_text)
        .map_err(|e| HttpError::RequestFailed(format!("Failed to build final Response: {}", e)))?;

    Ok(response)
}

// AsyncHttpClient for owned reqwest::Client
#[async_trait]
impl AsyncHttpClient for reqwest::Client {
    type Error = HttpError;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_reqwest(self, request).await
    }
}

// AsyncHttpClient for borrowed reqwest::Client
#[async_trait]
impl AsyncHttpClient for &reqwest::Client {
    type Error = HttpError;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_reqwest(self, request).await
    }
}

// AsyncHttpClient for Arc-wrapped reqwest::Client
#[async_trait]
impl AsyncHttpClient for std::sync::Arc<reqwest::Client> {
    type Error = HttpError;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error> {
        execute_with_reqwest(self.as_ref(), request).await
    }
}
