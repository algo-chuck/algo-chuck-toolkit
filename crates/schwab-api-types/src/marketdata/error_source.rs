use serde::{Deserialize, Serialize};

/// ErrorSource : Who is responsible for triggering these errors.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ErrorSource {
    /// list of attributes which lead to this error message.
    #[serde(rename = "pointer", skip_serializing_if = "Option::is_none")]
    pub pointer: Option<Vec<String>>,
    /// parameter name which lead to this error message.
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    pub parameter: Option<String>,
    /// header name which lead to this error message.
    #[serde(rename = "header", skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
}
