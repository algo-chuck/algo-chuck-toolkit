use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderValidationDetail {
    #[serde(rename = "validationRuleName", skip_serializing_if = "Option::is_none")]
    pub validation_rule_name: Option<String>,
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "activityMessage", skip_serializing_if = "Option::is_none")]
    pub activity_message: Option<String>,
    #[serde(rename = "originalSeverity", skip_serializing_if = "Option::is_none")]
    pub original_severity: Option<trader::ApiRuleAction>,
    #[serde(rename = "overrideName", skip_serializing_if = "Option::is_none")]
    pub override_name: Option<String>,
    #[serde(rename = "overrideSeverity", skip_serializing_if = "Option::is_none")]
    pub override_severity: Option<trader::ApiRuleAction>,
}

