use serde::Serialize;

use super::PreviewOrder;

/// Parameters for previewing an order.
#[derive(Debug, Clone, Serialize)]
pub struct PreviewOrderParams<'a> {
    /// The encrypted account ID
    #[serde(skip)] // skip path parameter from inclusion in query parameter
    pub account_hash: &'a str,
    /// The order to preview
    #[serde(skip)]
    pub order: &'a PreviewOrder,
}

impl<'a> PreviewOrderParams<'a> {
    /// Create new parameters for previewing an order
    pub fn new(account_hash: &'a str, order: &'a PreviewOrder) -> Self {
        Self {
            account_hash,
            order,
        }
    }
}
