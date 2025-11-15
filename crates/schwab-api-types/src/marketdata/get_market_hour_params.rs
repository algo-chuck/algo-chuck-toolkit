use serde::Serialize;

/// Parameters for fetching market hours for a single market.
#[derive(Debug, Clone, Serialize)]
pub struct GetMarketHourParams<'a> {
    /// The market (equity, option, bond, future, forex)
    #[serde(skip)] // skip path parameter from inclusion in query parameter
    pub market: &'a str,
    /// Date in yyyy-MM-dd format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<&'a str>,
}

impl<'a> GetMarketHourParams<'a> {
    /// Create new GetMarketHourParams with the required market
    pub fn new(market: &'a str) -> Self {
        Self { market, date: None }
    }

    /// Set the date for which to fetch market hours
    pub fn with_date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }
}
