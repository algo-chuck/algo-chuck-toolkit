use crate::trader;
use serde::{Deserialize, Serialize};

/// Securities account type - either margin or cash.
///
/// Tagged enum that discriminates between margin and cash account types.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SecuritiesAccount {
    #[serde(rename = "MARGIN")]
    Margin(Box<trader::MarginAccount>),
    #[serde(rename = "CASH")]
    Cash(Box<trader::CashAccount>),
}

impl Default for SecuritiesAccount {
    fn default() -> Self {
        Self::Margin(Default::default())
    }
}
