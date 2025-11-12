use serde::Serialize;

/// Parameters for fetching option expiration chain.
#[derive(Debug, Clone, Serialize)]
pub struct GetExpirationChainParams<'a> {
    /// The underlying symbol
    pub symbol: &'a str,
}

impl<'a> GetExpirationChainParams<'a> {
    /// Create new GetExpirationChainParams with the required symbol
    pub fn new(symbol: &'a str) -> Self {
        Self { symbol }
    }
}
