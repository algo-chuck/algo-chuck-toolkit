use serde::{Deserialize, Serialize};

/// ReferenceIndex : Reference data of Index security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceIndex {
    /// Description of Instrument
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Exchange Code
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Exchange Name
    #[serde(rename = "exchangeName", skip_serializing_if = "Option::is_none")]
    pub exchange_name: Option<String>,
}
