use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
///
/// **API Operations (Response):**
/// - `GET /accounts` - Get linked account(s) balances and positions for the logged in user.
/// - `GET /accounts/{accountNumber}` - Get a specific account balance and positions for the logged in user.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "securitiesAccount", skip_serializing_if = "Option::is_none")]
    pub securities_account: Option<Box<trader::SecuritiesAccount>>,
}

impl Account {
    pub fn new() -> Account {
        Account {
            securities_account: None,
        }
    }
}
