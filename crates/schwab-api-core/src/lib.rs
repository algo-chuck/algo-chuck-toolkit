use async_trait::async_trait;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use schwab_api_types::ServiceError;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabSuccess<T> {
    // Tries to parse the expected struct T first.
    Ok(T),
    // If the data doesn't match T, it falls back to capturing the raw JSON
    MismatchedResponse(serde_json::Value),
}

/// Errors returned by the Trader API (parsed from non-success HTTP responses).
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabError {
    #[error("Trader API Error ({status}): {detail:#?}")]
    Trader {
        status: u16, // Add the status code here
        detail: ServiceError,
    },
    #[error("Unknown Schwab API response structure: {0}")]
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
    #[error("Schwab Error: {0}")]
    Api(SchwabError),
}

// Re-exported `HttpMethod` above for downstream consumers.
pub type HttpMethod = http::Method;

// Use the standard `http::Request<String>` as our public request type.
pub type Request = http::Request<String>;

// Use the standard `http::Response<String>` as our public response type.
pub type Response = http::Response<String>;

/// Small extension trait for `HttpResponse` to keep caller code concise.
pub trait HttpResponse {
    type ParsingError: std::error::Error + Send + Sync + 'static;

    fn body_str(&self) -> &str;

    fn json<T: DeserializeOwned>(&self) -> Result<T, Self::ParsingError>;

    fn is_success(&self) -> bool;
}

impl HttpResponse for Response {
    type ParsingError = serde_json::Error;

    fn body_str(&self) -> &str {
        self.body()
    }

    fn json<T: DeserializeOwned>(&self) -> Result<T, Self::ParsingError> {
        serde_json::from_str(self.body())
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
    type Error: std::error::Error + Send + Sync + 'static;

    async fn execute(&self, request: Request) -> Result<Response, Self::Error>;

    // Associated function for common error parsing with a default implementation
    fn parse_api_error(status: http::StatusCode, body_text: &str) -> SchwabError {
        let status_code = status.as_u16();

        // Attempt 1: Try to parse the body as the expected structured ServiceError.
        match serde_json::from_str::<ServiceError>(&body_text) {
            Ok(se) => {
                // If parsing is successful, wrap the error and the status code
                SchwabError::Trader {
                    status: status_code,
                    detail: se,
                }
            }
            // Attempt 2: If structured parsing fails, assume it's an unknown/unstructured error.
            Err(_) => {
                // Try to parse it as generic JSON value for better debugging output.
                match serde_json::from_str::<serde_json::Value>(&body_text) {
                    Ok(v) => SchwabError::UnknownValue(v),

                    // Fallback: If it's not even valid JSON, just store the raw text.
                    Err(_) => SchwabError::UnknownValue(serde_json::Value::String(format!(
                        "Raw text (status {status_code}): {}",
                        body_text
                    ))),
                }
            }
        }
    }
}

impl<T: AsyncClient> HttpClient<T> {
    pub async fn execute(&self, request: Request) -> Result<Response, T::Error> {
        self.client.execute(request).await
    }
}

// Provide an implementation of AsyncClient for reqwest::Client so the HTTP
// adapter can be used directly with the higher-level clients.
#[async_trait]
impl AsyncClient for reqwest::Client {
    type Error = HttpError;

    async fn execute(&self, request: Request) -> Result<Response, Self::Error> {
        // --- 1. Safely convert http::Request fields to reqwest::Request parts ---

        // Deconstruct Request (assuming it's http::Request<String> or similar)
        let (parts, body_ref) = request.into_parts(); // Assuming Request has into_parts()

        // Safely convert Method and URI
        let req_method =
            parts
                .method
                .as_str()
                .parse()
                .map_err(|e: http::method::InvalidMethod| {
                    HttpError::RequestFailed(format!("Invalid HTTP method: {}", e))
                })?;

        let url = parts.uri.to_string();

        // Start building the reqwest request
        let mut rb = self.request(req_method, url);

        // Add headers from http::HeaderMap
        // reqwest::RequestBuilder::headers() takes HeaderMap, allowing zero-copy or efficient copy.
        // If your request.headers() returns &HeaderMap, use clone() if ownership is required later.
        // Assuming parts.headers is HeaderMap:
        rb = rb.headers(parts.headers);

        // Add body: Convert String body (body_ref) into reqwest's body type (Bytes or String)
        if !body_ref.is_empty() {
            // Using into_inner() to take ownership of the inner String is best for performance,
            // avoiding an unnecessary clone() if the inner type is String.
            rb = rb.body(body_ref);
        }

        // --- 2. Send request and handle network errors ---

        let resp = rb
            .send()
            .await
            .map_err(|e| HttpError::NetworkError(e.to_string()))?; // Use reqwest::Error::to_string()

        let status = resp.status();
        let headers = resp.headers().clone();

        // --- 3. Extract body text and handle API errors ---

        // We read the body into a String regardless of success/failure,
        // as we need the body text for error parsing or successful response.
        let body_text = resp
            .text()
            .await
            .map_err(|e| HttpError::NetworkError(format!("Failed to read response body: {}", e)))?;

        // Handle API failure using the extracted helper function
        if !status.is_success() {
            let parsed = Self::parse_api_error(status, &body_text);
            return Err(HttpError::Api(parsed));
        }

        // --- 4. Build and return the high-level Response ---

        // Reuse parts.headers as reqwest::Response::headers() is less robust than the builder pattern.
        let mut builder = http::Response::builder().status(status);

        // Get headers from the reqwest response and copy/convert them
        for (name, value) in headers.iter() {
            // reqwest::header::HeaderName and http::header::HeaderName are the same type.
            builder = builder.header(name, value);
        }

        // Build the final Response<String>
        let response = builder.body(body_text).map_err(|e| {
            HttpError::RequestFailed(format!("Failed to build final Response: {}", e))
        })?;

        Ok(response)
    }
}
