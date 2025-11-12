use crate::marketdata;
use serde::{Deserialize, Serialize};

/// ForexResponse : Quote info of Forex security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ForexResponse {
    #[serde(rename = "assetMainType", skip_serializing_if = "Option::is_none")]
    pub asset_main_type: Option<marketdata::AssetMainType>,
    /// SSID of instrument
    #[serde(rename = "ssid", skip_serializing_if = "Option::is_none")]
    pub ssid: Option<i64>,
    /// Symbol of instrument
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// is quote realtime
    #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
    pub realtime: Option<bool>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<marketdata::QuoteForex>>,
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<marketdata::ReferenceForex>>,
}
