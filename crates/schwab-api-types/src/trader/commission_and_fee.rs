use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommissionAndFee {
    #[serde(rename = "commission", skip_serializing_if = "Option::is_none")]
    pub commission: Option<Box<trader::Commission>>,
    #[serde(rename = "fee", skip_serializing_if = "Option::is_none")]
    pub fee: Option<Box<trader::Fees>>,
    #[serde(rename = "trueCommission", skip_serializing_if = "Option::is_none")]
    pub true_commission: Option<Box<trader::Commission>>,
}

