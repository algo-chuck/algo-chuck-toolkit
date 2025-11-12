use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarginInitialBalance {
    #[serde(rename = "accruedInterest", skip_serializing_if = "Option::is_none")]
    pub accrued_interest: Option<f64>,
    #[serde(
        rename = "availableFundsNonMarginableTrade",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_funds_non_marginable_trade: Option<f64>,
    #[serde(rename = "bondValue", skip_serializing_if = "Option::is_none")]
    pub bond_value: Option<f64>,
    #[serde(rename = "buyingPower", skip_serializing_if = "Option::is_none")]
    pub buying_power: Option<f64>,
    #[serde(rename = "cashBalance", skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<f64>,
    #[serde(
        rename = "cashAvailableForTrading",
        skip_serializing_if = "Option::is_none"
    )]
    pub cash_available_for_trading: Option<f64>,
    #[serde(rename = "cashReceipts", skip_serializing_if = "Option::is_none")]
    pub cash_receipts: Option<f64>,
    #[serde(
        rename = "dayTradingBuyingPower",
        skip_serializing_if = "Option::is_none"
    )]
    pub day_trading_buying_power: Option<f64>,
    #[serde(
        rename = "dayTradingBuyingPowerCall",
        skip_serializing_if = "Option::is_none"
    )]
    pub day_trading_buying_power_call: Option<f64>,
    #[serde(
        rename = "dayTradingEquityCall",
        skip_serializing_if = "Option::is_none"
    )]
    pub day_trading_equity_call: Option<f64>,
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<f64>,
    #[serde(rename = "equityPercentage", skip_serializing_if = "Option::is_none")]
    pub equity_percentage: Option<f64>,
    #[serde(rename = "liquidationValue", skip_serializing_if = "Option::is_none")]
    pub liquidation_value: Option<f64>,
    #[serde(rename = "longMarginValue", skip_serializing_if = "Option::is_none")]
    pub long_margin_value: Option<f64>,
    #[serde(
        rename = "longOptionMarketValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub long_option_market_value: Option<f64>,
    #[serde(rename = "longStockValue", skip_serializing_if = "Option::is_none")]
    pub long_stock_value: Option<f64>,
    #[serde(rename = "maintenanceCall", skip_serializing_if = "Option::is_none")]
    pub maintenance_call: Option<f64>,
    #[serde(
        rename = "maintenanceRequirement",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_requirement: Option<f64>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<f64>,
    #[serde(rename = "marginEquity", skip_serializing_if = "Option::is_none")]
    pub margin_equity: Option<f64>,
    #[serde(rename = "moneyMarketFund", skip_serializing_if = "Option::is_none")]
    pub money_market_fund: Option<f64>,
    #[serde(rename = "mutualFundValue", skip_serializing_if = "Option::is_none")]
    pub mutual_fund_value: Option<f64>,
    #[serde(rename = "regTCall", skip_serializing_if = "Option::is_none")]
    pub reg_t_call: Option<f64>,
    #[serde(rename = "shortMarginValue", skip_serializing_if = "Option::is_none")]
    pub short_margin_value: Option<f64>,
    #[serde(
        rename = "shortOptionMarketValue",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_option_market_value: Option<f64>,
    #[serde(rename = "shortStockValue", skip_serializing_if = "Option::is_none")]
    pub short_stock_value: Option<f64>,
    #[serde(rename = "totalCash", skip_serializing_if = "Option::is_none")]
    pub total_cash: Option<f64>,
    #[serde(rename = "isInCall", skip_serializing_if = "Option::is_none")]
    pub is_in_call: Option<f64>,
    #[serde(rename = "unsettledCash", skip_serializing_if = "Option::is_none")]
    pub unsettled_cash: Option<f64>,
    #[serde(rename = "pendingDeposits", skip_serializing_if = "Option::is_none")]
    pub pending_deposits: Option<f64>,
    #[serde(rename = "marginBalance", skip_serializing_if = "Option::is_none")]
    pub margin_balance: Option<f64>,
    #[serde(rename = "shortBalance", skip_serializing_if = "Option::is_none")]
    pub short_balance: Option<f64>,
    #[serde(rename = "accountValue", skip_serializing_if = "Option::is_none")]
    pub account_value: Option<f64>,
}

