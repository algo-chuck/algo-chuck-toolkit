use serde::Serialize;

/// Parameters for fetching orders across all accounts (query parameter variant).
#[derive(Debug, Clone, Serialize)]
pub struct GetOrdersByQueryParams<'a> {
    /// Start date for order search (ISO-8601 format)
    #[serde(rename = "fromEnteredTime")]
    pub from_entered_time: &'a str,
    /// End date for order search (ISO-8601 format)
    #[serde(rename = "toEnteredTime")]
    pub to_entered_time: &'a str,
    /// Maximum number of orders to return
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// Filter by order status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

impl<'a> GetOrdersByQueryParams<'a> {
    /// Create new parameters for fetching orders across all accounts
    pub fn new(from_entered_time: &'a str, to_entered_time: &'a str) -> Self {
        Self {
            from_entered_time,
            to_entered_time,
            max_results: None,
            status: None,
        }
    }

    /// Set the maximum number of results to return
    pub fn with_max_results(mut self, max_results: i32) -> Self {
        self.max_results = Some(max_results);
        self
    }

    /// Set the order status filter
    pub fn with_status(mut self, status: &'a str) -> Self {
        self.status = Some(status);
        self
    }
}
