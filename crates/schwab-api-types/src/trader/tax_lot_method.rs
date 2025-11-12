use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `FIFO`
/// - `LIFO`
/// - `HIGH_COST`
/// - `LOW_COST`
/// - `AVERAGE_COST`
/// - `SPECIFIC_LOT`
/// - `LOSS_HARVESTER`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxLotMethod {
    #[serde(rename = "FIFO")]
    Fifo,
    #[serde(rename = "LIFO")]
    Lifo,
    #[serde(rename = "HIGH_COST")]
    HighCost,
    #[serde(rename = "LOW_COST")]
    LowCost,
    #[serde(rename = "AVERAGE_COST")]
    AverageCost,
    #[serde(rename = "SPECIFIC_LOT")]
    SpecificLot,
    #[serde(rename = "LOSS_HARVESTER")]
    LossHarvester,
}

impl std::fmt::Display for TaxLotMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Fifo => write!(f, "FIFO"),
            Self::Lifo => write!(f, "LIFO"),
            Self::HighCost => write!(f, "HIGH_COST"),
            Self::LowCost => write!(f, "LOW_COST"),
            Self::AverageCost => write!(f, "AVERAGE_COST"),
            Self::SpecificLot => write!(f, "SPECIFIC_LOT"),
            Self::LossHarvester => write!(f, "LOSS_HARVESTER"),
        }
    }
}

impl Default for TaxLotMethod {
    fn default() -> TaxLotMethod {
        Self::Fifo
    }
}
