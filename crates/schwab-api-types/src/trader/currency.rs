use serde::{Deserialize, Serialize};

/// Type from Schwab Trader API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Currency {
    #[serde(rename = "assetType")]
    pub asset_type: AssetType,
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "instrumentId", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<i64>,
    #[serde(rename = "netChange", skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
}

impl Currency {
    pub fn new(asset_type: AssetType) -> Currency {
        Currency {
            asset_type,
            cusip: None,
            symbol: None,
            description: None,
            instrument_id: None,
            net_change: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "CASH_EQUIVALENT")]
    CashEquivalent,
    #[serde(rename = "FIXED_INCOME")]
    FixedIncome,
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "COLLECTIVE_INVESTMENT")]
    CollectiveInvestment,
}

impl Default for AssetType {
    fn default() -> AssetType {
        Self::Equity
    }
}
