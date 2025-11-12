use serde::Serialize;

/// Parameters for canceling an order.
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order ID to cancel
    pub order_id: i64,
}

impl<'a> CancelOrderParams<'a> {
    /// Create new parameters for canceling an order
    pub fn new(account_hash: &'a str, order_id: i64) -> Self {
        Self {
            account_hash,
            order_id,
        }
    }
}
