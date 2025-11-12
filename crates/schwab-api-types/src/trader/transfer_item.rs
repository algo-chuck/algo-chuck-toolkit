use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FeeType {
    #[serde(rename = "COMMISSION")]
    Commission,
    #[serde(rename = "SEC_FEE")]
    SecFee,
    #[serde(rename = "STR_FEE")]
    StrFee,
    #[serde(rename = "R_FEE")]
    RFee,
    #[serde(rename = "CDSC_FEE")]
    CdscFee,
    #[serde(rename = "OPT_REG_FEE")]
    OptRegFee,
    #[serde(rename = "ADDITIONAL_FEE")]
    AdditionalFee,
    #[serde(rename = "MISCELLANEOUS_FEE")]
    MiscellaneousFee,
    #[serde(rename = "FUTURES_EXCHANGE_FEE")]
    FuturesExchangeFee,
    #[serde(rename = "LOW_PROCEEDS_COMMISSION")]
    LowProceedsCommission,
    #[serde(rename = "BASE_CHARGE")]
    BaseCharge,
    #[serde(rename = "GENERAL_CHARGE")]
    GeneralCharge,
    #[serde(rename = "GST_FEE")]
    GstFee,
    #[serde(rename = "TAF_FEE")]
    TafFee,
    #[serde(rename = "INDEX_OPTION_FEE")]
    IndexOptionFee,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for FeeType {
    fn default() -> Self {
        Self::Commission
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PositionEffect {
    #[serde(rename = "OPENING")]
    Opening,
    #[serde(rename = "CLOSING")]
    Closing,
    #[serde(rename = "AUTOMATIC")]
    Automatic,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for PositionEffect {
    fn default() -> Self {
        Self::Opening
    }
}
