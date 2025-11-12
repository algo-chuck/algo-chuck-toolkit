use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StreamerInfo {
    #[serde(rename = "streamerSocketUrl", skip_serializing_if = "Option::is_none")]
    pub streamer_socket_url: Option<String>,
    #[serde(
        rename = "schwabClientCustomerId",
        skip_serializing_if = "Option::is_none"
    )]
    pub schwab_client_customer_id: Option<String>,
    #[serde(
        rename = "schwabClientCorrelId",
        skip_serializing_if = "Option::is_none"
    )]
    pub schwab_client_correl_id: Option<String>,
    #[serde(
        rename = "schwabClientChannel",
        skip_serializing_if = "Option::is_none"
    )]
    pub schwab_client_channel: Option<String>,
    #[serde(
        rename = "schwabClientFunctionId",
        skip_serializing_if = "Option::is_none"
    )]
    pub schwab_client_function_id: Option<String>,
}

