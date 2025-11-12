use serde::Serialize;

/// Parameters for fetching a single transaction by ID.
#[derive(Debug, Clone, Serialize)]
pub struct GetTransactionByIdParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The transaction ID
    pub transaction_id: i64,
}

impl<'a> GetTransactionByIdParams<'a> {
    /// Create new parameters for fetching a transaction by ID
    pub fn new(account_hash: &'a str, transaction_id: i64) -> Self {
        Self {
            account_hash,
            transaction_id,
        }
    }
}
