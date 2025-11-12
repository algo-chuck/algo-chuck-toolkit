use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionApiOptionDeliverable {
    #[serde(rename = "rootSymbol", skip_serializing_if = "Option::is_none")]
    pub root_symbol: Option<String>,
    #[serde(rename = "strikePercent", skip_serializing_if = "Option::is_none")]
    pub strike_percent: Option<i64>,
    #[serde(rename = "deliverableNumber", skip_serializing_if = "Option::is_none")]
    pub deliverable_number: Option<i64>,
    #[serde(rename = "deliverableUnits", skip_serializing_if = "Option::is_none")]
    pub deliverable_units: Option<f64>,
    #[serde(rename = "deliverable", skip_serializing_if = "Option::is_none")]
    pub deliverable: Option<Box<trader::TransactionInstrument>>,
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<trader::AssetType>,
}

