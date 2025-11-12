use serde::{Deserialize, Serialize};

/// ReferenceFuture : Reference data of Future security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceFuture {
    /// Description of Instrument
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exchange Code
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Exchange Name
    #[serde(rename = "exchangeName", skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    /// Active symbol
    #[serde(rename = "futureActiveSymbol", skip_serializing_if = "Option::is_none")]
    pub future_active_symbol: Option<String>,
    /// Future expiration date in milliseconds since epoch
    #[serde(
        rename = "futureExpirationDate",
        skip_serializing_if = "Option::is_none"
    )]
    pub future_expiration_date: Option<f64>,
    /// Future is active
    #[serde(rename = "futureIsActive", skip_serializing_if = "Option::is_none")]
    pub future_is_active: Option<bool>,
    /// Future multiplier
    #[serde(rename = "futureMultiplier", skip_serializing_if = "Option::is_none")]
    pub future_multiplier: Option<f64>,
    /// Price format
    #[serde(rename = "futurePriceFormat", skip_serializing_if = "Option::is_none")]
    pub future_price_format: Option<String>,
    /// Future Settlement Price
    #[serde(
        rename = "futureSettlementPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub future_settlement_price: Option<f64>,
    /// Trading Hours
    #[serde(rename = "futureTradingHours", skip_serializing_if = "Option::is_none")]
    pub future_trading_hours: Option<String>,
    /// Futures product symbol
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
}
