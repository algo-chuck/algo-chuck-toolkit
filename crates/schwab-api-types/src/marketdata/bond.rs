use serde::{Deserialize, Serialize};

/// Type from Schwab Market Data API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bond {
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    #[serde(rename = "assetType", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
    #[serde(rename = "bondFactor", skip_serializing_if = "Option::is_none")]
    pub bond_factor: Option<String>,
    #[serde(rename = "bondMultiplier", skip_serializing_if = "Option::is_none")]
    pub bond_multiplier: Option<String>,
    #[serde(rename = "bondPrice", skip_serializing_if = "Option::is_none")]
    pub bond_price: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "ETF")]
    Etf,
    #[serde(rename = "EXTENDED")]
    Extended,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FUTURE_OPTION")]
    FutureOption,
    #[serde(rename = "FUNDAMENTAL")]
    Fundamental,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "INDICATOR")]
    Indicator,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for AssetType {
    fn default() -> AssetType {
        Self::Bond
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "ETF")]
    Etf,
    #[serde(rename = "EXTENDED")]
    Extended,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "FUTURE")]
    Future,
    #[serde(rename = "FUTURE_OPTION")]
    FutureOption,
    #[serde(rename = "FUNDAMENTAL")]
    Fundamental,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "INDICATOR")]
    Indicator,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::Bond
    }
}
