use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserPreference {
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<trader::UserPreferenceAccount>>,
    #[serde(rename = "streamerInfo", skip_serializing_if = "Option::is_none")]
    pub streamer_info: Option<Vec<trader::StreamerInfo>>,
    #[serde(rename = "offers", skip_serializing_if = "Option::is_none")]
    pub offers: Option<Vec<trader::Offer>>,
}

impl UserPreference {
    pub fn new() -> UserPreference {
        UserPreference {
            accounts: None,
            streamer_info: None,
            offers: None,
        }
    }
}
