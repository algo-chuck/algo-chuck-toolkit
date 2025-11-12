use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "ADVISOR_USER")]
    AdvisorUser,
    #[serde(rename = "BROKER_USER")]
    BrokerUser,
    #[serde(rename = "CLIENT_USER")]
    ClientUser,
    #[serde(rename = "SYSTEM_USER")]
    SystemUser,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Type {
    fn default() -> Self {
        Self::AdvisorUser
    }
}
