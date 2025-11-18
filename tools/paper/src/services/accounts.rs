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
    /// Maps to: DELETE /admin/v1/accounts/{hashValue}
    ///
    /// Cascade deletion is handled by database foreign key constraints.
    pub async fn delete_account(&self, hash_value: &str) -> Result<(), AccountServiceError> {
        // Validate input
        if hash_value.trim().is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "hash_value cannot be empty".to_string(),
            ));
        }

        // Delete the account - related records cascade automatically
        let rows_affected = self.repository.delete(hash_value).await?;

        if rows_affected == 0 {
            return Err(AccountServiceError::NotFound(hash_value.to_string()));
        }

        Ok(())
    }

    /// Reset an account to initial state (admin operation)
    ///
    /// Maps to: POST /admin/v1/accounts/{hashValue}/reset
    ///
    /// Resets account balances to $200,000 and clears all orders/transactions.
    /// CASCADE DELETE handles related data automatically.
    pub async fn reset_account(&self, hash_value: &str) -> Result<(), AccountServiceError> {
        // Validate input
        if hash_value.trim().is_empty() {
            return Err(AccountServiceError::InvalidInput(
                "hash_value cannot be empty".to_string(),
            ));
        }

        // First get the current account to extract account_number
        let current_account = self
            .repository
            .get_account(&GetAccountParams {
                account_hash: hash_value,
                fields: None,
            })
            .await?;

        let account_number = match &current_account {
            SecuritiesAccount::Cash(cash_account) => {
                cash_account.account_number.as_ref().ok_or_else(|| {
                    AccountServiceError::InvalidInput("Missing account_number".into())
                })?
            }
            SecuritiesAccount::Margin(margin_account) => {
                margin_account.account_number.as_ref().ok_or_else(|| {
                    AccountServiceError::InvalidInput("Missing account_number".into())
                })?
            }
        };

        // Create fresh account data with initial $200,000 balance
        let initial_balance = 200_000.0;
        let fresh_account_data = create_initial_cash_account(account_number, initial_balance);

        // Reset the account data in the repository
        self.repository
            .reset(hash_value, &fresh_account_data)
            .await?;

        Ok(())
    }
}

/// Helper function to create initial CASH account structure
fn create_initial_cash_account(account_number: &str, initial_balance: f64) -> SecuritiesAccount {
    use schwab_api::types::trader::{CashAccount, CashBalance, CashInitialBalance};

    let initial_balances = Box::new(CashInitialBalance {
        cash_available_for_trading: Some(initial_balance),
        cash_balance: Some(initial_balance),
        ..Default::default()
    });

    let current_balances = Box::new(CashBalance {
        cash_available_for_trading: Some(initial_balance),
        total_cash: Some(initial_balance),
        ..Default::default()
    });

    SecuritiesAccount::Cash(Box::new(CashAccount {
        initial_balances: Some(initial_balances),
        current_balances: Some(current_balances.clone()),
        projected_balances: Some(current_balances),
        r#type: None,
        account_number: Some(account_number.to_string()),
        round_trips: Some(0),
        is_day_trader: Some(false),
        is_closing_only_restricted: Some(false),
        pfcb_flag: Some(false),
        positions: None,
    }))
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
