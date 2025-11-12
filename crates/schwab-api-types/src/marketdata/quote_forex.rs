use serde::{Deserialize, Serialize};

/// QuoteForex : Quote data of Forex security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuoteForex {
    /// Higest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub param_52_week_high: Option<f64>,
    /// Lowest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub param_52_week_low: Option<f64>,
    /// Current Best Ask Price
    #[serde(rename = "askPrice", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    /// Number of shares for ask
    #[serde(rename = "askSize", skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    /// Current Best Bid Price
    #[serde(rename = "bidPrice", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    /// Number of shares for bid
    #[serde(rename = "bidSize", skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    /// Previous day's closing price
    #[serde(rename = "closePrice", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    /// Day's high trade price
    #[serde(rename = "highPrice", skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(rename = "lastPrice", skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    /// Number of shares traded with last trade
    #[serde(rename = "lastSize", skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    /// Day's low trade price
    #[serde(rename = "lowPrice", skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    /// Mark price
    #[serde(rename = "mark", skip_serializing_if = "Option::is_none")]
    pub mark: Option<f64>,
    /// Current Last-Prev Close
    #[serde(rename = "netChange", skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    /// Net Percentage Change
    #[serde(rename = "netPercentChange", skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    /// Price at market open
    #[serde(rename = "openPrice", skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    /// Last quote time in milliseconds since Epoch
    #[serde(rename = "quoteTime", skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    /// Status of security
    #[serde(rename = "securityStatus", skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    /// Tick Price
    #[serde(rename = "tick", skip_serializing_if = "Option::is_none")]
    pub tick: Option<f64>,
    /// Tick Amount
    #[serde(rename = "tickAmount", skip_serializing_if = "Option::is_none")]
    pub tick_amount: Option<f64>,
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    #[serde(rename = "totalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    /// Last trade time in milliseconds since Epoch
    #[serde(rename = "tradeTime", skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}
