use crate::trader;
use serde::{Deserialize, Serialize};

/// Type from Schwab Trader API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionOption {
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "optionDeliverables", skip_serializing_if = "Option::is_none")]
    pub option_deliverables: Option<Vec<trader::TransactionApiOptionDeliverable>>,
    #[serde(
        rename = "optionPremiumMultiplier",
        skip_serializing_if = "Option::is_none"
    )]
    pub option_premium_multiplier: Option<i64>,
    #[serde(rename = "putCall", skip_serializing_if = "Option::is_none")]
    pub put_call: Option<PutCall>,
    #[serde(rename = "strikePrice", skip_serializing_if = "Option::is_none")]
    pub strike_price: Option<f64>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "underlyingSymbol", skip_serializing_if = "Option::is_none")]
    pub underlying_symbol: Option<String>,
    #[serde(rename = "underlyingCusip", skip_serializing_if = "Option::is_none")]
    pub underlying_cusip: Option<String>,
    #[serde(rename = "deliverable", skip_serializing_if = "Option::is_none")]
    pub deliverable: Option<Box<trader::TransactionInstrument>>,
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

impl TransactionOption {
    pub fn new(asset_type: AssetType) -> TransactionOption {
        TransactionOption {
            expiration_date: None,
            option_deliverables: None,
            option_premium_multiplier: None,
            put_call: None,
            strike_price: None,
            r#type: None,
            underlying_symbol: None,
            underlying_cusip: None,
            deliverable: None,
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
pub enum PutCall {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for PutCall {
    fn default() -> PutCall {
        Self::Put
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "VANILLA")]
    Vanilla,
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "BARRIER")]
    Barrier,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::Vanilla
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
