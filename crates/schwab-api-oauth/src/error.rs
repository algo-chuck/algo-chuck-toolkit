use thiserror::Error;

/// Errors that can occur during OAuth operations
#[derive(Error, Debug)]
pub enum OAuthError {
    #[error("HTTP request failed: {0}")]
    RequestFailed(String),

    #[error("Token request failed with status {status}: {body}")]
    TokenRequestFailed { status: u16, body: String },

    #[error("Failed to parse URL: {0}")]
    UrlParseFailed(#[from] url::ParseError),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
}

/// Convenience Result type for OAuth operations
pub type Result<T> = std::result::Result<T, OAuthError>;
