use serde::Serialize;

use super::OrderRequest;

/// Parameters for placing an order.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderParams<'a> {
    /// The encrypted account ID
    #[serde(skip)] // (skip path parameter from inclusion in query parameter)
    pub account_hash: &'a str,
    /// The order details
    #[serde(skip)]
    pub order: &'a OrderRequest,
}

impl<'a> PlaceOrderParams<'a> {
    /// Create new parameters for placing an order
    pub fn new(account_hash: &'a str, order: &'a OrderRequest) -> Self {
        Self {
            account_hash,
            order,
        }
    }
}
