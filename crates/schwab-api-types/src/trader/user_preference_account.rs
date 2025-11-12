use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPreferenceAccount {
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "primaryAccount", skip_serializing_if = "Option::is_none")]
    pub primary_account: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "nickName", skip_serializing_if = "Option::is_none")]
    pub nick_name: Option<String>,
    /// Green | Blue
    #[serde(rename = "accountColor", skip_serializing_if = "Option::is_none")]
    pub account_color: Option<String>,
    #[serde(rename = "displayAcctId", skip_serializing_if = "Option::is_none")]
    pub display_acct_id: Option<String>,
    #[serde(rename = "autoPositionEffect", skip_serializing_if = "Option::is_none")]
    pub auto_position_effect: Option<bool>,
}

