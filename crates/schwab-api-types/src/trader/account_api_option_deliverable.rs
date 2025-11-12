use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountApiOptionDeliverable {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "deliverableUnits", skip_serializing_if = "Option::is_none")]
    pub deliverable_units: Option<f64>,
    #[serde(rename = "apiCurrencyType", skip_serializing_if = "Option::is_none")]
    pub api_currency_type: Option<ApiCurrencyType>,
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<trader::AssetType>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiCurrencyType {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "JPY")]
    Jpy,
}

impl Default for ApiCurrencyType {
    fn default() -> ApiCurrencyType {
        Self::Usd
    }
}
