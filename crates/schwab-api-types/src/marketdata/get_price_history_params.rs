use serde::Serialize;

/// Parameters for fetching price history.
#[derive(Debug, Clone, Serialize)]
pub struct GetPriceHistoryParams<'a> {
    /// The symbol
    pub symbol: &'a str,
    /// Period type (day, month, year, ytd)
    #[serde(rename = "periodType", skip_serializing_if = "Option::is_none")]
    pub period_type: Option<&'a str>,
    /// Number of periods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// Frequency type (minute, daily, weekly, monthly)
    #[serde(rename = "frequencyType", skip_serializing_if = "Option::is_none")]
    pub frequency_type: Option<&'a str>,
    /// Frequency (1, 5, 10, 15, 30 for minute, 1 for daily/weekly/monthly)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<i32>,
    /// Start date as epoch milliseconds
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<i64>,
    /// End date as epoch milliseconds
    #[serde(rename = "endDate", skip_serializing_if = "Option::is_none")]
    pub end_date: Option<i64>,
    /// Include extended hours data
    #[serde(
        rename = "needExtendedHoursData",
        skip_serializing_if = "Option::is_none"
    )]
    pub need_extended_hours_data: Option<bool>,
    /// Include previous close
    #[serde(rename = "needPreviousClose", skip_serializing_if = "Option::is_none")]
    pub need_previous_close: Option<bool>,
}

impl<'a> GetPriceHistoryParams<'a> {
    /// Create new GetPriceHistoryParams with the required symbol
    pub fn new(symbol: &'a str) -> Self {
        Self {
            symbol,
            period_type: None,
            period: None,
            frequency_type: None,
            frequency: None,
            start_date: None,
            end_date: None,
            need_extended_hours_data: None,
            need_previous_close: None,
        }
    }

    /// Set the period type
    pub fn with_period_type(mut self, period_type: &'a str) -> Self {
        self.period_type = Some(period_type);
        self
    }

    /// Set the period
    pub fn with_period(mut self, period: i32) -> Self {
        self.period = Some(period);
        self
    }

    /// Set the frequency type
    pub fn with_frequency_type(mut self, frequency_type: &'a str) -> Self {
        self.frequency_type = Some(frequency_type);
        self
    }

    /// Set the frequency
    pub fn with_frequency(mut self, frequency: i32) -> Self {
        self.frequency = Some(frequency);
        self
    }

    /// Set the start date
    pub fn with_start_date(mut self, start_date: i64) -> Self {
        self.start_date = Some(start_date);
        self
    }

    /// Set the end date
    pub fn with_end_date(mut self, end_date: i64) -> Self {
        self.end_date = Some(end_date);
        self
    }

    /// Set whether to include extended hours data
    pub fn with_need_extended_hours_data(mut self, need_extended_hours_data: bool) -> Self {
        self.need_extended_hours_data = Some(need_extended_hours_data);
        self
    }

    /// Set whether to include previous close
    pub fn with_need_previous_close(mut self, need_previous_close: bool) -> Self {
        self.need_previous_close = Some(need_previous_close);
        self
    }
}
