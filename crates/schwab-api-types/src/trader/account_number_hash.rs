use serde::{Deserialize, Serialize};

/// Represents account and trading information.
///
/// **API Operations (Response):**
/// - `GET /accounts/accountNumbers` - Get list of account numbers and their encrypted values
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountNumberHash {
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "hashValue", skip_serializing_if = "Option::is_none")]
    pub hash_value: Option<String>,
}

