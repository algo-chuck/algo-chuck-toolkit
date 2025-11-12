use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct CashInitialBalance {
    #[serde(rename = "accruedInterest", skip_serializing_if = "Option::is_none")]
    pub accrued_interest: Option<f64>,
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
    #[serde(rename = "cashBalance", skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<f64>,
    #[serde(rename = "bondValue", skip_serializing_if = "Option::is_none")]
    pub bond_value: Option<f64>,
    #[serde(rename = "cashReceipts", skip_serializing_if = "Option::is_none")]
    pub cash_receipts: Option<f64>,
    #[serde(rename = "liquidationValue", skip_serializing_if = "Option::is_none")]
    pub liquidation_value: Option<f64>,
    #[serde(
        rename = "longOptionMarketValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_option_market_value: Option<f64>,
    #[serde(rename = "longStockValue", skip_serializing_if = "Option::is_none")]
    pub long_stock_value: Option<f64>,
    #[serde(rename = "moneyMarketFund", skip_serializing_if = "Option::is_none")]
    pub money_market_fund: Option<f64>,
    #[serde(rename = "mutualFundValue", skip_serializing_if = "Option::is_none")]
    pub mutual_fund_value: Option<f64>,
    #[serde(
        rename = "shortOptionMarketValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_option_market_value: Option<f64>,
    #[serde(rename = "shortStockValue", skip_serializing_if = "Option::is_none")]
    pub short_stock_value: Option<f64>,
    #[serde(rename = "isInCall", skip_serializing_if = "Option::is_none")]
    pub is_in_call: Option<f64>,
    #[serde(rename = "unsettledCash", skip_serializing_if = "Option::is_none")]
    pub unsettled_cash: Option<f64>,
    #[serde(rename = "cashDebitCallValue", skip_serializing_if = "Option::is_none")]
    pub cash_debit_call_value: Option<f64>,
    #[serde(rename = "pendingDeposits", skip_serializing_if = "Option::is_none")]
    pub pending_deposits: Option<f64>,
    #[serde(rename = "accountValue", skip_serializing_if = "Option::is_none")]
    pub account_value: Option<f64>,
}

