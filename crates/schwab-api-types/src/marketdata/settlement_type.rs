use serde::{Deserialize, Serialize};

/// SettlementType : option contract settlement type AM or PM
///
/// **Variants:**
/// - `A`
/// - `P`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SettlementType {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "P")]
    P,
}

impl std::fmt::Display for SettlementType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::P => write!(f, "P"),
        }
    }
}

impl Default for SettlementType {
    fn default() -> SettlementType {
        Self::A
    }
}
