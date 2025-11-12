use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Type from Schwab Market Data API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Hours {
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    #[serde(rename = "marketType", skip_serializing_if = "Option::is_none")]
    pub market_type: Option<MarketType>,
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,
    #[serde(rename = "productName", skip_serializing_if = "Option::is_none")]
    pub product_name: Option<String>,
    #[serde(rename = "isOpen", skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    #[serde(rename = "sessionHours", skip_serializing_if = "Option::is_none")]
    pub session_hours: Option<std::collections::HashMap<String, Vec<marketdata::Interval>>>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MarketType {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "ETF")]
    Etf,
    #[serde(rename = "EXTENDED")]
    Extended,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FUTURE_OPTION")]
    FutureOption,
    #[serde(rename = "FUNDAMENTAL")]
    Fundamental,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "INDICATOR")]
    Indicator,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for MarketType {
    fn default() -> MarketType {
        Self::Bond
    }
}
