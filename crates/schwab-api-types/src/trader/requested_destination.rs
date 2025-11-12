use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `INET`
/// - `ECN_ARCA`
/// - `CBOE`
/// - `AMEX`
/// - `PHLX`
/// - `ISE`
/// - `BOX`
/// - `NYSE`
/// - `NASDAQ`
/// - `BATS`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum RequestedDestination {
    #[serde(rename = "INET")]
    Inet,
    #[serde(rename = "ECN_ARCA")]
    EcnArca,
    #[serde(rename = "CBOE")]
    Cboe,
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "PHLX")]
    Phlx,
    #[serde(rename = "ISE")]
    Ise,
    #[serde(rename = "BOX")]
    Box,
    #[serde(rename = "NYSE")]
    Nyse,
    #[serde(rename = "NASDAQ")]
    Nasdaq,
    #[serde(rename = "BATS")]
    Bats,
    #[serde(rename = "C2")]
    C2,
    #[serde(rename = "AUTO")]
    Auto,
}

impl std::fmt::Display for RequestedDestination {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Inet => write!(f, "INET"),
            Self::EcnArca => write!(f, "ECN_ARCA"),
            Self::Cboe => write!(f, "CBOE"),
            Self::Amex => write!(f, "AMEX"),
            Self::Phlx => write!(f, "PHLX"),
            Self::Ise => write!(f, "ISE"),
            Self::Box => write!(f, "BOX"),
            Self::Nyse => write!(f, "NYSE"),
            Self::Nasdaq => write!(f, "NASDAQ"),
            Self::Bats => write!(f, "BATS"),
            Self::C2 => write!(f, "C2"),
            Self::Auto => write!(f, "AUTO"),
        }
    }
}

impl Default for RequestedDestination {
    fn default() -> RequestedDestination {
        Self::Inet
    }
}
