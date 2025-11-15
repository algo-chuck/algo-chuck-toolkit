//! Error types for Schwab API interactions
//!
//! This module contains all error types used throughout the Schwab API client,
//! including API-specific errors and HTTP client errors.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(feature = "marketdata")]
use schwab_api_types::marketdata::ErrorResponse;
#[cfg(feature = "trader")]
use schwab_api_types::trader::ServiceError;

/// Convenient Result type alias using HttpError
pub type Result<T> = std::result::Result<T, HttpError>;

/// Errors returned by the Schwab API (parsed from non-success HTTP responses).
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabError {
    #[cfg(feature = "trader")]
    #[error("Trader API Error ({status}): {detail:#?}")]
    Trader { status: u16, detail: ServiceError },
    #[cfg(feature = "marketdata")]
    #[error("Marketdata API Error ({status}): {detail:#?}")]
    Marketdata { status: u16, detail: ErrorResponse },
    #[error("Unknown Schwab API response structure: {0}")]
    UnknownValue(serde_json::Value),
}

/// HTTP client errors that can occur during API requests.
#[derive(Error, Debug)]
pub enum HttpError {
    #[error("Request failed: {0}")]
    RequestFailed(String),

    #[error("Request to {method} {path} failed: {reason}")]
    RequestFailedWithContext {
        method: String,
        path: String,
        reason: String,
    },

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Schwab API Error: {0}")]
    Api(SchwabError),

    /// Unparsed API error that needs context from ApiClient to properly classify
    #[error("Unparsed API error (status {status})")]
    UnparsedApiError {
        status: http::StatusCode,
        body: String,
    },
}

/// Parse Schwab API errors from HTTP response body.
///
/// This function attempts to parse error responses from both the Trader API
/// and Market Data API based on the provided API name context.
///
/// # Arguments
///
/// * `status` - The HTTP status code from the response
/// * `body_text` - The response body as a string
/// * `api_name` - The name of the API being called ("trader" or "marketdata")
///
/// # Returns
///
/// A `SchwabError` representing the parsed error, or an unknown error if
/// parsing fails.
pub fn parse_api_error(status: http::StatusCode, body_text: &str, api_name: &str) -> SchwabError {
    let status_code = status.as_u16();

    // Use the API name to determine which error structure to parse first
    match api_name {
        #[cfg(feature = "trader")]
        "trader" => {
            // Try Trader API error first
            if let Ok(se) = serde_json::from_str::<ServiceError>(body_text) {
                return SchwabError::Trader {
                    status: status_code,
                    detail: se,
                };
            }
        }
        #[cfg(feature = "marketdata")]
        "marketdata" => {
            // Try Marketdata API error first
            if let Ok(me) = serde_json::from_str::<ErrorResponse>(body_text) {
                return SchwabError::Marketdata {
                    status: status_code,
                    detail: me,
                };
            }
        }
        _ => {}
    }

    // Fallback: Try the other API type in case of misconfiguration
    #[cfg(feature = "trader")]
    if let Ok(se) = serde_json::from_str::<ServiceError>(body_text) {
        return SchwabError::Trader {
            status: status_code,
            detail: se,
        };
    }

    #[cfg(feature = "marketdata")]
    if let Ok(me) = serde_json::from_str::<ErrorResponse>(body_text) {
        return SchwabError::Marketdata {
            status: status_code,
            detail: me,
        };
    }

    // If structured parsing fails, assume it's an unknown/unstructured error.
    // Try to parse it as generic JSON value for better debugging output.
    match serde_json::from_str::<serde_json::Value>(body_text) {
        Ok(v) => SchwabError::UnknownValue(v),

        // Fallback: If it's not even valid JSON, just store the raw text.
        Err(_) => SchwabError::UnknownValue(serde_json::Value::String(format!(
            "Raw text (status {status_code}): {}",
            body_text
        ))),
    }
}
