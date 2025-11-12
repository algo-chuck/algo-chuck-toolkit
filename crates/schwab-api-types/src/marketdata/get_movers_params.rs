use serde::Serialize;

/// Parameters for fetching market movers.
#[derive(Debug, Clone, Serialize)]
pub struct GetMoversParams<'a> {
    /// The index symbol ($DJI, $COMPX, $SPX, etc.)
    pub symbol: &'a str,
    /// Sort order (VOLUME, TRADES, PERCENT_CHANGE_UP, PERCENT_CHANGE_DOWN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<&'a str>,
    /// Frequency in minutes (0, 1, 5, 10, 30, 60)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
}

impl<'a> GetMoversParams<'a> {
    /// Create new GetMoversParams with the required symbol
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            sort: None,
            frequency: None,
        }
    }

    /// Set the sort order
    pub fn with_sort(mut self, sort: &'a str) -> Self {
        self.sort = Some(sort);
        self
    }

    /// Set the frequency
    pub fn with_frequency(mut self, frequency: i32) -> Self {
        self.frequency = Some(frequency);
        self
    }
}
