use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderLegCollection {
    #[serde(rename = "orderLegType", skip_serializing_if = "Option::is_none")]
    pub order_leg_type: Option<OrderLegType>,
    #[serde(rename = "legId", skip_serializing_if = "Option::is_none")]
    pub leg_id: Option<i64>,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<trader::AccountsInstrument>>,
    #[serde(rename = "instruction", skip_serializing_if = "Option::is_none")]
    pub instruction: Option<trader::Instruction>,
    #[serde(rename = "positionEffect", skip_serializing_if = "Option::is_none")]
    pub position_effect: Option<PositionEffect>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(rename = "quantityType", skip_serializing_if = "Option::is_none")]
    pub quantity_type: Option<QuantityType>,
    #[serde(rename = "divCapGains", skip_serializing_if = "Option::is_none")]
    pub div_cap_gains: Option<DivCapGains>,
    #[serde(rename = "toSymbol", skip_serializing_if = "Option::is_none")]
    pub to_symbol: Option<String>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum OrderLegType {
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

impl Default for OrderLegType {
    fn default() -> OrderLegType {
        Self::Equity
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PositionEffect {
    #[serde(rename = "OPENING")]
    Opening,
    #[serde(rename = "CLOSING")]
    Closing,
    #[serde(rename = "AUTOMATIC")]
    Automatic,
}

impl Default for PositionEffect {
    fn default() -> PositionEffect {
        Self::Opening
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuantityType {
    #[serde(rename = "ALL_SHARES")]
    AllShares,
    #[serde(rename = "DOLLARS")]
    Dollars,
    #[serde(rename = "SHARES")]
    Shares,
}

impl Default for QuantityType {
    fn default() -> QuantityType {
        Self::AllShares
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DivCapGains {
    #[serde(rename = "REINVEST")]
    Reinvest,
    #[serde(rename = "PAYOUT")]
    Payout,
}

impl Default for DivCapGains {
    fn default() -> DivCapGains {
        Self::Reinvest
    }
}
