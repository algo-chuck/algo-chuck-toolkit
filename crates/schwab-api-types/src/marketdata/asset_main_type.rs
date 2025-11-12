use serde::{Deserialize, Serialize};

/// AssetMainType : Instrument's asset type
///
/// **Variants:**
/// - `BOND`
/// - `EQUITY`
/// - `FOREX`
/// - `FUTURE`
/// - `FUTURE_OPTION`
/// - `INDEX`
/// - `MUTUAL_FUND`
/// - `OPTION`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetMainType {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FUTURE_OPTION")]
    FutureOption,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "OPTION")]
    Option,
}

impl std::fmt::Display for AssetMainType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bond => write!(f, "BOND"),
            Self::Equity => write!(f, "EQUITY"),
            Self::Forex => write!(f, "FOREX"),
            Self::Future => write!(f, "FUTURE"),
            Self::FutureOption => write!(f, "FUTURE_OPTION"),
            Self::Index => write!(f, "INDEX"),
            Self::MutualFund => write!(f, "MUTUAL_FUND"),
            Self::Option => write!(f, "OPTION"),
        }
    }
}

impl Default for AssetMainType {
    fn default() -> AssetMainType {
        Self::Bond
    }
}
