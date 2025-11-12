use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Market data information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetInstruments200Response {
    #[serde(rename = "instruments", skip_serializing_if = "Option::is_none")]
    pub instruments: Option<Vec<marketdata::InstrumentResponse>>,
}
