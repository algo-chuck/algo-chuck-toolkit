use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct FeeLeg {
    #[serde(rename = "feeValues", skip_serializing_if = "Option::is_none")]
    pub fee_values: Option<Vec<trader::FeeValue>>,
}

