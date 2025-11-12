use serde::{Deserialize, Serialize};

/// Type from Schwab Market Data API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Interval {
    #[serde(rename = "start", skip_serializing_if = "Option::is_none")]
    pub start: Option<String>,
    #[serde(rename = "end", skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,
}

