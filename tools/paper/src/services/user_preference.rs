//! User preference service - business logic for user preference operations
//!
//! Thin CRUD wrapper around UserPreferenceRepository with input validation.

use crate::db::repositories::{UserPreferenceError, UserPreferenceRepository};
use schwab_api::types::trader::UserPreference;

/// Errors that can occur in user preference service operations
#[derive(Debug, thiserror::Error)]
pub enum UserPreferenceServiceError {
    #[error("User preference not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Repository error: {0}")]
    Repository(#[from] UserPreferenceError),
}

/// Service for user preference operations
pub struct UserPreferenceService {
    repository: UserPreferenceRepository,
}

impl UserPreferenceService {
    /// Create a new user preference service
    pub fn new(repository: UserPreferenceRepository) -> Self {
        Self { repository }
    }

    /// Get user preferences
    ///
    /// Maps to: GET /trader/v1/userPreference
    pub async fn get_user_preference(&self) -> Result<UserPreference, UserPreferenceServiceError> {
        self.repository
            .get_user_preference()
            .await
            .map_err(UserPreferenceServiceError::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests will be added with in-memory database setup
    #[tokio::test]
    async fn test_placeholder() {
        // TODO: Implement tests with :memory: database
    }
}
