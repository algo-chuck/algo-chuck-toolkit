use serde::{Deserialize, Serialize};

/// ReferenceMutualFund : Reference data of MutualFund security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReferenceMutualFund {
    /// CUSIP of Instrument
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
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
