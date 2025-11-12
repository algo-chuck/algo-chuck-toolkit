use serde::Serialize;

use super::OrderRequest;

/// Parameters for replacing an order.
#[derive(Debug, Clone, Serialize)]
pub struct ReplaceOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order ID to replace
    pub order_id: i64,
    /// The new order details
    #[serde(skip)]
    pub order: &'a OrderRequest,
}

impl<'a> ReplaceOrderParams<'a> {
    /// Create new parameters for replacing an order
    pub fn new(account_hash: &'a str, order_id: i64, order: &'a OrderRequest) -> Self {
        Self {
            account_hash,
            order_id,
            order,
        }
    }
}
