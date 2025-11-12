use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLeg {
    #[serde(rename = "askPrice", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(rename = "bidPrice", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(rename = "lastPrice", skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<f64>,
    #[serde(
        rename = "projectedCommission",
        skip_serializing_if = "Option::is_none"
    )]
    pub projected_commission: Option<f64>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(rename = "finalSymbol", skip_serializing_if = "Option::is_none")]
    pub final_symbol: Option<String>,
    #[serde(rename = "legId", skip_serializing_if = "Option::is_none")]
    pub leg_id: Option<f64>,
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<trader::AssetType>,
    #[serde(rename = "instruction", skip_serializing_if = "Option::is_none")]
    pub instruction: Option<trader::Instruction>,
}

