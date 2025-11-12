use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `STANDARD`
/// - `BID`
/// - `ASK`
/// - `LAST`
/// - `MARK`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StopType {
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
    #[serde(rename = "LAST")]
    Last,
    #[serde(rename = "MARK")]
    Mark,
}

impl std::fmt::Display for StopType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Standard => write!(f, "STANDARD"),
            Self::Bid => write!(f, "BID"),
            Self::Ask => write!(f, "ASK"),
            Self::Last => write!(f, "LAST"),
            Self::Mark => write!(f, "MARK"),
        }
    }
}

impl Default for StopType {
    fn default() -> StopType {
        Self::Standard
    }
}
