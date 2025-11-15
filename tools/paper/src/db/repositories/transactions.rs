// db/repositories/transactions.rs
// Implements operations from OpenAPI tag: "Transactions"

use schwab_api::types::trader::Transaction;
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
        account_number: &str,
        start_date: &str,
        end_date: &str,
        transaction_type: Option<&str>,
    ) -> Result<Vec<Transaction>, TransactionError> {
        // TODO: Implement date and type filtering
        let _ = (start_date, end_date, transaction_type);

        let rows = sqlx::query_scalar::<_, String>(
            "SELECT transaction_data FROM transactions 
             WHERE account_number = ?
             ORDER BY time DESC",
        )
        .bind(account_number)
        .fetch_all(&self.pool)
        .await?;

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
