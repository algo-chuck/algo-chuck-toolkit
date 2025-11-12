use serde::{Deserialize, Serialize};

/// QuoteType : NBBO - realtime, NFL - Non-fee liable quote.
///
/// **Variants:**
/// - `NBBO`
/// - `NFL`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum QuoteType {
    #[serde(rename = "NBBO")]
    Nbbo,
    #[serde(rename = "NFL")]
    Nfl,
}

impl std::fmt::Display for QuoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Nbbo => write!(f, "NBBO"),
            Self::Nfl => write!(f, "NFL"),
        }
    }
}

impl Default for QuoteType {
    fn default() -> QuoteType {
        Self::Nbbo
    }
}
