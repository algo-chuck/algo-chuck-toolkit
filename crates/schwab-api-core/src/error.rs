//! Error types for Schwab API interactions
//!
//! This module contains all error types used throughout the Schwab API client,
//! including API-specific errors and HTTP client errors.

use serde::{Deserialize, Serialize};
use thiserror::Error;

use schwab_api_types::ServiceError;
use schwab_api_types::marketdata::ErrorResponse;

/// Errors returned by the Schwab API (parsed from non-success HTTP responses).
#[derive(Error, Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SchwabError {
    #[error("Trader API Error ({status}): {detail:#?}")]
    Trader { status: u16, detail: ServiceError },
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
}

/// Parse Schwab API errors from HTTP response body.
///
/// This function attempts to parse error responses from both the Trader API
/// and Market Data API, falling back to a generic error representation if
/// the structure doesn't match either format.
///
/// # Arguments
///
/// * `status` - The HTTP status code from the response
/// * `body_text` - The response body as a string
///
/// # Returns
///
/// A `SchwabError` representing the parsed error, or an unknown error if
/// parsing fails.
pub fn parse_api_error(status: http::StatusCode, body_text: &str) -> SchwabError {
    let status_code = status.as_u16();

    // Attempt 1: Try to parse the body as the Marketdata API structured ErrorResponse.
    if let Ok(me) = serde_json::from_str::<ErrorResponse>(body_text) {
        return SchwabError::Marketdata {
            status: status_code,
            detail: me,
        };
    }

    // Attempt 2: Try to parse the body as the Trader API structured ServiceError.
    match serde_json::from_str::<ServiceError>(body_text) {
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
            match serde_json::from_str::<serde_json::Value>(body_text) {
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
