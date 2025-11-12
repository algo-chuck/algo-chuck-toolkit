use serde::{Deserialize, Serialize};

/// Enumeration type for API values.
///
/// **Variants:**
/// - `TRADE`
/// - `RECEIVE_AND_DELIVER`
/// - `DIVIDEND_OR_INTEREST`
/// - `ACH_RECEIPT`
/// - `ACH_DISBURSEMENT`
/// - `CASH_RECEIPT`
/// - `CASH_DISBURSEMENT`
/// - `ELECTRONIC_FUND`
/// - `WIRE_OUT`
/// - `WIRE_IN`
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "TRADE")]
    Trade,
    #[serde(rename = "RECEIVE_AND_DELIVER")]
    ReceiveAndDeliver,
    #[serde(rename = "DIVIDEND_OR_INTEREST")]
    DividendOrInterest,
    #[serde(rename = "ACH_RECEIPT")]
    AchReceipt,
    #[serde(rename = "ACH_DISBURSEMENT")]
    AchDisbursement,
    #[serde(rename = "CASH_RECEIPT")]
    CashReceipt,
    #[serde(rename = "CASH_DISBURSEMENT")]
    CashDisbursement,
    #[serde(rename = "ELECTRONIC_FUND")]
    ElectronicFund,
    #[serde(rename = "WIRE_OUT")]
    WireOut,
    #[serde(rename = "WIRE_IN")]
    WireIn,
    #[serde(rename = "JOURNAL")]
    Journal,
    #[serde(rename = "MEMORANDUM")]
    Memorandum,
    #[serde(rename = "MARGIN_CALL")]
    MarginCall,
    #[serde(rename = "MONEY_MARKET")]
    MoneyMarket,
    #[serde(rename = "SMA_ADJUSTMENT")]
    SmaAdjustment,
}

impl std::fmt::Display for TransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Trade => write!(f, "TRADE"),
            Self::ReceiveAndDeliver => write!(f, "RECEIVE_AND_DELIVER"),
            Self::DividendOrInterest => write!(f, "DIVIDEND_OR_INTEREST"),
            Self::AchReceipt => write!(f, "ACH_RECEIPT"),
            Self::AchDisbursement => write!(f, "ACH_DISBURSEMENT"),
            Self::CashReceipt => write!(f, "CASH_RECEIPT"),
            Self::CashDisbursement => write!(f, "CASH_DISBURSEMENT"),
            Self::ElectronicFund => write!(f, "ELECTRONIC_FUND"),
            Self::WireOut => write!(f, "WIRE_OUT"),
            Self::WireIn => write!(f, "WIRE_IN"),
            Self::Journal => write!(f, "JOURNAL"),
            Self::Memorandum => write!(f, "MEMORANDUM"),
            Self::MarginCall => write!(f, "MARGIN_CALL"),
            Self::MoneyMarket => write!(f, "MONEY_MARKET"),
            Self::SmaAdjustment => write!(f, "SMA_ADJUSTMENT"),
        }
    }
}

impl Default for TransactionType {
    fn default() -> TransactionType {
        Self::Trade
    }
}
