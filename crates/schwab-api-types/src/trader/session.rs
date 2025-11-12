use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `NORMAL`
/// - `AM`
/// - `PM`
/// - `SEAMLESS`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Session {
    #[serde(rename = "NORMAL")]
    Normal,
    #[serde(rename = "AM")]
    Am,
    #[serde(rename = "PM")]
    Pm,
    #[serde(rename = "SEAMLESS")]
    Seamless,
}

impl std::fmt::Display for Session {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "NORMAL"),
            Self::Am => write!(f, "AM"),
            Self::Pm => write!(f, "PM"),
            Self::Seamless => write!(f, "SEAMLESS"),
        }
    }
}

impl Default for Session {
    fn default() -> Session {
        Self::Normal
    }
}
