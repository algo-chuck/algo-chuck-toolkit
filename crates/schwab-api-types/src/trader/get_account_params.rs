use serde::Serialize;

/// Parameters for fetching a single account.
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountParams<'a> {
    /// The encrypted account ID
    #[serde(skip)] // skip path parameter from inclusion in query parameter
    pub account_hash: &'a str,
    /// Fields to include in the response (e.g., "positions")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
}

impl<'a> GetAccountParams<'a> {
    /// Create new parameters for fetching an account
    pub fn new(account_hash: &'a str) -> Self {
        Self {
            account_hash,
            fields: None,
        }
    }

    /// Set the fields to include in the response
    pub fn with_fields(mut self, fields: &'a str) -> Self {
        self.fields = Some(fields);
        self
    }
}
