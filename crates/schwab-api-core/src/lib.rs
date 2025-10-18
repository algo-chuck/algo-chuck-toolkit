use async_trait::async_trait;
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

// HTTP Methods
#[derive(Debug, Clone)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
}

// Request structure
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub url: String,
    pub method: HttpMethod,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
}

impl HttpRequest {
    pub fn new(method: HttpMethod, url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            method,
            headers: HashMap::new(),
            body: None,
        }
    }

    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    pub fn json_body<T: Serialize>(mut self, body: &T) -> Result<Self, HttpError> {
        self.body = Some(serde_json::to_string(body)?);
        self.headers
            .insert("Content-Type".to_string(), "application/json".to_string());
        Ok(self)
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
}

// Response structure
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    pub fn json<T: for<'de> Deserialize<'de>>(&self) -> Result<T, HttpError> {
        Ok(serde_json::from_str(&self.body)?)
    }

    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
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

pub trait SyncClient {
    fn execute(&self, request: HttpRequest) -> Result<HttpResponse, HttpError>;
}

impl<T: SyncClient> HttpClient<T> {
    pub fn execute_sync(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        self.client.execute(request)
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
        // Build the reqwest request based on our HttpRequest
        let builder = match request.method {
            HttpMethod::Get => self.get(&request.url),
            HttpMethod::Post => self.post(&request.url),
            HttpMethod::Put => self.put(&request.url),
            HttpMethod::Delete => self.delete(&request.url),
            HttpMethod::Patch => self.patch(&request.url),
        };

        // Add headers
        let mut rb = builder;
        for (k, v) in request.headers.iter() {
            rb = rb.header(k, v);
        }

        // Add body if present
        let rb = match request.body {
            Some(ref body) => rb.body(body.clone()),
            None => rb,
        };

        let resp = rb
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        let status = resp.status().as_u16();
        let mut headers_map = HashMap::new();
        for (name, value) in resp.headers().iter() {
            headers_map.insert(
                name.to_string(),
                value.to_str().unwrap_or_default().to_string(),
            );
        }

        let body_text = resp
            .text()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?;

        Ok(HttpResponse {
            status,
            headers: headers_map,
            body: body_text,
        })
    }
}
