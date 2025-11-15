use serde::Serialize;

/// Parameters for getting an instrument by CUSIP.
#[derive(Debug, Clone, Serialize)]
pub struct GetInstrumentByCusipParams<'a> {
    /// The CUSIP identifier
    #[serde(skip)] // (skip path parameter from inclusion in query parameter)
    pub cusip: &'a str,
}

impl<'a> GetInstrumentByCusipParams<'a> {
    /// Create new GetInstrumentByCusipParams with the required CUSIP
    pub fn new(cusip: &'a str) -> Self {
        Self { cusip }
    }
}
