use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `COMMISSION`
/// - `SEC_FEE`
/// - `STR_FEE`
/// - `R_FEE`
/// - `CDSC_FEE`
/// - `OPT_REG_FEE`
/// - `ADDITIONAL_FEE`
/// - `MISCELLANEOUS_FEE`
/// - `FTT`
/// - `FUTURES_CLEARING_FEE`
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
    #[serde(rename = "FTT")]
    Ftt,
    #[serde(rename = "FUTURES_CLEARING_FEE")]
    FuturesClearingFee,
    #[serde(rename = "FUTURES_DESK_OFFICE_FEE")]
    FuturesDeskOfficeFee,
    #[serde(rename = "FUTURES_EXCHANGE_FEE")]
    FuturesExchangeFee,
    #[serde(rename = "FUTURES_GLOBEX_FEE")]
    FuturesGlobexFee,
    #[serde(rename = "FUTURES_NFA_FEE")]
    FuturesNfaFee,
    #[serde(rename = "FUTURES_PIT_BROKERAGE_FEE")]
    FuturesPitBrokerageFee,
    #[serde(rename = "FUTURES_TRANSACTION_FEE")]
    FuturesTransactionFee,
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
    #[serde(rename = "TEFRA_TAX")]
    TefraTax,
    #[serde(rename = "STATE_TAX")]
    StateTax,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl std::fmt::Display for FeeType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Commission => write!(f, "COMMISSION"),
            Self::SecFee => write!(f, "SEC_FEE"),
            Self::StrFee => write!(f, "STR_FEE"),
            Self::RFee => write!(f, "R_FEE"),
            Self::CdscFee => write!(f, "CDSC_FEE"),
            Self::OptRegFee => write!(f, "OPT_REG_FEE"),
            Self::AdditionalFee => write!(f, "ADDITIONAL_FEE"),
            Self::MiscellaneousFee => write!(f, "MISCELLANEOUS_FEE"),
            Self::Ftt => write!(f, "FTT"),
            Self::FuturesClearingFee => write!(f, "FUTURES_CLEARING_FEE"),
            Self::FuturesDeskOfficeFee => write!(f, "FUTURES_DESK_OFFICE_FEE"),
            Self::FuturesExchangeFee => write!(f, "FUTURES_EXCHANGE_FEE"),
            Self::FuturesGlobexFee => write!(f, "FUTURES_GLOBEX_FEE"),
            Self::FuturesNfaFee => write!(f, "FUTURES_NFA_FEE"),
            Self::FuturesPitBrokerageFee => write!(f, "FUTURES_PIT_BROKERAGE_FEE"),
            Self::FuturesTransactionFee => write!(f, "FUTURES_TRANSACTION_FEE"),
            Self::LowProceedsCommission => write!(f, "LOW_PROCEEDS_COMMISSION"),
            Self::BaseCharge => write!(f, "BASE_CHARGE"),
            Self::GeneralCharge => write!(f, "GENERAL_CHARGE"),
            Self::GstFee => write!(f, "GST_FEE"),
            Self::TafFee => write!(f, "TAF_FEE"),
            Self::IndexOptionFee => write!(f, "INDEX_OPTION_FEE"),
            Self::TefraTax => write!(f, "TEFRA_TAX"),
            Self::StateTax => write!(f, "STATE_TAX"),
            Self::Unknown => write!(f, "UNKNOWN"),
        }
    }
}

impl Default for FeeType {
    fn default() -> FeeType {
        Self::Commission
    }
}
