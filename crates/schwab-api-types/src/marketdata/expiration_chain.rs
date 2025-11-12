use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Market data information.
///
/// **API Operations (Response):**
/// - `GET /expirationchain` - Get option expiration chain for an optionable symbol
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpirationChain {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "expirationList", skip_serializing_if = "Option::is_none")]
    pub expiration_list: Option<Vec<marketdata::Expiration>>,
}

