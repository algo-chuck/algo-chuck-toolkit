use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `NONE`
/// - `COVERED`
/// - `VERTICAL`
/// - `BACK_RATIO`
/// - `CALENDAR`
/// - `DIAGONAL`
/// - `STRADDLE`
/// - `STRANGLE`
/// - `COLLAR_SYNTHETIC`
/// - `BUTTERFLY`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ComplexOrderStrategyType {
    #[serde(rename = "NONE")]
    None,
    #[serde(rename = "COVERED")]
    Covered,
    #[serde(rename = "VERTICAL")]
    Vertical,
    #[serde(rename = "BACK_RATIO")]
    BackRatio,
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "DIAGONAL")]
    Diagonal,
    #[serde(rename = "STRADDLE")]
    Straddle,
    #[serde(rename = "STRANGLE")]
    Strangle,
    #[serde(rename = "COLLAR_SYNTHETIC")]
    CollarSynthetic,
    #[serde(rename = "BUTTERFLY")]
    Butterfly,
    #[serde(rename = "CONDOR")]
    Condor,
    #[serde(rename = "IRON_CONDOR")]
    IronCondor,
    #[serde(rename = "VERTICAL_ROLL")]
    VerticalRoll,
    #[serde(rename = "COLLAR_WITH_STOCK")]
    CollarWithStock,
    #[serde(rename = "DOUBLE_DIAGONAL")]
    DoubleDiagonal,
    #[serde(rename = "UNBALANCED_BUTTERFLY")]
    UnbalancedButterfly,
    #[serde(rename = "UNBALANCED_CONDOR")]
    UnbalancedCondor,
    #[serde(rename = "UNBALANCED_IRON_CONDOR")]
    UnbalancedIronCondor,
    #[serde(rename = "UNBALANCED_VERTICAL_ROLL")]
    UnbalancedVerticalRoll,
    #[serde(rename = "MUTUAL_FUND_SWAP")]
    MutualFundSwap,
    #[serde(rename = "CUSTOM")]
    Custom,
}

impl std::fmt::Display for ComplexOrderStrategyType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "NONE"),
            Self::Covered => write!(f, "COVERED"),
            Self::Vertical => write!(f, "VERTICAL"),
            Self::BackRatio => write!(f, "BACK_RATIO"),
            Self::Calendar => write!(f, "CALENDAR"),
            Self::Diagonal => write!(f, "DIAGONAL"),
            Self::Straddle => write!(f, "STRADDLE"),
            Self::Strangle => write!(f, "STRANGLE"),
            Self::CollarSynthetic => write!(f, "COLLAR_SYNTHETIC"),
            Self::Butterfly => write!(f, "BUTTERFLY"),
            Self::Condor => write!(f, "CONDOR"),
            Self::IronCondor => write!(f, "IRON_CONDOR"),
            Self::VerticalRoll => write!(f, "VERTICAL_ROLL"),
            Self::CollarWithStock => write!(f, "COLLAR_WITH_STOCK"),
            Self::DoubleDiagonal => write!(f, "DOUBLE_DIAGONAL"),
            Self::UnbalancedButterfly => write!(f, "UNBALANCED_BUTTERFLY"),
            Self::UnbalancedCondor => write!(f, "UNBALANCED_CONDOR"),
            Self::UnbalancedIronCondor => write!(f, "UNBALANCED_IRON_CONDOR"),
            Self::UnbalancedVerticalRoll => write!(f, "UNBALANCED_VERTICAL_ROLL"),
            Self::MutualFundSwap => write!(f, "MUTUAL_FUND_SWAP"),
            Self::Custom => write!(f, "CUSTOM"),
        }
    }
}

impl Default for ComplexOrderStrategyType {
    fn default() -> ComplexOrderStrategyType {
        Self::None
    }
}
