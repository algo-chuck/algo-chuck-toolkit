use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Offer {
    #[serde(rename = "level2Permissions", skip_serializing_if = "Option::is_none")]
    pub level2_permissions: Option<bool>,
    #[serde(rename = "mktDataPermission", skip_serializing_if = "Option::is_none")]
    pub mkt_data_permission: Option<String>,
}

