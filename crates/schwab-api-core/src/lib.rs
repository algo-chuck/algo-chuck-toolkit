use async_trait::async_trait;
pub use http::Method as HttpMethod;
pub use http::header::HeaderName;
use http::{Request, Response};
pub use serde::{Deserialize, Serialize};
pub use std::collections::HashMap;
pub use thiserror::Error;

// Error types for our HTTP client
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    NetworkError(String),
}

// Re-exported `HttpMethod` above for downstream consumers.

// Use the standard `http::Request<String>` as our public request type.
pub type HttpRequest = Request<String>;

// Use the standard `http::Response<String>` as our public response type.
pub type HttpResponse = Response<String>;

// Generic HTTP client that can work with either sync or async implementations
pub struct HttpClient<T> {
    client: T,
}

impl<T> HttpClient<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
}

#[async_trait]
pub trait AsyncClient {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, HttpError>;
}

impl<T: AsyncClient> HttpClient<T> {
    pub async fn execute_async(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        self.client.execute(request).await
    }
}

// Provide an implementation of AsyncClient for reqwest::Client so the HTTP
// adapter can be used directly with the higher-level clients.
#[async_trait]
impl AsyncClient for reqwest::Client {
    async fn execute(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        // Build the reqwest request based on our HttpRequest wrapper.
        // Convert http::Method -> reqwest::Method and use the uri from the request.
        let req_method = match reqwest::Method::from_bytes(request.method().as_str().as_bytes()) {
            Ok(m) => m,
            Err(e) => return Err(HttpError::RequestFailed(format!("invalid method: {}", e))),
        };

        // Use the request URI as a string
        let url = request.uri().to_string();
        let mut rb = self.request(req_method, url);

        // Add headers from the http::HeaderMap
        for (name, value) in request.headers().iter() {
            let v = value.to_str().unwrap_or_default();
            rb = rb.header(name.as_str(), v);
        }

        // Add body if non-empty (http::Request::body returns &String)
        let body_ref = request.body();
        if !body_ref.is_empty() {
            rb = rb.body(body_ref.clone());
        }

        let resp = rb
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        let mut builder = Response::builder().status(resp.status());
        for (name, value) in resp.headers().iter() {
            builder = builder.header(name, value.to_str().unwrap_or_default());
        }

        let body_text = resp
            .text()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        let response = builder
            .body(body_text)
            .map_err(|e| HttpError::RequestFailed(format!("failed to build response: {}", e)))?;

        Ok(response)
    }
}
