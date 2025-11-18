// db/repositories/user_preference.rs
// Implements operations from OpenAPI tag: "User Preference"

use schwab_api::types::trader::UserPreference;
use sqlx::SqlitePool;

use crate::db::{RepositoryError, not_found};

pub struct UserPreferenceRepository {
    pool: SqlitePool,
}

impl UserPreferenceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getUserPreference
    pub async fn get_user_preference(&self) -> Result<UserPreference, RepositoryError> {
        let preference_data = sqlx::query_scalar::<_, String>(
            "SELECT preference_data FROM user_preferences WHERE id = 1",
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| not_found("UserPreference", "1"))?;

        serde_json::from_str(&preference_data).map_err(RepositoryError::from)
    }

    // Additional helper method

    pub async fn upsert(&self, preference_data: &UserPreference) -> Result<(), RepositoryError> {
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
