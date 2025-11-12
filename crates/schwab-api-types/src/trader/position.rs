use crate::trader;
use serde::{Deserialize, Serialize};

/// Represents account and trading information.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    #[serde(rename = "shortQuantity", skip_serializing_if = "Option::is_none")]
    pub short_quantity: Option<f64>,
    #[serde(rename = "averagePrice", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<f64>,
    #[serde(
        rename = "currentDayProfitLoss",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_day_profit_loss: Option<f64>,
    #[serde(
        rename = "currentDayProfitLossPercentage",
        skip_serializing_if = "Option::is_none"
    )]
    pub current_day_profit_loss_percentage: Option<f64>,
    #[serde(rename = "longQuantity", skip_serializing_if = "Option::is_none")]
    pub long_quantity: Option<f64>,
    #[serde(
        rename = "settledLongQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub settled_long_quantity: Option<f64>,
    #[serde(
        rename = "settledShortQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub settled_short_quantity: Option<f64>,
    #[serde(rename = "agedQuantity", skip_serializing_if = "Option::is_none")]
    pub aged_quantity: Option<f64>,
    #[serde(rename = "instrument", skip_serializing_if = "Option::is_none")]
    pub instrument: Option<Box<trader::AccountsInstrument>>,
    #[serde(rename = "marketValue", skip_serializing_if = "Option::is_none")]
    pub market_value: Option<f64>,
    #[serde(
        rename = "maintenanceRequirement",
        skip_serializing_if = "Option::is_none"
    )]
    pub maintenance_requirement: Option<f64>,
    #[serde(rename = "averageLongPrice", skip_serializing_if = "Option::is_none")]
    pub average_long_price: Option<f64>,
    #[serde(rename = "averageShortPrice", skip_serializing_if = "Option::is_none")]
    pub average_short_price: Option<f64>,
    #[serde(
        rename = "taxLotAverageLongPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_lot_average_long_price: Option<f64>,
    #[serde(
        rename = "taxLotAverageShortPrice",
        skip_serializing_if = "Option::is_none"
    )]
    pub tax_lot_average_short_price: Option<f64>,
    #[serde(rename = "longOpenProfitLoss", skip_serializing_if = "Option::is_none")]
    pub long_open_profit_loss: Option<f64>,
    #[serde(
        rename = "shortOpenProfitLoss",
        skip_serializing_if = "Option::is_none"
    )]
    pub short_open_profit_loss: Option<f64>,
    #[serde(
        rename = "previousSessionLongQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_session_long_quantity: Option<f64>,
    #[serde(
        rename = "previousSessionShortQuantity",
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_session_short_quantity: Option<f64>,
    #[serde(rename = "currentDayCost", skip_serializing_if = "Option::is_none")]
    pub current_day_cost: Option<f64>,
}

impl Position {
    pub fn new() -> Position {
        Position {
            short_quantity: None,
            average_price: None,
            current_day_profit_loss: None,
            current_day_profit_loss_percentage: None,
            long_quantity: None,
            settled_long_quantity: None,
            settled_short_quantity: None,
            aged_quantity: None,
            instrument: None,
            market_value: None,
            maintenance_requirement: None,
            average_long_price: None,
            average_short_price: None,
            tax_lot_average_long_price: None,
            tax_lot_average_short_price: None,
            long_open_profit_loss: None,
            short_open_profit_loss: None,
            previous_session_long_quantity: None,
            previous_session_short_quantity: None,
            current_day_cost: None,
        }
    }
}
