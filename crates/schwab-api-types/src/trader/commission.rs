use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Commission {
    #[serde(rename = "commissionLegs", skip_serializing_if = "Option::is_none")]
    pub commission_legs: Option<Vec<trader::CommissionLeg>>,
}

