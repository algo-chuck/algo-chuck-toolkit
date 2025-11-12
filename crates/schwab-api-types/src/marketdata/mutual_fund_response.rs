use crate::marketdata;
use serde::{Deserialize, Serialize};

/// MutualFundResponse : Quote info of MutualFund security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MutualFundResponse {
    #[serde(rename = "assetMainType", skip_serializing_if = "Option::is_none")]
    pub asset_main_type: Option<marketdata::AssetMainType>,
    #[serde(
        rename = "assetSubType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_sub_type: Option<Option<marketdata::MutualFundAssetSubType>>,
    /// SSID of instrument
    #[serde(rename = "ssid", skip_serializing_if = "Option::is_none")]
    pub ssid: Option<i64>,
    /// Symbol of instrument
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// is quote realtime
    #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
    pub realtime: Option<bool>,
    #[serde(rename = "fundamental", skip_serializing_if = "Option::is_none")]
    pub fundamental: Option<Box<marketdata::Fundamental>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<marketdata::QuoteMutualFund>>,
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<marketdata::ReferenceMutualFund>>,
}
