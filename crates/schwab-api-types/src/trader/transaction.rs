use serde::{Deserialize, Serialize};

/// Transaction activity type discriminator.
///
/// Used to identify the type of transaction activity.
///
/// **API Operations (Response):**
/// - `GET /accounts/{accountNumber}/transactions` - Get all transactions information for a specific account.
/// - `GET /accounts/{accountNumber}/transactions/{transactionId}` - Get specific transaction information for a specific account
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "activityType")]
pub enum Transaction {
    #[serde(rename = "ACTIVITY_CORRECTION")]
    ActivityCorrection,
    #[serde(rename = "EXECUTION")]
    Execution,
    #[serde(rename = "ORDER_ACTION")]
    OrderAction,
    #[serde(rename = "TRANSFER")]
    Transfer,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Transaction {
    fn default() -> Self {
        Self::ActivityCorrection
    }
}

/// Transaction status.
///
/// Indicates the validity and processing state of a transaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "VALID")]
    Valid,
    #[serde(rename = "INVALID")]
    Invalid,
    #[serde(rename = "PENDING")]
    Pending,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Status {
    fn default() -> Self {
        Self::Valid
    }
}

/// Sub-account type within a securities account.
///
/// Identifies which sub-account a transaction affects.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SubAccount {
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "MARGIN")]
    Margin,
    #[serde(rename = "SHORT")]
    Short,
    #[serde(rename = "DIV")]
    Div,
    #[serde(rename = "INCOME")]
    Income,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for SubAccount {
    fn default() -> Self {
        Self::Cash
    }
}
