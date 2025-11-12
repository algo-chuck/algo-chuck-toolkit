use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DateParam {
    /// Valid ISO-8601 format is :<br> <code>yyyy-MM-dd'T'HH:mm:ss.SSSZ</code>
    #[serde(rename = "date", skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

