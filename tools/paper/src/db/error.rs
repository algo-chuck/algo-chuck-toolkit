// db/error.rs
// Unified error type for all repository operations
//
// Maps database and serialization errors to the standard HTTP error codes
// defined in the OpenAPI spec (400, 401, 403, 404, 500, 503)

use serde_json;
use sqlx;

/// Unified repository error type
///
/// All repositories use this error type, which maps to the standard
/// HTTP error responses defined in api-spec-trader.json
#[derive(Debug)]
pub enum RepositoryError {
    /// 404 Not Found - Resource does not exist
    NotFound { resource: String, id: String },

    /// 400 Bad Request - Invalid input or validation error
    InvalidInput(String),

    /// 500 Internal Server Error - Database error
    Database(sqlx::Error),

    /// 500 Internal Server Error - JSON serialization/deserialization error
    Serialization(serde_json::Error),
}

impl From<sqlx::Error> for RepositoryError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => RepositoryError::NotFound {
                resource: "Resource".to_string(),
                id: "unknown".to_string(),
            },
            e => RepositoryError::Database(e),
        }
    }
}

impl From<serde_json::Error> for RepositoryError {
    fn from(e: serde_json::Error) -> Self {
        RepositoryError::Serialization(e)
    }
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound { resource, id } => {
                write!(f, "{} not found: {}", resource, id)
            }
            RepositoryError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            RepositoryError::Database(e) => write!(f, "Database error: {}", e),
            RepositoryError::Serialization(e) => write!(f, "Serialization error: {}", e),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Helper to create a NotFound error with resource type and ID
pub fn not_found(resource: &str, id: &str) -> RepositoryError {
    RepositoryError::NotFound {
        resource: resource.to_string(),
        id: id.to_string(),
    }
}
