use serde::{Deserialize, Serialize};

/// MutualFundAssetSubType : Asset Sub Type (only there if applicable)
///
/// **Variants:**
/// - `OEF`
/// - `CEF`
/// - `MMF`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MutualFundAssetSubType {
    #[serde(rename = "OEF")]
    Oef,
    #[serde(rename = "CEF")]
    Cef,
    #[serde(rename = "MMF")]
    Mmf,
}

impl std::fmt::Display for MutualFundAssetSubType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Oef => write!(f, "OEF"),
            Self::Cef => write!(f, "CEF"),
            Self::Mmf => write!(f, "MMF"),
        }
    }
}

impl Default for MutualFundAssetSubType {
    fn default() -> MutualFundAssetSubType {
        Self::Oef
    }
}
