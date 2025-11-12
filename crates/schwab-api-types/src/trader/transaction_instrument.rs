use crate::trader;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "assetType")]
pub enum TransactionInstrument {
    #[serde(rename = "CASH_EQUIVALENT")]
    CashEquivalent(Box<trader::TransactionCashEquivalent>),
    #[serde(rename = "COLLECTIVE_INVESTMENT")]
    CollectiveInvestment(Box<trader::CollectiveInvestment>),
    #[serde(rename = "CURRENCY")]
    Currency(Box<trader::Currency>),
    #[serde(rename = "EQUITY")]
    Equity(Box<trader::TransactionEquity>),
    #[serde(rename = "FIXED_INCOME")]
    FixedIncome(Box<trader::TransactionFixedIncome>),
    #[serde(rename = "FOREX")]
    Forex(Box<trader::Forex>),
    #[serde(rename = "FUTURE")]
    Future(Box<trader::Future>),
    #[serde(rename = "INDEX")]
    Index(Box<trader::Index>),
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund(Box<trader::TransactionMutualFund>),
    #[serde(rename = "OPTION")]
    Option(Box<trader::TransactionOption>),
    #[serde(rename = "PRODUCT")]
    Product(Box<trader::Product>),
}

impl Default for TransactionInstrument {
    fn default() -> Self {
        Self::CashEquivalent(Default::default())
    }
}

/// Type from Schwab Trader API.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "SWEEP_VEHICLE")]
    SweepVehicle,
    #[serde(rename = "SAVINGS")]
    Savings,
    #[serde(rename = "MONEY_MARKET_FUND")]
    MoneyMarketFund,
    #[serde(rename = "UNKNOWN")]
    Unknown,
    #[serde(rename = "UNIT_INVESTMENT_TRUST")]
    UnitInvestmentTrust,
    #[serde(rename = "EXCHANGE_TRADED_FUND")]
    ExchangeTradedFund,
    #[serde(rename = "CLOSED_END_FUND")]
    ClosedEndFund,
    #[serde(rename = "INDEX")]
    Index,
    #[serde(rename = "UNITS")]
    Units,
    #[serde(rename = "COMMON_STOCK")]
    CommonStock,
    #[serde(rename = "PREFERRED_STOCK")]
    PreferredStock,
    #[serde(rename = "DEPOSITORY_RECEIPT")]
    DepositoryReceipt,
    #[serde(rename = "PREFERRED_DEPOSITORY_RECEIPT")]
    PreferredDepositoryReceipt,
    #[serde(rename = "RESTRICTED_STOCK")]
    RestrictedStock,
    #[serde(rename = "COMPONENT_UNIT")]
    ComponentUnit,
    #[serde(rename = "RIGHT")]
    Right,
    #[serde(rename = "WARRANT")]
    Warrant,
    #[serde(rename = "CONVERTIBLE_PREFERRED_STOCK")]
    ConvertiblePreferredStock,
    #[serde(rename = "CONVERTIBLE_STOCK")]
    ConvertibleStock,
    #[serde(rename = "LIMITED_PARTNERSHIP")]
    LimitedPartnership,
    #[serde(rename = "WHEN_ISSUED")]
    WhenIssued,
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
    #[serde(rename = "STANDARD")]
    Standard,
    #[serde(rename = "NBBO")]
    Nbbo,
    #[serde(rename = "TBD")]
    Tbd,
    #[serde(rename = "NOT_APPLICABLE")]
    NotApplicable,
    #[serde(rename = "OPEN_END_NON_TAXABLE")]
    OpenEndNonTaxable,
    #[serde(rename = "OPEN_END_TAXABLE")]
    OpenEndTaxable,
    #[serde(rename = "NO_LOAD_NON_TAXABLE")]
    NoLoadNonTaxable,
    #[serde(rename = "NO_LOAD_TAXABLE")]
    NoLoadTaxable,
    #[serde(rename = "VANILLA")]
    Vanilla,
    #[serde(rename = "BINARY")]
    Binary,
    #[serde(rename = "BARRIER")]
    Barrier,
}

impl Default for Type {
    fn default() -> Type {
        Self::SweepVehicle
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PutCall {
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "CALL")]
    Call,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for PutCall {
    fn default() -> PutCall {
        Self::Put
    }
}
