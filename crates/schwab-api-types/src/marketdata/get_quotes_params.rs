use serde::Serialize;

/// Parameters for fetching quotes for multiple symbols.
#[derive(Debug, Clone, Serialize)]
pub struct GetQuotesParams<'a> {
    /// Comma-separated list of symbols
    pub symbols: &'a str,
    /// Fields to include in the response
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
    /// Include indicative symbol quotes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indicative: Option<bool>,
}

impl<'a> GetQuotesParams<'a> {
    /// Create new GetQuotesParams with the required symbols
    pub fn new(symbols: &'a str) -> Self {
        Self {
            symbols,
            fields: None,
            indicative: None,
        }
    }

    /// Set the fields to include in the response
    pub fn with_fields(mut self, fields: &'a str) -> Self {
        self.fields = Some(fields);
        self
    }

    /// Set whether to include indicative symbol quotes
    pub fn with_indicative(mut self, indicative: bool) -> Self {
        self.indicative = Some(indicative);
        self
    }
}
