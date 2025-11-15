use serde::Serialize;

/// Parameters for fetching a single order.
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderParams<'a> {
    /// The encrypted account ID
    #[serde(skip)] // skip path parameter from inclusion in query parameter
    pub account_hash: &'a str,
    /// The order ID
    #[serde(skip)] // skip path parameter from inclusion in query parameter
    pub order_id: i64,
}

impl<'a> GetOrderParams<'a> {
    /// Create new parameters for fetching an order
    pub fn new(account_hash: &'a str, order_id: i64) -> Self {
        Self {
            account_hash,
            order_id,
        }
    }
}
