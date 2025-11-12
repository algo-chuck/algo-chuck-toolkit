use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `ACCEPT`
/// - `ALERT`
/// - `REJECT`
/// - `REVIEW`
/// - `UNKNOWN`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ApiRuleAction {
    #[serde(rename = "ACCEPT")]
    Accept,
    #[serde(rename = "ALERT")]
    Alert,
    #[serde(rename = "REJECT")]
    Reject,
    #[serde(rename = "REVIEW")]
    Review,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl std::fmt::Display for ApiRuleAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Accept => write!(f, "ACCEPT"),
            Self::Alert => write!(f, "ALERT"),
            Self::Reject => write!(f, "REJECT"),
            Self::Review => write!(f, "REVIEW"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for ApiRuleAction {
    fn default() -> ApiRuleAction {
        Self::Accept
    }
}
