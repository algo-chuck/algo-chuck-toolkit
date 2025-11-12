use serde::{Deserialize, Serialize};

/// ReferenceEquity : Reference data of Equity security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceEquity {
    /// CUSIP of Instrument
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Description of Instrument
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exchange Code
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Exchange Name
    #[serde(rename = "exchangeName", skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    /// FSI Desc
    #[serde(rename = "fsiDesc", skip_serializing_if = "Option::is_none")]
    pub fsi_desc: Option<String>,
    /// Hard to borrow quantity.
    #[serde(rename = "htbQuantity", skip_serializing_if = "Option::is_none")]
    pub htb_quantity: Option<i32>,
    /// Hard to borrow rate.
    #[serde(rename = "htbRate", skip_serializing_if = "Option::is_none")]
    pub htb_rate: Option<f64>,
    /// is Hard to borrow security.
    #[serde(rename = "isHardToBorrow", skip_serializing_if = "Option::is_none")]
    pub is_hard_to_borrow: Option<bool>,
    /// is shortable security.
    #[serde(rename = "isShortable", skip_serializing_if = "Option::is_none")]
    pub is_shortable: Option<bool>,
    /// OTC Market Tier
    #[serde(rename = "otcMarketTier", skip_serializing_if = "Option::is_none")]
    pub otc_market_tier: Option<String>,
}
