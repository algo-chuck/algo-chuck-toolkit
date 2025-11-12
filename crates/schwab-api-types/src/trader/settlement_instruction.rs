use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `REGULAR`
/// - `CASH`
/// - `NEXT_DAY`
/// - `UNKNOWN`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SettlementInstruction {
    #[serde(rename = "REGULAR")]
    Regular,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "NEXT_DAY")]
    NextDay,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl std::fmt::Display for SettlementInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Regular => write!(f, "REGULAR"),
            Self::Cash => write!(f, "CASH"),
            Self::NextDay => write!(f, "NEXT_DAY"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for SettlementInstruction {
    fn default() -> SettlementInstruction {
        Self::Regular
    }
}
