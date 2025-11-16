// db/repositories/transactions.rs
// Implements operations from OpenAPI tag: "Transactions"

use schwab_api::types::trader::{GetTransactionsByPathParams, Transaction};
use serde_json;
use sqlx::SqlitePool;

#[derive(Debug)]
pub enum TransactionError {
    Database(sqlx::Error),
    Serialization(serde_json::Error),
    NotFound(i64),
}

impl From<sqlx::Error> for TransactionError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => TransactionError::NotFound(0),
            e => TransactionError::Database(e),
        }
    }
}

impl From<serde_json::Error> for TransactionError {
    fn from(e: serde_json::Error) -> Self {
        TransactionError::Serialization(e)
    }
}

impl std::fmt::Display for TransactionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransactionError::Database(e) => write!(f, "Database error: {}", e),
            TransactionError::Serialization(e) => write!(f, "Serialization error: {}", e),
            TransactionError::NotFound(id) => write!(f, "Transaction not found: {}", id),
        }
    }
}

impl std::error::Error for TransactionError {}

pub struct TransactionRepository {
    pool: SqlitePool,
}

impl TransactionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getTransactionsByPathParam
    pub async fn get_transactions_by_path_param(
        &self,
        params: &GetTransactionsByPathParams<'_>,
    ) -> Result<Vec<Transaction>, TransactionError> {
        // Build dynamic query based on provided parameters
        let mut query =
            String::from("SELECT transaction_data FROM transactions WHERE account_number = ?");
        let mut bind_values: Vec<String> = vec![params.account_hash.to_string()];

        // Add date range filtering
        query.push_str(" AND time >= ?");
        bind_values.push(params.start_date.to_string());

        query.push_str(" AND time <= ?");
        bind_values.push(params.end_date.to_string());

        // Add type filtering (comma-separated list)
        // Split comma-separated types and build IN clause
        let type_list: Vec<&str> = params.types.split(',').map(|s| s.trim()).collect();
        if !type_list.is_empty() {
            let placeholders = type_list.iter().map(|_| "?").collect::<Vec<_>>().join(",");
            query.push_str(&format!(" AND type IN ({})", placeholders));
            for t in type_list {
                bind_values.push(t.to_string());
            }
        }

        // Add symbol filtering if provided
        // Note: This requires extracting symbol from JSON, which is complex
        // For now, we'll skip symbol filtering and add TODO
        // TODO: Implement symbol filtering (requires JSON extraction or indexed column)
        if params.symbol.is_some() {
            // Placeholder - symbol filtering not yet implemented
        }

        query.push_str(" ORDER BY time DESC");

        // Execute query with dynamic bindings
        let mut sqlx_query = sqlx::query_scalar::<_, String>(&query);
        for value in bind_values {
            sqlx_query = sqlx_query.bind(value);
        }

        let rows = sqlx_query.fetch_all(&self.pool).await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r).map_err(TransactionError::from))
            .collect()
    }

    // operationId: getTransactionsById
    pub async fn get_transactions_by_id(
        &self,
        activity_id: i64,
    ) -> Result<Transaction, TransactionError> {
        let transaction_data = sqlx::query_scalar::<_, String>(
            "SELECT transaction_data FROM transactions WHERE activity_id = ?",
        )
        .bind(activity_id)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(TransactionError::NotFound(activity_id))?;

        serde_json::from_str(&transaction_data).map_err(TransactionError::from)
    }

    // Additional helper method

    pub async fn create(
        &self,
        account_number: &str,
        transaction_type: &str,
        transaction_data: &Transaction,
    ) -> Result<i64, TransactionError> {
        let transaction_data_json = serde_json::to_string(transaction_data)?;

        // Get next activity_id (starting from 1001)
        let activity_id: i64 =
            sqlx::query_scalar("SELECT COALESCE(MAX(activity_id), 1000) + 1 FROM transactions")
                .fetch_one(&self.pool)
                .await?;

        sqlx::query(
            "INSERT INTO transactions (activity_id, account_number, type, transaction_data, time)
             VALUES (?, ?, ?, ?, CURRENT_TIMESTAMP)",
        )
        .bind(activity_id)
        .bind(account_number)
        .bind(transaction_type)
        .bind(transaction_data_json)
        .execute(&self.pool)
        .await?;

        Ok(activity_id)
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
    async fn test_activity_id_starts_at_1001() {
        let pool = setup_test_db().await;
        let repo = TransactionRepository::new(pool);

        // First transaction should have ID 1001
        let txn = Transaction::default();
        let activity_id = repo.create("12345", "TRADE", &txn).await.unwrap();
        assert_eq!(activity_id, 1001);
    }
}
