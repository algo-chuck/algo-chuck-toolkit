use serde::{Deserialize, Serialize};

/// QuoteMutualFund : Quote data of Mutual Fund security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuoteMutualFund {
    /// Higest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekHigh", skip_serializing_if = "Option::is_none")]
    pub param_52_week_high: Option<f64>,
    /// Lowest price traded in the past 12 months, or 52 weeks
    #[serde(rename = "52WeekLow", skip_serializing_if = "Option::is_none")]
    pub param_52_week_low: Option<f64>,
    /// Previous day's closing price
    #[serde(rename = "closePrice", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    /// Net Asset Value
    #[serde(rename = "nAV", skip_serializing_if = "Option::is_none")]
    pub n_av: Option<f64>,
    /// Current Last-Prev Close
    #[serde(rename = "netChange", skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    /// Net Percentage Change
    #[serde(rename = "netPercentChange", skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    /// Status of security
    #[serde(rename = "securityStatus", skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    #[serde(rename = "totalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    /// Last trade time in milliseconds since Epoch
    #[serde(rename = "tradeTime", skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
}
