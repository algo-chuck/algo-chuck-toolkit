use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Market data information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetMovers200Response {
    #[serde(rename = "screeners", skip_serializing_if = "Option::is_none")]
    pub screeners: Option<Vec<marketdata::Screener>>,
}
