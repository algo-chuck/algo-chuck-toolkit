use crate::marketdata;
use serde::{Deserialize, Serialize};

/// Fundamental : Fundamentals of a security
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Fundamental {
    /// Average 10 day volume
    #[serde(rename = "avg10DaysVolume", skip_serializing_if = "Option::is_none")]
    pub avg10_days_volume: Option<f64>,
    /// Average 1 day volume
    #[serde(rename = "avg1YearVolume", skip_serializing_if = "Option::is_none")]
    pub avg1_year_volume: Option<f64>,
    /// Declaration date in yyyy-mm-ddThh:mm:ssZ
    #[serde(rename = "declarationDate", skip_serializing_if = "Option::is_none")]
    pub declaration_date: Option<String>,
    /// Dividend Amount
    #[serde(rename = "divAmount", skip_serializing_if = "Option::is_none")]
    pub div_amount: Option<f64>,
    /// Dividend date in yyyy-mm-ddThh:mm:ssZ
    #[serde(rename = "divExDate", skip_serializing_if = "Option::is_none")]
    pub div_ex_date: Option<String>,
    #[serde(
        rename = "divFreq",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub div_freq: Option<Option<marketdata::DivFreq>>,
    /// Dividend Pay Amount
    #[serde(rename = "divPayAmount", skip_serializing_if = "Option::is_none")]
    pub div_pay_amount: Option<f64>,
    /// Dividend pay date in yyyy-mm-ddThh:mm:ssZ
    #[serde(rename = "divPayDate", skip_serializing_if = "Option::is_none")]
    pub div_pay_date: Option<String>,
    /// Dividend yield
    #[serde(rename = "divYield", skip_serializing_if = "Option::is_none")]
    pub div_yield: Option<f64>,
    /// Earnings per Share
    #[serde(rename = "eps", skip_serializing_if = "Option::is_none")]
    pub eps: Option<f64>,
    /// Fund Leverage Factor + > 0 <-
    #[serde(rename = "fundLeverageFactor", skip_serializing_if = "Option::is_none")]
    pub fund_leverage_factor: Option<f64>,
    #[serde(
        rename = "fundStrategy",
        default,
        with = "::serde_with::rust::double_option",
        skip_serializing_if = "Option::is_none"
    )]
    pub fund_strategy: Option<Option<marketdata::FundStrategy>>,
    /// Next Dividend date
    #[serde(rename = "nextDivExDate", skip_serializing_if = "Option::is_none")]
    pub next_div_ex_date: Option<String>,
    /// Next Dividend pay date
    #[serde(rename = "nextDivPayDate", skip_serializing_if = "Option::is_none")]
    pub next_div_pay_date: Option<String>,
    /// P/E Ratio
    #[serde(rename = "peRatio", skip_serializing_if = "Option::is_none")]
    pub pe_ratio: Option<f64>,
}
