use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderBalance {
    #[serde(rename = "orderValue", skip_serializing_if = "Option::is_none")]
    pub order_value: Option<f64>,
    #[serde(
        rename = "projectedAvailableFund",
        skip_serializing_if = "Option::is_none"
    )]
    pub projected_available_fund: Option<f64>,
    #[serde(
        rename = "projectedBuyingPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub projected_buying_power: Option<f64>,
    #[serde(
        rename = "projectedCommission",
        skip_serializing_if = "Option::is_none"
    )]
    pub projected_commission: Option<f64>,
}

