use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct MarginBalance {
    #[serde(rename = "availableFunds", skip_serializing_if = "Option::is_none")]
    pub available_funds: Option<f64>,
    #[serde(
        rename = "availableFundsNonMarginableTrade",
        skip_serializing_if = "Option::is_none"
    )]
    pub available_funds_non_marginable_trade: Option<f64>,
    #[serde(rename = "buyingPower", skip_serializing_if = "Option::is_none")]
    pub buying_power: Option<f64>,
    #[serde(
        rename = "buyingPowerNonMarginableTrade",
        skip_serializing_if = "Option::is_none"
    )]
    pub buying_power_non_marginable_trade: Option<f64>,
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
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<f64>,
    #[serde(rename = "equityPercentage", skip_serializing_if = "Option::is_none")]
    pub equity_percentage: Option<f64>,
    #[serde(rename = "longMarginValue", skip_serializing_if = "Option::is_none")]
    pub long_margin_value: Option<f64>,
    #[serde(rename = "maintenanceCall", skip_serializing_if = "Option::is_none")]
    pub maintenance_call: Option<f64>,
    #[serde(
        rename = "maintenanceRequirement",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_requirement: Option<f64>,
    #[serde(rename = "marginBalance", skip_serializing_if = "Option::is_none")]
    pub margin_balance: Option<f64>,
    #[serde(rename = "regTCall", skip_serializing_if = "Option::is_none")]
    pub reg_t_call: Option<f64>,
    #[serde(rename = "shortBalance", skip_serializing_if = "Option::is_none")]
    pub short_balance: Option<f64>,
    #[serde(rename = "shortMarginValue", skip_serializing_if = "Option::is_none")]
    pub short_margin_value: Option<f64>,
    #[serde(rename = "sma", skip_serializing_if = "Option::is_none")]
    pub sma: Option<f64>,
    #[serde(rename = "isInCall", skip_serializing_if = "Option::is_none")]
    pub is_in_call: Option<f64>,
    #[serde(rename = "stockBuyingPower", skip_serializing_if = "Option::is_none")]
    pub stock_buying_power: Option<f64>,
    #[serde(rename = "optionBuyingPower", skip_serializing_if = "Option::is_none")]
    pub option_buying_power: Option<f64>,
}

