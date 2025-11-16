//! Account service - business logic for account operations
//!
//! Thin CRUD wrapper around AccountRepository with input validation.

use crate::db::repositories::{AccountError, AccountRepository};
use schwab_api::types::trader::{
    AccountNumberHash, GetAccountParams, GetAccountsParams, SecuritiesAccount,
};

/// Errors that can occur in account service operations
#[derive(Debug, thiserror::Error)]
pub enum AccountServiceError {
    #[error("Account not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Repository error: {0}")]
    Repository(#[from] AccountError),
}

/// Service for account operations
pub struct AccountService {
    repository: AccountRepository,
}

impl AccountService {
    /// Create a new account service
    pub fn new(repository: AccountRepository) -> Self {
        Self { repository }
    }

    /// Get account numbers and hashes for all accounts
    ///
    /// Maps to: GET /trader/v1/accounts/accountNumbers
    pub async fn get_account_numbers(&self) -> Result<Vec<AccountNumberHash>, AccountServiceError> {
        let account_tuples = self.repository.get_account_numbers().await?;

        // Convert Vec<(String, String)> to Vec<AccountNumberHash>
        Ok(account_tuples
            .into_iter()
            .map(|(account_number, hash_value)| AccountNumberHash {
                account_number: Some(account_number),
                hash_value: Some(hash_value),
            })
            .collect())
    }

    /// Get all accounts with optional filtering
    ///
    /// Maps to: GET /trader/v1/accounts
    pub async fn get_accounts(
        &self,
        _params: GetAccountsParams<'_>,
    ) -> Result<Vec<SecuritiesAccount>, AccountServiceError> {
        self.repository
            .get_accounts()
            .await
            .map_err(AccountServiceError::from)
    }

    /// Get a specific account by encrypted ID
    ///
    /// Maps to: GET /trader/v1/accounts/{encryptedAccountId}
    pub async fn get_account(
        &self,
        params: GetAccountParams<'_>,
    ) -> Result<SecuritiesAccount, AccountServiceError> {
        // Validate hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        self.repository
            .get_account(&params.account_hash)
            .await
            .map_err(AccountServiceError::from)
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
