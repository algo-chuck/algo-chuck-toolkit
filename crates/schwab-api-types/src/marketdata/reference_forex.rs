use serde::{Deserialize, Serialize};

/// ReferenceForex : Reference data of Forex security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceForex {
    /// Description of Instrument
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exchange Code
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Exchange Name
    #[serde(rename = "exchangeName", skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    /// is FOREX tradable
    #[serde(rename = "isTradable", skip_serializing_if = "Option::is_none")]
    pub is_tradable: Option<bool>,
    /// Market marker
    #[serde(rename = "marketMaker", skip_serializing_if = "Option::is_none")]
    pub market_maker: Option<String>,
    /// Product name
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    /// Trading hours
    #[serde(rename = "tradingHours", skip_serializing_if = "Option::is_none")]
    pub trading_hours: Option<String>,
}
