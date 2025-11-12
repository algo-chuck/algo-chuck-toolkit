use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `EQUITY`
/// - `MUTUAL_FUND`
/// - `OPTION`
/// - `FUTURE`
/// - `FOREX`
/// - `INDEX`
/// - `CASH_EQUIVALENT`
/// - `FIXED_INCOME`
/// - `PRODUCT`
/// - `CURRENCY`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "CASH_EQUIVALENT")]
    CashEquivalent,
    #[serde(rename = "FIXED_INCOME")]
    FixedIncome,
    #[serde(rename = "PRODUCT")]
    Product,
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "COLLECTIVE_INVESTMENT")]
    CollectiveInvestment,
}

impl std::fmt::Display for AssetType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Equity => write!(f, "EQUITY"),
            Self::MutualFund => write!(f, "MUTUAL_FUND"),
            Self::Option => write!(f, "OPTION"),
            Self::Future => write!(f, "FUTURE"),
            Self::Forex => write!(f, "FOREX"),
            Self::Index => write!(f, "INDEX"),
            Self::CashEquivalent => write!(f, "CASH_EQUIVALENT"),
            Self::FixedIncome => write!(f, "FIXED_INCOME"),
            Self::Product => write!(f, "PRODUCT"),
            Self::Currency => write!(f, "CURRENCY"),
            Self::CollectiveInvestment => write!(f, "COLLECTIVE_INVESTMENT"),
        }
    }
}

impl Default for AssetType {
    fn default() -> AssetType {
        Self::Equity
    }
}
