use serde::Serialize;

/// Parameters for fetching market hours for multiple markets.
#[derive(Debug, Clone, Serialize)]
pub struct GetMarketHoursParams<'a> {
    /// Comma-separated list of markets (equity, option, bond, future, forex)
    pub markets: &'a str,
    /// Date in yyyy-MM-dd format
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<&'a str>,
}

impl<'a> GetMarketHoursParams<'a> {
    /// Create new GetMarketHoursParams with the required markets
    pub fn new(markets: &'a str) -> Self {
        Self {
            markets,
            date: None,
        }
    }

    /// Set the date for which to fetch market hours
    pub fn with_date(mut self, date: &'a str) -> Self {
        self.date = Some(date);
        self
    }
}
