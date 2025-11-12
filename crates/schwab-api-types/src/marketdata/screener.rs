use serde::{Deserialize, Serialize};

/// Screener : Security info of most moved with in an index
///
/// **API Operations (Response):**
/// - `GET /movers/{symbol_id}` - Get Movers for a specific index.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Screener {
    /// percent or value changed, by default its percent changed
    #[serde(rename = "change", skip_serializing_if = "Option::is_none")]
    pub change: Option<f64>,
    /// Name of security
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "direction", skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    /// what was last quoted price
    #[serde(rename = "last", skip_serializing_if = "Option::is_none")]
    pub last: Option<f64>,
    /// schwab security symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "totalVolume", skip_serializing_if = "Option::is_none")]
    pub total_volume: Option<i64>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::Up
    }
}
