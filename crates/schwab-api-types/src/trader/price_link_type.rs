use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `VALUE`
/// - `PERCENT`
/// - `TICK`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PriceLinkType {
    #[serde(rename = "VALUE")]
    Value,
    #[serde(rename = "PERCENT")]
    Percent,
    #[serde(rename = "TICK")]
    Tick,
}

impl std::fmt::Display for PriceLinkType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Value => write!(f, "VALUE"),
            Self::Percent => write!(f, "PERCENT"),
            Self::Tick => write!(f, "TICK"),
        }
    }
}

impl Default for PriceLinkType {
    fn default() -> PriceLinkType {
        Self::Value
    }
}
