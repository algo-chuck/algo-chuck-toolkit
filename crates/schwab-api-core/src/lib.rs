use async_trait::async_trait;
pub use http::Method as HttpMethod;
pub use http::header::HeaderName;
use http::{Request, Response};
use serde::de::DeserializeOwned;
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

// --- Convenience constructors and helpers to keep caller code concise ---

/// Build a simple GET `HttpRequest` for the given URL.
pub fn request_get(url: impl Into<String>) -> HttpRequest {
    let req = HttpRequest::new(String::new());
    let (mut parts, body) = req.into_parts();
    parts.method = HttpMethod::GET;
    parts.uri = url.into().parse().expect("invalid url");
    HttpRequest::from_parts(parts, body)
}

/// Build an `HttpRequest` with a Bearer Authorization header.
/// `token` should be the raw token (the function adds the "Bearer " prefix).
pub fn request_with_bearer(method: HttpMethod, url: impl Into<String>, token: &str) -> HttpRequest {
    let req = HttpRequest::new(String::new());
    let (mut parts, body) = req.into_parts();
    parts.method = method;
    parts.uri = url.into().parse().expect("invalid url");
    parts.headers.insert(
        HeaderName::from_static("authorization"),
        format!("Bearer {}", token)
            .parse()
            .expect("invalid header value"),
    );
    HttpRequest::from_parts(parts, body)
}

/// Build an `HttpRequest` with a JSON body and Content-Type header.
pub fn request_json<T: Serialize>(
    method: HttpMethod,
    url: impl Into<String>,
    body: &T,
) -> Result<HttpRequest, HttpError> {
    let body_str = serde_json::to_string(body)?;
    let req = HttpRequest::new(String::new());
    let (mut parts, _old_body) = req.into_parts();
    parts.method = method;
    parts.uri = url.into().parse().expect("invalid url");
    parts.headers.insert(
        HeaderName::from_static("content-type"),
        "application/json".parse().unwrap(),
    );
    Ok(HttpRequest::from_parts(parts, body_str))
}

/// Small extension trait for `HttpResponse` to keep caller code concise.
pub trait HttpResponseExt {
    fn body_str(&self) -> &str;
    fn json<T: DeserializeOwned>(&self) -> Result<T, HttpError>;
    fn is_success(&self) -> bool;
}

impl HttpResponseExt for HttpResponse {
    fn body_str(&self) -> &str {
        self.body()
    }

    fn json<T: DeserializeOwned>(&self) -> Result<T, HttpError> {
        serde_json::from_str(self.body()).map_err(HttpError::SerializationError)
    }

    fn is_success(&self) -> bool {
        (200..300).contains(&self.status().as_u16())
    }
}

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
