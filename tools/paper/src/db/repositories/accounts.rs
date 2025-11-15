// db/repositories/accounts.rs
// Implements operations from OpenAPI tag: "Accounts"

use schwab_api::types::trader::SecuritiesAccount;
use serde_json;
use sqlx::SqlitePool;

#[derive(Debug)]
pub enum AccountError {
    Database(sqlx::Error),
    Serialization(serde_json::Error),
    NotFound(String),
}

impl From<sqlx::Error> for AccountError {
    fn from(e: sqlx::Error) -> Self {
        match e {
            sqlx::Error::RowNotFound => AccountError::NotFound("Account".to_string()),
            e => AccountError::Database(e),
        }
    }
}

impl From<serde_json::Error> for AccountError {
    fn from(e: serde_json::Error) -> Self {
        AccountError::Serialization(e)
    }
}

impl std::fmt::Display for AccountError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountError::Database(e) => write!(f, "Database error: {}", e),
            AccountError::Serialization(e) => write!(f, "Serialization error: {}", e),
            AccountError::NotFound(id) => write!(f, "Account not found: {}", id),
        }
    }
}

impl std::error::Error for AccountError {}

pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getAccountNumbers
    pub async fn get_account_numbers(&self) -> Result<Vec<(String, String)>, AccountError> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT account_number, hash_value FROM accounts ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    // operationId: getAccounts (list all accounts)
    pub async fn get_accounts(&self) -> Result<Vec<SecuritiesAccount>, AccountError> {
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT account_data FROM accounts ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r).map_err(AccountError::from))
            .collect()
    }

    // operationId: getAccount (by hash)
    pub async fn get_account(&self, hash: &str) -> Result<SecuritiesAccount, AccountError> {
        let account_data = sqlx::query_scalar::<_, String>(
            "SELECT account_data FROM accounts WHERE hash_value = ?",
        )
        .bind(hash)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AccountError::NotFound(hash.to_string()))?;

        serde_json::from_str(&account_data).map_err(AccountError::from)
    }

    // Additional helper methods (not directly from operationIds)

    pub async fn create(
        &self,
        account_number: &str,
        hash_value: &str,
        account_type: &str,
        account_data: &SecuritiesAccount,
    ) -> Result<i64, AccountError> {
        let account_data_json = serde_json::to_string(account_data)?;

        let result = sqlx::query(
            "INSERT INTO accounts (account_number, hash_value, account_type, account_data)
             VALUES (?, ?, ?, ?)",
        )
        .bind(account_number)
        .bind(hash_value)
        .bind(account_type)
        .bind(account_data_json)
        .execute(&self.pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    pub async fn find_by_account_number(
        &self,
        account_number: &str,
    ) -> Result<SecuritiesAccount, AccountError> {
        let account_data = sqlx::query_scalar::<_, String>(
            "SELECT account_data FROM accounts WHERE account_number = ?",
        )
        .bind(account_number)
        .fetch_optional(&self.pool)
        .await?
        .ok_or(AccountError::NotFound(account_number.to_string()))?;

        serde_json::from_str(&account_data).map_err(AccountError::from)
    }

    pub async fn update(
        &self,
        account_number: &str,
        account_data: &SecuritiesAccount,
    ) -> Result<(), AccountError> {
        let account_data_json = serde_json::to_string(account_data)?;

        sqlx::query(
            "UPDATE accounts
             SET account_data = ?, updated_at = CURRENT_TIMESTAMP
             WHERE account_number = ?",
        )
        .bind(account_data_json)
        .bind(account_number)
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
    async fn test_get_account_numbers_empty() {
        let pool = setup_test_db().await;
        let repo = AccountRepository::new(pool);
        let result = repo.get_account_numbers().await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
