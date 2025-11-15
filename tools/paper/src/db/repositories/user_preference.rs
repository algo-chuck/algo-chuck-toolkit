// db/repositories/user_preference.rs
// Implements operations from OpenAPI tag: "User Preference"

use schwab_api::types::trader::UserPreference;
use serde_json;
use sqlx::SqlitePool;

#[derive(Debug)]
pub enum UserPreferenceError {
    Database(sqlx::Error),
    Serialization(serde_json::Error),
    NotFound,
}

impl From<sqlx::Error> for UserPreferenceError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => UserPreferenceError::NotFound,
            e => UserPreferenceError::Database(e),
        }
    }
}

impl From<serde_json::Error> for UserPreferenceError {
    fn from(e: serde_json::Error) -> Self {
        UserPreferenceError::Serialization(e)
    }
}

impl std::fmt::Display for UserPreferenceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserPreferenceError::Database(e) => write!(f, "Database error: {}", e),
            UserPreferenceError::Serialization(e) => write!(f, "Serialization error: {}", e),
            UserPreferenceError::NotFound => write!(f, "User preference not found"),
        }
    }
}

impl std::error::Error for UserPreferenceError {}

pub struct UserPreferenceRepository {
    pool: SqlitePool,
}

impl UserPreferenceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getUserPreference
    pub async fn get_user_preference(&self) -> Result<UserPreference, UserPreferenceError> {
        let preference_data = sqlx::query_scalar::<_, String>(
            "SELECT preference_data FROM user_preferences WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(UserPreferenceError::NotFound)?;

        serde_json::from_str(&preference_data).map_err(UserPreferenceError::from)
    }

    // Additional helper method

    pub async fn upsert(
        &self,
        preference_data: &UserPreference,
    ) -> Result<(), UserPreferenceError> {
        let preference_data_json = serde_json::to_string(preference_data)?;

        sqlx::query(
            "INSERT INTO user_preferences (id, preference_data)
             VALUES (1, ?)
             ON CONFLICT(id) DO UPDATE SET 
                 preference_data = excluded.preference_data,
                 updated_at = CURRENT_TIMESTAMP",
        )
        .bind(preference_data_json)
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn setup_test_db() -> SqlitePool {
        let pool = SqlitePool::connect("sqlite::memory:").await.unwrap();
        sqlx::migrate!("./src/db/migrations")
            .run(&pool)
            .await
            .unwrap();
        pool
    }

    #[tokio::test]
    async fn test_get_user_preference_not_found() {
        let pool = setup_test_db().await;
        let repo = UserPreferenceRepository::new(pool);

        let result = repo.get_user_preference().await;
        assert!(result.is_err());
    }
}
