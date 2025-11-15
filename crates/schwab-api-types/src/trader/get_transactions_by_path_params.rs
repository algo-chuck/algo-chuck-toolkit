use serde::Serialize;

/// Parameters for fetching transactions by account (path parameter variant).
#[derive(Debug, Clone, Serialize)]
pub struct GetTransactionsByPathParams<'a> {
    /// The encrypted account ID
    #[serde(skip)] // skip path parameter from inclusion in query parameter
    pub account_hash: &'a str,
    /// Start date for transaction search (ISO-8601 format)
    #[serde(rename = "startDate")]
    pub start_date: &'a str,
    /// End date for transaction search (ISO-8601 format)
    #[serde(rename = "endDate")]
    pub end_date: &'a str,
    /// Transaction types to include (comma-separated)
    pub types: &'a str,
    /// Filter by symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,
}

impl<'a> GetTransactionsByPathParams<'a> {
    /// Create new parameters for fetching transactions by account
    pub fn new(
        account_hash: &'a str,
        start_date: &'a str,
        end_date: &'a str,
        types: &'a str,
    ) -> Self {
        Self {
            account_hash,
            start_date,
            end_date,
            types,
            symbol: None,
        }
    }

    /// Set the symbol filter
    pub fn with_symbol(mut self, symbol: &'a str) -> Self {
        self.symbol = Some(symbol);
        self
    }
}
