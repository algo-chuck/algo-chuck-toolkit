use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use schwab_api_types::ServiceError;

/// Errors returned by the Trader API (parsed from non-success HTTP responses).
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabTraderError {
    #[error("400: {0:?}")]
    Status400(ServiceError),
    #[error("401: {0:?}")]
    Status401(ServiceError),
    #[error("403: {0:?}")]
    Status403(ServiceError),
    #[error("404: {0:?}")]
    Status404(ServiceError),
    #[error("500: {0:?}")]
    Status500(ServiceError),
    #[error("503: {0:?}")]
    Status503(ServiceError),
    #[error("Unknown value: {0}")]
    UnknownValue(serde_json::Value),
}

#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestFailed(String),
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    NetworkError(String),
    #[error("API error: {0}")]
    Api(SchwabTraderError),
}

// Re-exported `HttpMethod` above for downstream consumers.
pub type HttpMethod = http::Method;

// Use the standard `http::Request<String>` as our public request type.
pub type Request = http::Request<String>;

// Use the standard `http::Response<String>` as our public response type.
pub type Response = http::Response<String>;

/// Small extension trait for `HttpResponse` to keep caller code concise.
pub trait HttpResponse {
    fn body_str(&self) -> &str;
    fn json<T: DeserializeOwned>(&self) -> Result<T, HttpError>;
    fn is_success(&self) -> bool;
}

impl HttpResponse for Response {
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
pub trait AsyncClient: Send + Sync {
    async fn execute(&self, request: Request) -> Result<Response, HttpError>;
}

impl<T: AsyncClient> HttpClient<T> {
    pub async fn execute(&self, request: Request) -> Result<Response, HttpError> {
        self.client.execute(request).await
    }
}

// Provide an implementation of AsyncClient for reqwest::Client so the HTTP
// adapter can be used directly with the higher-level clients.
#[async_trait]
impl AsyncClient for reqwest::Client {
    async fn execute(&self, request: Request) -> Result<Response, HttpError> {
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

        let status = resp.status();
        let mut builder = http::Response::builder().status(status);
        for (name, value) in resp.headers().iter() {
            builder = builder.header(name, value.to_str().unwrap_or_default());
        }

        let body_text = resp
            .text()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        if !status.is_success() {
            // Try parse ServiceError -> SchwabTraderError, fallback to JSON or raw string
            let parsed = match serde_json::from_str::<ServiceError>(&body_text) {
                Ok(se) => match status.as_u16() {
                    400 => SchwabTraderError::Status400(se),
                    401 => SchwabTraderError::Status401(se),
                    403 => SchwabTraderError::Status403(se),
                    404 => SchwabTraderError::Status404(se),
                    500 => SchwabTraderError::Status500(se),
                    503 => SchwabTraderError::Status503(se),
                    _ => SchwabTraderError::UnknownValue(serde_json::Value::String(
                        body_text.clone(),
                    )),
                },
                Err(_) => match serde_json::from_str::<serde_json::Value>(&body_text) {
                    Ok(v) => SchwabTraderError::UnknownValue(v),
                    Err(_) => SchwabTraderError::UnknownValue(serde_json::Value::String(
                        body_text.clone(),
                    )),
                },
            };

            return Err(HttpError::Api(parsed));
        }

        let response = builder
            .body(body_text)
            .map_err(|e| HttpError::RequestFailed(format!("failed to build response: {}", e)))?;

        Ok(response)
    }
}
