use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `SINGLE`
/// - `CANCEL`
/// - `RECALL`
/// - `PAIR`
/// - `FLATTEN`
/// - `TWO_DAY_SWAP`
/// - `BLAST_ALL`
/// - `OCO`
/// - `TRIGGER`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderStrategyType {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "CANCEL")]
    Cancel,
    #[serde(rename = "RECALL")]
    Recall,
    #[serde(rename = "PAIR")]
    Pair,
    #[serde(rename = "FLATTEN")]
    Flatten,
    #[serde(rename = "TWO_DAY_SWAP")]
    TwoDaySwap,
    #[serde(rename = "BLAST_ALL")]
    BlastAll,
    #[serde(rename = "OCO")]
    Oco,
    #[serde(rename = "TRIGGER")]
    Trigger,
}

impl std::fmt::Display for OrderStrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Single => write!(f, "SINGLE"),
            Self::Cancel => write!(f, "CANCEL"),
            Self::Recall => write!(f, "RECALL"),
            Self::Pair => write!(f, "PAIR"),
            Self::Flatten => write!(f, "FLATTEN"),
            Self::TwoDaySwap => write!(f, "TWO_DAY_SWAP"),
            Self::BlastAll => write!(f, "BLAST_ALL"),
            Self::Oco => write!(f, "OCO"),
            Self::Trigger => write!(f, "TRIGGER"),
        }
    }
}

impl Default for OrderStrategyType {
    fn default() -> OrderStrategyType {
        Self::Single
    }
}
