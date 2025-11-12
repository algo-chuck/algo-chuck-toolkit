use serde::{Deserialize, Serialize};

/// FundStrategy "A" - Active "L" - Leveraged "P" - Passive "Q" - Quantitative "S" - Short
///
/// **Variants:**
/// - `A`
/// - `L`
/// - `P`
/// - `Q`
/// - `S`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FundStrategy {
    #[serde(rename = "A")]
    A,
    #[serde(rename = "L")]
    L,
    #[serde(rename = "P")]
    P,
    #[serde(rename = "Q")]
    Q,
    #[serde(rename = "S")]
    S,
}

impl std::fmt::Display for FundStrategy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::L => write!(f, "L"),
            Self::P => write!(f, "P"),
            Self::Q => write!(f, "Q"),
            Self::S => write!(f, "S"),
        }
    }
}

impl Default for FundStrategy {
    fn default() -> FundStrategy {
        Self::A
    }
}
