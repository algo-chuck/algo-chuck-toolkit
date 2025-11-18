// db/repositories/accounts.rs
// Implements operations from OpenAPI tag: "Accounts"

use schwab_api::types::trader::{GetAccountParams, GetAccountsParams, SecuritiesAccount};
use sqlx::SqlitePool;

use crate::db::{RepositoryError, not_found};

pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    // operationId: getAccountNumbers
    pub async fn get_account_numbers(&self) -> Result<Vec<(String, String)>, RepositoryError> {
        let rows = sqlx::query_as::<_, (String, String)>(
            "SELECT account_number, hash_value FROM accounts ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows)
    }

    // operationId: getAccounts (list all accounts)
    pub async fn get_accounts(
        &self,
        _params: &GetAccountsParams<'_>,
    ) -> Result<Vec<SecuritiesAccount>, RepositoryError> {
        // TODO: Implement field filtering based on params.fields
        // For now, return all fields regardless of params.fields value
        let rows = sqlx::query_scalar::<_, String>(
            "SELECT account_data FROM accounts ORDER BY created_at DESC",
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| serde_json::from_str(&r).map_err(RepositoryError::from))
            .collect()
    }

    // operationId: getAccount (by hash)
    pub async fn get_account(
        &self,
        params: &GetAccountParams<'_>,
    ) -> Result<SecuritiesAccount, RepositoryError> {
        // TODO: Implement field filtering based on params.fields
        // For now, return all fields regardless of params.fields value
        let account_data = sqlx::query_scalar::<_, String>(
            "SELECT account_data FROM accounts WHERE hash_value = ?",
        )
        .bind(params.account_hash)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| not_found("Account", params.account_hash))?;

        serde_json::from_str(&account_data).map_err(RepositoryError::from)
    }

    // Additional helper methods (not directly from operationIds)

    pub async fn create(
        &self,
        account_number: &str,
        hash_value: &str,
        account_type: &str,
        account_data: &SecuritiesAccount,
    ) -> Result<i64, RepositoryError> {
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
    ) -> Result<SecuritiesAccount, RepositoryError> {
        let account_data = sqlx::query_scalar::<_, String>(
            "SELECT account_data FROM accounts WHERE account_number = ?",
        )
        .bind(account_number)
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| not_found("Account", account_number))?;

        serde_json::from_str(&account_data).map_err(RepositoryError::from)
    }

    pub async fn update(
        &self,
        account_number: &str,
        account_data: &SecuritiesAccount,
    ) -> Result<(), RepositoryError> {
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

    /// Delete an account by hash value
    pub async fn delete(&self, hash_value: &str) -> Result<u64, RepositoryError> {
        let result = sqlx::query("DELETE FROM accounts WHERE hash_value = ?")
            .bind(hash_value)
            .execute(&self.pool)
            .await?;

        Ok(result.rows_affected())
    }

    /// Reset an account to initial state by hash value
    ///
    /// This resets the account_data back to initial $200,000 CASH account.
    /// Orders and transactions are automatically deleted via CASCADE DELETE.
    pub async fn reset(
        &self,
        hash_value: &str,
        initial_account_data: &SecuritiesAccount,
    ) -> Result<(), RepositoryError> {
        let account_data_json = serde_json::to_string(initial_account_data)?;

        let result = sqlx::query(
            "UPDATE accounts
             SET account_data = ?, updated_at = CURRENT_TIMESTAMP
             WHERE hash_value = ?",
        )
        .bind(account_data_json)
        .bind(hash_value)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(not_found("Account", hash_value));
        }

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
