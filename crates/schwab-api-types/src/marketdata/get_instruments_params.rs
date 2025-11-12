use serde::Serialize;

/// Parameters for searching instruments.
#[derive(Debug, Clone, Serialize)]
pub struct GetInstrumentsParams<'a> {
    /// The symbol or partial symbol to search for
    pub symbol: &'a str,
    /// The projection type (symbol-search, symbol-regex, desc-search, desc-regex, search, fundamental)
    pub projection: &'a str,
}

impl<'a> GetInstrumentsParams<'a> {
    /// Create new GetInstrumentsParams with the required symbol and projection
    pub fn new(symbol: &'a str, projection: &'a str) -> Self {
        Self { symbol, projection }
    }
}
