use serde::{Deserialize, Serialize};

/// RegularMarket : Market info of security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegularMarket {
    /// Regular market last price
    #[serde(
        rename = "regularMarketLastPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub regular_market_last_price: Option<f64>,
    /// Regular market last size
    #[serde(
        rename = "regularMarketLastSize",
        skip_serializing_if = "Option::is_none"
    )]
    pub regular_market_last_size: Option<i32>,
    /// Regular market net change
    #[serde(
        rename = "regularMarketNetChange",
        skip_serializing_if = "Option::is_none"
    )]
    pub regular_market_net_change: Option<f64>,
    /// Regular market percent change
    #[serde(
        rename = "regularMarketPercentChange",
        skip_serializing_if = "Option::is_none"
    )]
    pub regular_market_percent_change: Option<f64>,
    /// Regular market trade time in milliseconds since Epoch
    #[serde(
        rename = "regularMarketTradeTime",
        skip_serializing_if = "Option::is_none"
    )]
    pub regular_market_trade_time: Option<i64>,
}
