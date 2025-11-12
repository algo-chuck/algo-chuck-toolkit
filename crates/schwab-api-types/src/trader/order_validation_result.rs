use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderValidationResult {
    #[serde(rename = "alerts", skip_serializing_if = "Option::is_none")]
    pub alerts: Option<Vec<trader::OrderValidationDetail>>,
    #[serde(rename = "accepts", skip_serializing_if = "Option::is_none")]
    pub accepts: Option<Vec<trader::OrderValidationDetail>>,
    #[serde(rename = "rejects", skip_serializing_if = "Option::is_none")]
    pub rejects: Option<Vec<trader::OrderValidationDetail>>,
    #[serde(rename = "reviews", skip_serializing_if = "Option::is_none")]
    pub reviews: Option<Vec<trader::OrderValidationDetail>>,
    #[serde(rename = "warns", skip_serializing_if = "Option::is_none")]
    pub warns: Option<Vec<trader::OrderValidationDetail>>,
}

