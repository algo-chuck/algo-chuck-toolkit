use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `DOLLARS`
/// - `SHARES`
/// - `ALL_SHARES`
/// - `PERCENTAGE`
/// - `UNKNOWN`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AmountIndicator {
    #[serde(rename = "DOLLARS")]
    Dollars,
    #[serde(rename = "SHARES")]
    Shares,
    #[serde(rename = "ALL_SHARES")]
    AllShares,
    #[serde(rename = "PERCENTAGE")]
    Percentage,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl std::fmt::Display for AmountIndicator {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Dollars => write!(f, "DOLLARS"),
            Self::Shares => write!(f, "SHARES"),
            Self::AllShares => write!(f, "ALL_SHARES"),
            Self::Percentage => write!(f, "PERCENTAGE"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for AmountIndicator {
    fn default() -> AmountIndicator {
        Self::Dollars
    }
}
