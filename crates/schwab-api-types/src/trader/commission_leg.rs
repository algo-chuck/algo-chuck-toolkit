use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CommissionLeg {
    #[serde(rename = "commissionValues", skip_serializing_if = "Option::is_none")]
    pub commission_values: Option<Vec<trader::CommissionValue>>,
}

