use serde::Serialize;

/// Parameters for fetching a quote for a single symbol.
#[derive(Debug, Clone, Serialize)]
pub struct GetQuoteParams<'a> {
    /// The symbol to get a quote for
    pub symbol: &'a str,
    /// Fields to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
}

impl<'a> GetQuoteParams<'a> {
    /// Create new GetQuoteParams with the required symbol
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            fields: None,
        }
    }

    /// Set the fields to include in the response
    pub fn with_fields(mut self, fields: &'a str) -> Self {
        self.fields = Some(fields);
        self
    }
}
