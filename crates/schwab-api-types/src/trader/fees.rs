use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fees {
    #[serde(rename = "feeLegs", skip_serializing_if = "Option::is_none")]
    pub fee_legs: Option<Vec<trader::FeeLeg>>,
}

