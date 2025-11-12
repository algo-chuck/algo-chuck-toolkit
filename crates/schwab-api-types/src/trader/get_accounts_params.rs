use serde::Serialize;

/// Parameters for fetching multiple accounts.
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountsParams<'a> {
    /// Fields to include in the response (e.g., "positions")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
}

impl<'a> GetAccountsParams<'a> {
    /// Create new parameters for fetching accounts
    pub fn new() -> Self {
        Self { fields: None }
    }

    /// Set the fields to include in the response
    pub fn with_fields(mut self, fields: &'a str) -> Self {
        self.fields = Some(fields);
        self
    }
}
