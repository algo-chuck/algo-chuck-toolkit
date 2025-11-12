use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Type from Schwab Market Data API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionContract {
    #[serde(rename = "putCall", skip_serializing_if = "Option::is_none")]
    pub put_call: Option<PutCall>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "exchangeName", skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
    #[serde(rename = "bidPrice", skip_serializing_if = "Option::is_none")]
    pub bid_price: Option<f64>,
    #[serde(rename = "askPrice", skip_serializing_if = "Option::is_none")]
    pub ask_price: Option<f64>,
    #[serde(rename = "lastPrice", skip_serializing_if = "Option::is_none")]
    pub last_price: Option<f64>,
    #[serde(rename = "markPrice", skip_serializing_if = "Option::is_none")]
    pub mark_price: Option<f64>,
    #[serde(rename = "bidSize", skip_serializing_if = "Option::is_none")]
    pub bid_size: Option<i32>,
    #[serde(rename = "askSize", skip_serializing_if = "Option::is_none")]
    pub ask_size: Option<i32>,
    #[serde(rename = "lastSize", skip_serializing_if = "Option::is_none")]
    pub last_size: Option<i32>,
    #[serde(rename = "highPrice", skip_serializing_if = "Option::is_none")]
    pub high_price: Option<f64>,
    #[serde(rename = "lowPrice", skip_serializing_if = "Option::is_none")]
    pub low_price: Option<f64>,
    #[serde(rename = "openPrice", skip_serializing_if = "Option::is_none")]
    pub open_price: Option<f64>,
    #[serde(rename = "closePrice", skip_serializing_if = "Option::is_none")]
    pub close_price: Option<f64>,
    #[serde(rename = "totalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i32>,
    #[serde(rename = "tradeDate", skip_serializing_if = "Option::is_none")]
    pub trade_date: Option<f64>,
    #[serde(rename = "quoteTimeInLong", skip_serializing_if = "Option::is_none")]
    pub quote_time_in_long: Option<i32>,
    #[serde(rename = "tradeTimeInLong", skip_serializing_if = "Option::is_none")]
    pub trade_time_in_long: Option<i32>,
    #[serde(rename = "netChange", skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
    #[serde(rename = "volatility", skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
    #[serde(rename = "delta", skip_serializing_if = "Option::is_none")]
    pub delta: Option<f64>,
    #[serde(rename = "gamma", skip_serializing_if = "Option::is_none")]
    pub gamma: Option<f64>,
    #[serde(rename = "theta", skip_serializing_if = "Option::is_none")]
    pub theta: Option<f64>,
    #[serde(rename = "vega", skip_serializing_if = "Option::is_none")]
    pub vega: Option<f64>,
    #[serde(rename = "rho", skip_serializing_if = "Option::is_none")]
    pub rho: Option<f64>,
    #[serde(rename = "timeValue", skip_serializing_if = "Option::is_none")]
    pub time_value: Option<f64>,
    #[serde(rename = "openInterest", skip_serializing_if = "Option::is_none")]
    pub open_interest: Option<f64>,
    #[serde(rename = "isInTheMoney", skip_serializing_if = "Option::is_none")]
    pub is_in_the_money: Option<bool>,
    #[serde(
        rename = "theoreticalOptionValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub theoretical_option_value: Option<f64>,
    #[serde(
        rename = "theoreticalVolatility",
        skip_serializing_if = "Option::is_none"
    )]
    pub theoretical_volatility: Option<f64>,
    #[serde(rename = "isMini", skip_serializing_if = "Option::is_none")]
    pub is_mini: Option<bool>,
    #[serde(rename = "isNonStandard", skip_serializing_if = "Option::is_none")]
    pub is_non_standard: Option<bool>,
    #[serde(
        rename = "optionDeliverablesList",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_deliverables_list: Option<Vec<marketdata::OptionDeliverables>>,
    #[serde(rename = "strikePrice", skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "daysToExpiration", skip_serializing_if = "Option::is_none")]
    pub days_to_expiration: Option<f64>,
    #[serde(rename = "expirationType", skip_serializing_if = "Option::is_none")]
    pub expiration_type: Option<marketdata::ExpirationType>,
    #[serde(rename = "lastTradingDay", skip_serializing_if = "Option::is_none")]
    pub last_trading_day: Option<f64>,
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(rename = "settlementType", skip_serializing_if = "Option::is_none")]
    pub settlement_type: Option<marketdata::SettlementType>,
    #[serde(rename = "deliverableNote", skip_serializing_if = "Option::is_none")]
    pub deliverable_note: Option<String>,
    #[serde(rename = "isIndexOption", skip_serializing_if = "Option::is_none")]
    pub is_index_option: Option<bool>,
    #[serde(rename = "percentChange", skip_serializing_if = "Option::is_none")]
    pub percent_change: Option<f64>,
    #[serde(rename = "markChange", skip_serializing_if = "Option::is_none")]
    pub mark_change: Option<f64>,
    #[serde(rename = "markPercentChange", skip_serializing_if = "Option::is_none")]
    pub mark_percent_change: Option<f64>,
    #[serde(rename = "isPennyPilot", skip_serializing_if = "Option::is_none")]
    pub is_penny_pilot: Option<bool>,
    #[serde(rename = "intrinsicValue", skip_serializing_if = "Option::is_none")]
    pub intrinsic_value: Option<f64>,
    #[serde(rename = "optionRoot", skip_serializing_if = "Option::is_none")]
    pub option_root: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PutCall {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
}

impl Default for PutCall {
    fn default() -> PutCall {
        Self::Put
    }
}
