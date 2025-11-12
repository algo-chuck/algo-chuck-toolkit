use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `ALL_OR_NONE`
/// - `DO_NOT_REDUCE`
/// - `ALL_OR_NONE_DO_NOT_REDUCE`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SpecialInstruction {
    #[serde(rename = "ALL_OR_NONE")]
    AllOrNone,
    #[serde(rename = "DO_NOT_REDUCE")]
    DoNotReduce,
    #[serde(rename = "ALL_OR_NONE_DO_NOT_REDUCE")]
    AllOrNoneDoNotReduce,
}

impl std::fmt::Display for SpecialInstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::AllOrNone => write!(f, "ALL_OR_NONE"),
            Self::DoNotReduce => write!(f, "DO_NOT_REDUCE"),
            Self::AllOrNoneDoNotReduce => write!(f, "ALL_OR_NONE_DO_NOT_REDUCE"),
        }
    }
}

impl Default for SpecialInstruction {
    fn default() -> SpecialInstruction {
        Self::AllOrNone
    }
}
