use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CashBalance {
    #[serde(
        rename = "cashAvailableForTrading",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_available_for_trading: Option<f64>,
    #[serde(
        rename = "cashAvailableForWithdrawal",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_available_for_withdrawal: Option<f64>,
    #[serde(rename = "cashCall", skip_serializing_if = "Option::is_none")]
    pub cash_call: Option<f64>,
    #[serde(
        rename = "longNonMarginableMarketValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_non_marginable_market_value: Option<f64>,
    #[serde(rename = "totalCash", skip_serializing_if = "Option::is_none")]
    pub total_cash: Option<f64>,
    #[serde(rename = "cashDebitCallValue", skip_serializing_if = "Option::is_none")]
    pub cash_debit_call_value: Option<f64>,
    #[serde(rename = "unsettledCash", skip_serializing_if = "Option::is_none")]
    pub unsettled_cash: Option<f64>,
}

