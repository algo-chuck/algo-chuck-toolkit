use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `MANUAL`
/// - `BASE`
/// - `TRIGGER`
/// - `LAST`
/// - `BID`
/// - `ASK`
/// - `ASK_BID`
/// - `MARK`
/// - `AVERAGE`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum StopPriceLinkBasis {
    #[serde(rename = "MANUAL")]
    Manual,
    #[serde(rename = "BASE")]
    Base,
    #[serde(rename = "TRIGGER")]
    Trigger,
    #[serde(rename = "LAST")]
    Last,
    #[serde(rename = "BID")]
    Bid,
    #[serde(rename = "ASK")]
    Ask,
    #[serde(rename = "ASK_BID")]
    AskBid,
    #[serde(rename = "MARK")]
    Mark,
    #[serde(rename = "AVERAGE")]
    Average,
}

impl std::fmt::Display for StopPriceLinkBasis {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Manual => write!(f, "MANUAL"),
            Self::Base => write!(f, "BASE"),
            Self::Trigger => write!(f, "TRIGGER"),
            Self::Last => write!(f, "LAST"),
            Self::Bid => write!(f, "BID"),
            Self::Ask => write!(f, "ASK"),
            Self::AskBid => write!(f, "ASK_BID"),
            Self::Mark => write!(f, "MARK"),
            Self::Average => write!(f, "AVERAGE"),
        }
    }
}

impl Default for StopPriceLinkBasis {
    fn default() -> StopPriceLinkBasis {
        Self::Manual
    }
}
