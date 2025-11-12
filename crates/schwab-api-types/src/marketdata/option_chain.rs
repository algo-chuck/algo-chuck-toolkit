use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Type from Schwab Market Data API.
///
/// **API Operations (Response):**
/// - `GET /chains` - Get option chain for an optionable Symbol
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OptionChain {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "underlying", skip_serializing_if = "Option::is_none")]
    pub underlying: Option<Box<marketdata::Underlying>>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(rename = "interval", skip_serializing_if = "Option::is_none")]
    pub interval: Option<f64>,
    #[serde(rename = "isDelayed", skip_serializing_if = "Option::is_none")]
    pub is_delayed: Option<bool>,
    #[serde(rename = "isIndex", skip_serializing_if = "Option::is_none")]
    pub is_index: Option<bool>,
    #[serde(rename = "daysToExpiration", skip_serializing_if = "Option::is_none")]
    pub days_to_expiration: Option<f64>,
    #[serde(rename = "interestRate", skip_serializing_if = "Option::is_none")]
    pub interest_rate: Option<f64>,
    #[serde(rename = "underlyingPrice", skip_serializing_if = "Option::is_none")]
    pub underlying_price: Option<f64>,
    #[serde(rename = "volatility", skip_serializing_if = "Option::is_none")]
    pub volatility: Option<f64>,
    #[serde(rename = "callExpDateMap", skip_serializing_if = "Option::is_none")]
    pub call_exp_date_map: Option<
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, marketdata::OptionContract>,
        >,
    >,
    #[serde(rename = "putExpDateMap", skip_serializing_if = "Option::is_none")]
    pub put_exp_date_map: Option<
        std::collections::HashMap<
            String,
            std::collections::HashMap<String, marketdata::OptionContract>,
        >,
    >,
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "SINGLE")]
    Single,
    #[serde(rename = "ANALYTICAL")]
    Analytical,
    #[serde(rename = "COVERED")]
    Covered,
    #[serde(rename = "VERTICAL")]
    Vertical,
    #[serde(rename = "CALENDAR")]
    Calendar,
    #[serde(rename = "STRANGLE")]
    Strangle,
    #[serde(rename = "STRADDLE")]
    Straddle,
    #[serde(rename = "BUTTERFLY")]
    Butterfly,
    #[serde(rename = "CONDOR")]
    Condor,
    #[serde(rename = "DIAGONAL")]
    Diagonal,
    #[serde(rename = "COLLAR")]
    Collar,
    #[serde(rename = "ROLL")]
    Roll,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::Single
    }
}
