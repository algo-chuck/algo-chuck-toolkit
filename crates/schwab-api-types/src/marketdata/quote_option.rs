use serde::{Deserialize, Serialize};

/// QuoteOption : Quote data of Option security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct QuoteOption {
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
    /// Delta Value
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    /// Gamma Value
    #[serde(rename = "gamma", skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    /// Day's high trade price
    #[serde(rename = "highPrice", skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    /// Indicative Ask Price applicable only for Indicative Option Symbols
    #[serde(rename = "indAskPrice", skip_serializing_if = "Option::is_none")]
    pub ind_ask_price: Option<f64>,
    /// Indicative Bid Price applicable only for Indicative Option Symbols
    #[serde(rename = "indBidPrice", skip_serializing_if = "Option::is_none")]
    pub ind_bid_price: Option<f64>,
    /// Indicative Quote Time in milliseconds since Epoch applicable only for Indicative Option Symbols
    #[serde(rename = "indQuoteTime", skip_serializing_if = "Option::is_none")]
    pub ind_quote_time: Option<i64>,
    /// Implied Yield
    #[serde(rename = "impliedYield", skip_serializing_if = "Option::is_none")]
    pub implied_yield: Option<f64>,
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
    /// Mark Price change
    #[serde(rename = "markChange", skip_serializing_if = "Option::is_none")]
    pub mark_change: Option<f64>,
    /// Mark Price percent change
    #[serde(rename = "markPercentChange", skip_serializing_if = "Option::is_none")]
    pub mark_percent_change: Option<f64>,
    /// Money Intrinsic Value
    #[serde(
        rename = "moneyIntrinsicValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub money_intrinsic_value: Option<f64>,
    /// Current Last-Prev Close
    #[serde(rename = "netChange", skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    /// Net Percentage Change
    #[serde(rename = "netPercentChange", skip_serializing_if = "Option::is_none")]
    pub net_percent_change: Option<f64>,
    /// Open Interest
    #[serde(rename = "openInterest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    /// Price at market open
    #[serde(rename = "openPrice", skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    /// Last quote time in milliseconds since Epoch
    #[serde(rename = "quoteTime", skip_serializing_if = "Option::is_none")]
    pub quote_time: Option<i64>,
    /// Rho Value
    #[serde(rename = "rho", skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
    /// Status of security
    #[serde(rename = "securityStatus", skip_serializing_if = "Option::is_none")]
    pub security_status: Option<String>,
    /// Theoretical option Value
    #[serde(
        rename = "theoreticalOptionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub theoretical_option_value: Option<f64>,
    /// Theta Value
    #[serde(rename = "theta", skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    /// Time Value
    #[serde(rename = "timeValue", skip_serializing_if = "Option::is_none")]
    pub time_value: Option<f64>,
    /// Aggregated shares traded throughout the day, including pre/post market hours.
    #[serde(rename = "totalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
    /// Last trade time in milliseconds since Epoch
    #[serde(rename = "tradeTime", skip_serializing_if = "Option::is_none")]
    pub trade_time: Option<i64>,
    /// Underlying Price
    #[serde(rename = "underlyingPrice", skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    /// Vega Value
    #[serde(rename = "vega", skip_serializing_if = "Option::is_none")]
    pub vega: Option<f64>,
    /// Option Risk/Volatility Measurement
    #[serde(rename = "volatility", skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
}
