//! Response types and traits for handling API responses.

use http::Response;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

/// Wrapper for successful API responses with robust error handling.
///
/// This enum handles two cases:
/// 1. The response body matches the expected structure (Ok variant)
/// 2. The response body doesn't match, but we capture it for debugging (MismatchedResponse)
///
/// This is particularly useful for handling API changes or unexpected response formats
/// without crashing the application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabSuccess<J> {
    /// The response successfully parsed into the expected type
    Ok(J),

    /// The response didn't match the expected structure, but was captured for debugging
    MismatchedResponse(serde_json::Value),
}

/// Extension trait for `http::Response` to add convenient parsing methods.
///
/// This trait provides ergonomic methods for working with HTTP responses,
/// particularly for JSON deserialization and status code checking.
pub trait HttpResponse {
    /// The error type returned by parsing operations
    type ParsingError: std::error::Error + Send + Sync + 'static;

    /// Get the response body as a string slice
    fn body_str(&self) -> &str;

    /// Parse the response body as JSON into the specified type
    ///
    /// # Errors
    ///
    /// Returns an error if the body is not valid JSON or doesn't match the expected type
    fn json<J: DeserializeOwned>(&self) -> Result<J, Self::ParsingError>;

    /// Check if the response status code indicates success (2xx)
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
