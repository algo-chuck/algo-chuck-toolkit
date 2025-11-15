// Database module for paper trader application
// Manages SQLite connection pool and migrations

use sqlx::Error as SqlxError;
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

pub mod repositories;

/// Initialize the database pool and run migrations
pub async fn init_db(database_url: &str) -> Result<SqlitePool, SqlxError> {
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // Run migrations
    sqlx::migrate!("./src/db/migrations").run(&pool).await?;

    Ok(pool)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_init_db() {
        let pool = init_db("sqlite::memory:").await;
        assert!(pool.is_ok());
    }
}
