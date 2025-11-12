use serde::{Deserialize, Serialize};

/// Market data information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Candle {
    #[serde(rename = "close", skip_serializing_if = "Option::is_none")]
    pub close: Option<f64>,
    #[serde(rename = "datetime", skip_serializing_if = "Option::is_none")]
    pub datetime: Option<i64>,
    #[serde(rename = "datetimeISO8601", skip_serializing_if = "Option::is_none")]
    pub datetime_iso8601: Option<String>,
    #[serde(rename = "high", skip_serializing_if = "Option::is_none")]
    pub high: Option<f64>,
    #[serde(rename = "low", skip_serializing_if = "Option::is_none")]
    pub low: Option<f64>,
    #[serde(rename = "open", skip_serializing_if = "Option::is_none")]
    pub open: Option<f64>,
    #[serde(rename = "volume", skip_serializing_if = "Option::is_none")]
    pub volume: Option<i64>,
}

