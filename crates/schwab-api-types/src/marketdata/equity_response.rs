use crate::marketdata;
use serde::{Deserialize, Serialize};

/// EquityResponse : Quote info of Equity security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquityResponse {
    #[serde(rename = "assetMainType", skip_serializing_if = "Option::is_none")]
    pub asset_main_type: Option<marketdata::AssetMainType>,
    #[serde(
        rename = "assetSubType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub asset_sub_type: Option<Option<marketdata::EquityAssetSubType>>,
    /// SSID of instrument
    #[serde(rename = "ssid", skip_serializing_if = "Option::is_none")]
    pub ssid: Option<i64>,
    /// Symbol of instrument
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// is quote realtime
    #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
    pub realtime: Option<bool>,
    #[serde(
        rename = "quoteType",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub quote_type: Option<Option<marketdata::QuoteType>>,
    #[serde(rename = "extended", skip_serializing_if = "Option::is_none")]
    pub extended: Option<Box<marketdata::ExtendedMarket>>,
    #[serde(rename = "fundamental", skip_serializing_if = "Option::is_none")]
    pub fundamental: Option<Box<marketdata::Fundamental>>,
    #[serde(rename = "quote", skip_serializing_if = "Option::is_none")]
    pub quote: Option<Box<marketdata::QuoteEquity>>,
    #[serde(rename = "reference", skip_serializing_if = "Option::is_none")]
    pub reference: Option<Box<marketdata::ReferenceEquity>>,
    #[serde(rename = "regular", skip_serializing_if = "Option::is_none")]
    pub regular: Option<Box<marketdata::RegularMarket>>,
}
