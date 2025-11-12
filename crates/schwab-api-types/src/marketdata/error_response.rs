use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Market data information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<marketdata::Error>>,
}

