use async_trait::async_trait;
use http::{Request, Response};
use serde::de::{DeserializeOwned, Error as DeError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use schwab_api_types::ServiceError;
use schwab_api_types::marketdata::ErrorResponse;

// Feature-gated HTTP client implementations
#[cfg(feature = "reqwest-client")]
mod reqwest_client;

#[cfg(feature = "ureq-client")]
mod ureq_client;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabSuccess<J> {
    // Tries to parse the expected struct J first.
    Ok(J),
    // If the data doesn't match J, it falls back to capturing the raw J
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
    #[error("Marketdata API Error ({status}): {detail:#?}")]
    Marketdata { status: u16, detail: ErrorResponse },
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

#[derive(Debug)]
pub struct RequestParams<'a, B = ()> {
    pub access_token: &'a str,
    pub body: Option<B>,
    pub method: http::Method,
    pub path: String,
    pub query: Option<String>,
}

/// Small extension trait for `HttpResponse` to keep caller code concise.
pub trait HttpResponse {
    type ParsingError: std::error::Error + Send + Sync + 'static;

    fn body_str(&self) -> &str;

    fn json<J: DeserializeOwned>(&self) -> Result<J, Self::ParsingError>;

    fn is_success(&self) -> bool;
}

impl HttpResponse for Response<String> {
    type ParsingError = serde_json::Error;

    fn body_str(&self) -> &str {
        self.body()
    }

    fn json<J: DeserializeOwned>(&self) -> Result<J, Self::ParsingError> {
        serde_json::from_str(self.body())
    }

    fn is_success(&self) -> bool {
        (200..300).contains(&self.status().as_u16())
    }
}

// Generic HTTP client that can work with either sync or async implementations
pub struct HttpClient<C> {
    client: C,
}

impl<C> HttpClient<C> {
    pub fn new(client: C) -> Self {
        Self { client }
    }

    /// Get a reference to the inner client
    pub fn inner(&self) -> &C {
        &self.client
    }
}

/// Trait for async HTTP clients
#[async_trait]
pub trait AsyncHttpClient: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    async fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error>;
}

/// Trait for sync/blocking HTTP clients
pub trait SyncHttpClient: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;

    fn execute(&self, request: Request<String>) -> Result<Response<String>, Self::Error>;
}

// Helper function for parsing Schwab API errors from response body
pub fn parse_api_error(status: http::StatusCode, body_text: &str) -> SchwabError {
    let status_code = status.as_u16();

    // Attempt 1: Try to parse the body as the Marketdata API structured ErrorResponse.
    if let Ok(me) = serde_json::from_str::<ErrorResponse>(&body_text) {
        return SchwabError::Marketdata {
            status: status_code,
            detail: me,
        };
    }

    // Attempt 2: Try to parse the body as the Trader API structured ServiceError.
    match serde_json::from_str::<ServiceError>(&body_text) {
        Ok(se) => {
            // If parsing is successful, wrap the error and the status code
            SchwabError::Trader {
                status: status_code,
                detail: se,
            }
        }
        // Attempt 3: If structured parsing fails, assume it's an unknown/unstructured error.
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

impl<C: AsyncHttpClient> HttpClient<C> {
    pub async fn execute(&self, request: Request<String>) -> Result<Response<String>, C::Error> {
        self.client.execute(request).await
    }
}

impl<C, Cfg> ApiClient<C, Cfg>
where
    C: AsyncHttpClient,
    Cfg: ApiConfig,
    HttpError: From<C::Error>,
{
    /// Helper method to fetch and deserialize a response
    pub async fn fetch<'a, R, B>(&self, params: &'a RequestParams<'a, B>) -> Result<R, HttpError>
    where
        R: DeserializeOwned,
        B: Serialize,
    {
        let request = self.build_request(params)?;

        // Success path continues immediately.
        let response = self
            .client
            .execute(request)
            .await
            // Use HttpError::from to explicitly tell the compiler the target type.
            .map_err(HttpError::from)?;

        // Use the single helper method to handle deserialization, logging, and error conversion.
        let typed = self.parse_ok_response(&response)?;
        Ok(typed)
    }

    /// Helper method to execute a request without parsing a response body
    pub async fn execute<'a, B>(&self, params: &'a RequestParams<'a, B>) -> Result<(), HttpError>
    where
        B: Serialize,
    {
        let request = self.build_request(params)?;

        self.client
            .execute(request)
            .await
            .map_err(HttpError::from)?;

        Ok(())
    }
}

/// Configuration trait for API-specific clients (Trader, Marketdata, etc.)
pub trait ApiConfig {
    /// Base URL for this API (e.g., "https://api.schwabapi.com/trader/v1")
    fn base_url() -> &'static str;
}

/// Generic API client that works with any API configuration
pub struct ApiClient<C, Cfg: ApiConfig> {
    pub client: HttpClient<C>,
    _config: std::marker::PhantomData<Cfg>,
}

impl<C, Cfg: ApiConfig> ApiClient<C, Cfg> {
    pub fn new(client: C) -> Self {
        Self {
            client: HttpClient::new(client),
            _config: std::marker::PhantomData,
        }
    }

    fn build_url(&self, path: &str, query_string_opt: Option<&str>) -> String {
        let query_prefix = match query_string_opt {
            // Check if the string exists and is not empty.
            Some(value) if !value.is_empty() => format!("?{value}"),
            // If it's None or empty, use an empty string.
            _ => String::new(),
        };

        // Combine base URL, path, and the (optionally prefixed) query string.
        format!("{}{}{}", Cfg::base_url(), path, query_prefix)
    }

    pub fn build_request<B: Serialize>(
        &self,
        params: &RequestParams<B>,
    ) -> Result<Request<String>, HttpError> {
        let url = self.build_url(&params.path, params.query.as_deref());
        let bearer_token = format!("Bearer {}", params.access_token);

        // Serialize the body if present
        let final_body = match &params.body {
            Some(body) => serde_json::to_string(body)?,
            None => String::new(),
        };

        Request::builder()
            .uri(url)
            .method(params.method.clone())
            .header("Authorization", bearer_token)
            // .header("Content-Type", "application/json") // Causing 400 error, need to fix for POST
            .body(final_body)
            // The request building error (http::Error) is handled explicitly
            // by mapping it to an appropriate HttpError variant, avoiding the need for
            // a global From<http::Error> implementation.
            .map_err(|e| HttpError::RequestFailed(format!("Request builder failed: {}", e)))
    }

    // This method performs the robust deserialization, logging, and error conversion.
    pub fn parse_ok_response<R: DeserializeOwned>(
        &self,
        response: &Response<String>,
    ) -> Result<R, HttpError> {
        // Perform robust parsing into the GENERIC wrapper type.
        let ok_result = response.json()?;

        // Inspect the result, log the anomaly, and return the final type.
        match ok_result {
            SchwabSuccess::Ok(data) => Ok(data),
            SchwabSuccess::MismatchedResponse(value) => {
                // Log the anomaly: API returned 2xx, but structure was mismatched.
                eprintln!(
                    "WARNING: API returned status {}, but response body was mismatched:\n {:#?}",
                    response.status(),
                    value
                );

                // Treat the unexpected structure as a serialization failure.
                Err(HttpError::SerializationError(
                    // Generate a serde_json error object detailing the issue.
                    DeError::custom(format!(
                        "Received mismatched {} response structure:\n {:#?}",
                        response.status(),
                        value
                    )),
                ))
            }
        }
    }
}
