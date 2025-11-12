use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `BUY`
/// - `SELL`
/// - `BUY_TO_COVER`
/// - `SELL_SHORT`
/// - `BUY_TO_OPEN`
/// - `BUY_TO_CLOSE`
/// - `SELL_TO_OPEN`
/// - `SELL_TO_CLOSE`
/// - `EXCHANGE`
/// - `SELL_SHORT_EXEMPT`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Instruction {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
    #[serde(rename = "BUY_TO_COVER")]
    BuyToCover,
    #[serde(rename = "SELL_SHORT")]
    SellShort,
    #[serde(rename = "BUY_TO_OPEN")]
    BuyToOpen,
    #[serde(rename = "BUY_TO_CLOSE")]
    BuyToClose,
    #[serde(rename = "SELL_TO_OPEN")]
    SellToOpen,
    #[serde(rename = "SELL_TO_CLOSE")]
    SellToClose,
    #[serde(rename = "EXCHANGE")]
    Exchange,
    #[serde(rename = "SELL_SHORT_EXEMPT")]
    SellShortExempt,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Buy => write!(f, "BUY"),
            Self::Sell => write!(f, "SELL"),
            Self::BuyToCover => write!(f, "BUY_TO_COVER"),
            Self::SellShort => write!(f, "SELL_SHORT"),
            Self::BuyToOpen => write!(f, "BUY_TO_OPEN"),
            Self::BuyToClose => write!(f, "BUY_TO_CLOSE"),
            Self::SellToOpen => write!(f, "SELL_TO_OPEN"),
            Self::SellToClose => write!(f, "SELL_TO_CLOSE"),
            Self::Exchange => write!(f, "EXCHANGE"),
            Self::SellShortExempt => write!(f, "SELL_SHORT_EXEMPT"),
        }
    }
}

impl Default for Instruction {
    fn default() -> Instruction {
        Self::Buy
    }
}
