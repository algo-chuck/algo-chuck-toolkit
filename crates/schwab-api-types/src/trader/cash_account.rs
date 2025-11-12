use crate::trader;
use serde::{Deserialize, Serialize};

/// Cash account type (non-margin).
///
/// Represents a cash-only brokerage account with initial, current, and projected balance information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CashAccount {
    #[serde(rename = "initialBalances", skip_serializing_if = "Option::is_none")]
    pub initial_balances: Option<Box<trader::CashInitialBalance>>,
    #[serde(rename = "currentBalances", skip_serializing_if = "Option::is_none")]
    pub current_balances: Option<Box<trader::CashBalance>>,
    #[serde(rename = "projectedBalances", skip_serializing_if = "Option::is_none")]
    pub projected_balances: Option<Box<trader::CashBalance>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "roundTrips", skip_serializing_if = "Option::is_none")]
    pub round_trips: Option<i32>,
    #[serde(rename = "isDayTrader", skip_serializing_if = "Option::is_none")]
    pub is_day_trader: Option<bool>,
    #[serde(
        rename = "isClosingOnlyRestricted",
        skip_serializing_if = "Option::is_none"
    )]
    pub is_closing_only_restricted: Option<bool>,
    #[serde(rename = "pfcbFlag", skip_serializing_if = "Option::is_none")]
    pub pfcb_flag: Option<bool>,
    #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
    pub positions: Option<Vec<trader::Position>>,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "MARGIN")]
    Margin,
}

impl Default for Type {
    fn default() -> Type {
        Self::Cash
    }
}
