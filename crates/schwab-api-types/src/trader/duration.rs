use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `DAY`
/// - `GOOD_TILL_CANCEL`
/// - `FILL_OR_KILL`
/// - `IMMEDIATE_OR_CANCEL`
/// - `END_OF_WEEK`
/// - `END_OF_MONTH`
/// - `NEXT_END_OF_MONTH`
/// - `UNKNOWN`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Duration {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "GOOD_TILL_CANCEL")]
    GoodTillCancel,
    #[serde(rename = "FILL_OR_KILL")]
    FillOrKill,
    #[serde(rename = "IMMEDIATE_OR_CANCEL")]
    ImmediateOrCancel,
    #[serde(rename = "END_OF_WEEK")]
    EndOfWeek,
    #[serde(rename = "END_OF_MONTH")]
    EndOfMonth,
    #[serde(rename = "NEXT_END_OF_MONTH")]
    NextEndOfMonth,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Day => write!(f, "DAY"),
            Self::GoodTillCancel => write!(f, "GOOD_TILL_CANCEL"),
            Self::FillOrKill => write!(f, "FILL_OR_KILL"),
            Self::ImmediateOrCancel => write!(f, "IMMEDIATE_OR_CANCEL"),
            Self::EndOfWeek => write!(f, "END_OF_WEEK"),
            Self::EndOfMonth => write!(f, "END_OF_MONTH"),
            Self::NextEndOfMonth => write!(f, "NEXT_END_OF_MONTH"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for Duration {
    fn default() -> Duration {
        Self::Day
    }
}
