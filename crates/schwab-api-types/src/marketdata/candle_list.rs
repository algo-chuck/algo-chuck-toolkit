use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Type from Schwab Market Data API.
///
/// **API Operations (Response):**
/// - `GET /pricehistory` - Get PriceHistory for a single symbol and date ranges.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CandleList {
    #[serde(rename = "candles", skip_serializing_if = "Option::is_none")]
    pub candles: Option<Vec<marketdata::Candle>>,
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
    #[serde(rename = "previousClose", skip_serializing_if = "Option::is_none")]
    pub previous_close: Option<f64>,
    #[serde(rename = "previousCloseDate", skip_serializing_if = "Option::is_none")]
    pub previous_close_date: Option<i64>,
    #[serde(
        rename = "previousCloseDateISO8601",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_close_date_iso8601: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

