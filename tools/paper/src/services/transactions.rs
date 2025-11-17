//! Transaction service - business logic for transaction operations
//!
//! Thin CRUD wrapper around TransactionRepository with input validation.

use crate::db::repositories::{TransactionError, TransactionRepository};
use schwab_api::types::trader::{
    GetTransactionByIdParams, GetTransactionsByPathParams, Transaction,
};

/// Errors that can occur in transaction service operations
#[derive(Debug, thiserror::Error)]
pub enum TransactionServiceError {
    #[error("Transaction not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Repository error: {0}")]
    Repository(#[from] TransactionError),
}

/// Service for transaction operations
pub struct TransactionService {
    repository: TransactionRepository,
}

impl TransactionService {
    /// Create a new transaction service
    pub fn new(repository: TransactionRepository) -> Self {
        Self { repository }
    }

    /// Get transactions for an account
    ///
    /// Maps to: GET /trader/v1/accounts/{accountNumber}/transactions
    pub async fn get_transactions(
        &self,
        params: GetTransactionsByPathParams<'_>,
    ) -> Result<Vec<Transaction>, TransactionServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(TransactionServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        self.repository
            .get_transactions_by_path_param(&params)
            .await
            .map_err(TransactionServiceError::from)
    }

    /// Get a specific transaction by ID
    ///
    /// Maps to: GET /trader/v1/accounts/{accountNumber}/transactions/{transactionId}
    pub async fn get_transaction(
        &self,
        params: GetTransactionByIdParams<'_>,
    ) -> Result<Transaction, TransactionServiceError> {
        // Validate account hash is not empty
        if params.account_hash.trim().is_empty() {
            return Err(TransactionServiceError::InvalidInput(
                "account_hash cannot be empty".to_string(),
            ));
        }

        self.repository
            .get_transactions_by_id(params.transaction_id)
            .await
            .map_err(TransactionServiceError::from)
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
