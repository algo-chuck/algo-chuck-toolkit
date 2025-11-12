use serde::{Deserialize, Serialize};

/// Market data information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionDeliverables {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<String>,
    #[serde(rename = "deliverableUnits", skip_serializing_if = "Option::is_none")]
    pub deliverable_units: Option<String>,
    #[serde(rename = "currencyType", skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
}

