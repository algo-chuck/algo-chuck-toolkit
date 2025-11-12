use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
///
/// **API Operations (Response):**
/// - `POST /accounts/{accountNumber}/previewOrder` - Preview order for a specific account.
/// **API Operations (Request):**
/// - `POST /accounts/{accountNumber}/previewOrder` - Preview order for a specific account.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PreviewOrder {
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(rename = "orderStrategy", skip_serializing_if = "Option::is_none")]
    pub order_strategy: Option<Box<trader::OrderStrategy>>,
    #[serde(
        rename = "orderValidationResult",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_validation_result: Option<Box<trader::OrderValidationResult>>,
    #[serde(rename = "commissionAndFee", skip_serializing_if = "Option::is_none")]
    pub commission_and_fee: Option<Box<trader::CommissionAndFee>>,
}

impl PreviewOrder {
    pub fn new() -> PreviewOrder {
        PreviewOrder {
            order_id: None,
            order_strategy: None,
            order_validation_result: None,
            commission_and_fee: None,
        }
    }
}
