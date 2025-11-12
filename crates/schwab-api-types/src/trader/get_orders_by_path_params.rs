use serde::Serialize;

/// Parameters for fetching orders by account (path parameter variant).
#[derive(Debug, Clone, Serialize)]
pub struct GetOrdersByPathParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
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

impl<'a> GetOrdersByPathParams<'a> {
    /// Create new parameters for fetching orders by account
    pub fn new(
        account_hash: &'a str,
        from_entered_time: &'a str,
        to_entered_time: &'a str,
    ) -> Self {
        Self {
            account_hash,
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
