// Database module for paper trader application
// Manages SQLite connection pool and migrations

use sqlx::Error as SqlxError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub mod repositories;

/// Initialize the database pool and run migrations
pub async fn init_db() -> Result<SqlitePool, SqlxError> {
    let database_url = sql_db_url();
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await?;

    // Run migrations
    sqlx::migrate!("./src/db/migrations").run(&pool).await?;

    Ok(pool)
}

/// Get the database URL from environment or use default in-memory SQLite
fn sql_db_url() -> String {
    std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db() {
        let pool = init_db().await;
        assert!(pool.is_ok());
    }
}
