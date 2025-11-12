use serde::{Deserialize, Serialize};

/// Option expiration cycle type.
///
/// - `M` - End Of Month Expiration (last business day of the month)
/// - `Q` - Quarterly expirations (last business day of MAR/JUN/SEP/DEC)
/// - `W` - Weekly expiration (Friday Short Term Expirations)
/// - `S` - Expires 3rd Friday of the month (regular options)
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ExpirationType {
    #[serde(rename = "M")]
    M,
    #[serde(rename = "Q")]
    Q,
    #[serde(rename = "S")]
    S,
    #[serde(rename = "W")]
    W,
}

impl std::fmt::Display for ExpirationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::M => write!(f, "M"),
            Self::Q => write!(f, "Q"),
            Self::S => write!(f, "S"),
            Self::W => write!(f, "W"),
        }
    }
}

impl Default for ExpirationType {
    fn default() -> ExpirationType {
        Self::M
    }
}
