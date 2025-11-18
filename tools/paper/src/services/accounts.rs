//! Account service - business logic for account operations
//!
//! Thin CRUD wrapper around AccountRepository with input validation.

use crate::db::repositories::{AccountRepository, RepositoryError};
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
    Repository(#[from] RepositoryError),
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
        params: GetAccountsParams<'_>,
    ) -> Result<Vec<SecuritiesAccount>, AccountServiceError> {
        self.repository
            .get_accounts(&params)
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
            .get_account(&params)
            .await
            .map_err(AccountServiceError::from)
    }

    /// Create a new account (admin operation)
    ///
    /// Maps to: POST /admin/v1/accounts
    pub async fn create_account(
        &self,
        account_number: &str,
        hash_value: &str,
        account_data: &SecuritiesAccount,
    ) -> Result<(), AccountServiceError> {
        // Validate inputs
        if account_number.trim().is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "account_number cannot be empty".to_string(),
            ));
        }

        if hash_value.trim().is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "hash_value cannot be empty".to_string(),
            ));
        }

        // Determine account type from SecuritiesAccount enum
        let account_type = match account_data {
            SecuritiesAccount::Cash(_) => "CASH",
            SecuritiesAccount::Margin(_) => "MARGIN",
        };

        // Call repository to create the account
        self.repository
            .create(account_number, hash_value, account_type, account_data)
            .await?;

        Ok(())
    }

    /// Delete an account (admin operation)
    ///
    /// Maps to: DELETE /admin/v1/accounts/{accountNumber}
    ///
    /// Cascade deletion is handled by database foreign key constraints.
    pub async fn delete_account(&self, account_number: &str) -> Result<(), AccountServiceError> {
        // Validate input
        if account_number.trim().is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "account_number cannot be empty".to_string(),
            ));
        }

        // Delete the account - related records cascade automatically
        let rows_affected = self.repository.delete(account_number).await?;

        if rows_affected == 0 {
            return Err(AccountServiceError::NotFound(account_number.to_string()));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // Tests will be added with in-memory database setup
    #[tokio::test]
    async fn test_placeholder() {
        // TODO: Implement tests with :memory: database
    }
}
