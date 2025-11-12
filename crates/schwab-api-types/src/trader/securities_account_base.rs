use crate::trader;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
/// Represents account and trading information.
pub enum SecuritiesAccountBase {
    #[serde(rename = "CashAccount")]
    CashAccount {
        #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
        account_number: Option<String>,
        #[serde(rename = "roundTrips", skip_serializing_if = "Option::is_none")]
        round_trips: Option<i32>,
        #[serde(rename = "isDayTrader", skip_serializing_if = "Option::is_none")]
        is_day_trader: Option<bool>,
        #[serde(
            rename = "isClosingOnlyRestricted",
            skip_serializing_if = "Option::is_none"
        )]
        is_closing_only_restricted: Option<bool>,
        #[serde(rename = "pfcbFlag", skip_serializing_if = "Option::is_none")]
        pfcb_flag: Option<bool>,
        #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
        positions: Option<Vec<trader::Position>>,
    },
    #[serde(rename = "MarginAccount")]
    MarginAccount {
        #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
        account_number: Option<String>,
        #[serde(rename = "roundTrips", skip_serializing_if = "Option::is_none")]
        round_trips: Option<i32>,
        #[serde(rename = "isDayTrader", skip_serializing_if = "Option::is_none")]
        is_day_trader: Option<bool>,
        #[serde(
            rename = "isClosingOnlyRestricted",
            skip_serializing_if = "Option::is_none"
        )]
        is_closing_only_restricted: Option<bool>,
        #[serde(rename = "pfcbFlag", skip_serializing_if = "Option::is_none")]
        pfcb_flag: Option<bool>,
        #[serde(rename = "positions", skip_serializing_if = "Option::is_none")]
        positions: Option<Vec<trader::Position>>,
    },
}

impl Default for SecuritiesAccountBase {
    fn default() -> Self {
        Self::CashAccount {
            account_number: Default::default(),
            round_trips: Default::default(),
            is_day_trader: Default::default(),
            is_closing_only_restricted: Default::default(),
            pfcb_flag: Default::default(),
            positions: Default::default(),
        }
    }
}
