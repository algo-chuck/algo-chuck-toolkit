use serde::{Deserialize, Serialize};

/// ContractType : Indicates call or put
///
/// **Variants:**
/// - `P`
/// - `C`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ContractType {
    #[serde(rename = "P")]
    P,
    #[serde(rename = "C")]
    C,
}

impl std::fmt::Display for ContractType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::P => write!(f, "P"),
            Self::C => write!(f, "C"),
        }
    }
}

impl Default for ContractType {
    fn default() -> ContractType {
        Self::P
    }
}
