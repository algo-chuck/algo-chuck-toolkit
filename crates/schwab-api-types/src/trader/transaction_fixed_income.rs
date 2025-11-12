use serde::{Deserialize, Serialize};

/// Type from Schwab Trader API.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TransactionFixedIncome {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "maturityDate", skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<String>,
    #[serde(rename = "factor", skip_serializing_if = "Option::is_none")]
    pub factor: Option<f64>,
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    #[serde(rename = "variableRate", skip_serializing_if = "Option::is_none")]
    pub variable_rate: Option<f64>,
    #[serde(rename = "assetType")]
    pub asset_type: AssetType,
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "instrumentId", skip_serializing_if = "Option::is_none")]
    pub instrument_id: Option<i64>,
    #[serde(rename = "netChange", skip_serializing_if = "Option::is_none")]
    pub net_change: Option<f64>,
}

impl TransactionFixedIncome {
    pub fn new(asset_type: AssetType) -> TransactionFixedIncome {
        TransactionFixedIncome {
            r#type: None,
            maturity_date: None,
            factor: None,
            multiplier: None,
            variable_rate: None,
            asset_type,
            cusip: None,
            symbol: None,
            description: None,
            instrument_id: None,
            net_change: None,
        }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "BOND_UNIT")]
    BondUnit,
    #[serde(rename = "CERTIFICATE_OF_DEPOSIT")]
    CertificateOfDeposit,
    #[serde(rename = "CONVERTIBLE_BOND")]
    ConvertibleBond,
    #[serde(rename = "COLLATERALIZED_MORTGAGE_OBLIGATION")]
    CollateralizedMortgageObligation,
    #[serde(rename = "CORPORATE_BOND")]
    CorporateBond,
    #[serde(rename = "GOVERNMENT_MORTGAGE")]
    GovernmentMortgage,
    #[serde(rename = "GNMA_BONDS")]
    GnmaBonds,
    #[serde(rename = "MUNICIPAL_ASSESSMENT_DISTRICT")]
    MunicipalAssessmentDistrict,
    #[serde(rename = "MUNICIPAL_BOND")]
    MunicipalBond,
    #[serde(rename = "OTHER_GOVERNMENT")]
    OtherGovernment,
    #[serde(rename = "SHORT_TERM_PAPER")]
    ShortTermPaper,
    #[serde(rename = "US_TREASURY_BOND")]
    UsTreasuryBond,
    #[serde(rename = "US_TREASURY_BILL")]
    UsTreasuryBill,
    #[serde(rename = "US_TREASURY_NOTE")]
    UsTreasuryNote,
    #[serde(rename = "US_TREASURY_ZERO_COUPON")]
    UsTreasuryZeroCoupon,
    #[serde(rename = "AGENCY_BOND")]
    AgencyBond,
    #[serde(rename = "WHEN_AS_AND_IF_ISSUED_BOND")]
    WhenAsAndIfIssuedBond,
    #[serde(rename = "ASSET_BACKED_SECURITY")]
    AssetBackedSecurity,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for Type {
    fn default() -> Type {
        Self::BondUnit
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetType {
    #[serde(rename = "EQUITY")]
    Equity,
    #[serde(rename = "OPTION")]
    Option,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "CASH_EQUIVALENT")]
    CashEquivalent,
    #[serde(rename = "FIXED_INCOME")]
    FixedIncome,
    #[serde(rename = "CURRENCY")]
    Currency,
    #[serde(rename = "COLLECTIVE_INVESTMENT")]
    CollectiveInvestment,
}

impl Default for AssetType {
    fn default() -> AssetType {
        Self::Equity
    }
}
