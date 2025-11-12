use crate::trader;
use serde::{Deserialize, Serialize};

/// Order information including status, type, and execution details.
///
/// This is a response type returned by order-related API operations.
/// Contains complete order details including session, duration, type, quantity, price,
/// order legs, and child strategies for complex orders.
///
/// # API Operations
/// - `GET /accounts/{accountNumber}/orders` - Returns `Vec<Order>`
/// - `GET /orders` - Returns `Vec<Order>`  
/// - `GET /accounts/{accountNumber}/orders/{orderId}` - Returns `Order`
///
/// # Fields
/// Includes session, duration, order type, quantities, prices, stop prices,
/// order legs, activity collection, and child order strategies.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    #[serde(rename = "session", skip_serializing_if = "Option::is_none")]
    pub session: Option<trader::Session>,
    #[serde(rename = "duration", skip_serializing_if = "Option::is_none")]
    pub duration: Option<trader::Duration>,
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<trader::OrderType>,
    #[serde(rename = "cancelTime", skip_serializing_if = "Option::is_none")]
    pub cancel_time: Option<String>,
    #[serde(
        rename = "complexOrderStrategyType",
        skip_serializing_if = "Option::is_none"
    )]
    pub complex_order_strategy_type: Option<trader::ComplexOrderStrategyType>,
    #[serde(rename = "quantity", skip_serializing_if = "Option::is_none")]
    pub quantity: Option<f64>,
    #[serde(rename = "filledQuantity", skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<f64>,
    #[serde(rename = "remainingQuantity", skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<f64>,
    #[serde(
        rename = "requestedDestination",
        skip_serializing_if = "Option::is_none"
    )]
    pub requested_destination: Option<trader::RequestedDestination>,
    #[serde(
        rename = "destinationLinkName",
        skip_serializing_if = "Option::is_none"
    )]
    pub destination_link_name: Option<String>,
    #[serde(rename = "releaseTime", skip_serializing_if = "Option::is_none")]
    pub release_time: Option<String>,
    #[serde(rename = "stopPrice", skip_serializing_if = "Option::is_none")]
    pub stop_price: Option<f64>,
    #[serde(rename = "stopPriceLinkBasis", skip_serializing_if = "Option::is_none")]
    pub stop_price_link_basis: Option<trader::StopPriceLinkBasis>,
    #[serde(rename = "stopPriceLinkType", skip_serializing_if = "Option::is_none")]
    pub stop_price_link_type: Option<trader::StopPriceLinkType>,
    #[serde(rename = "stopPriceOffset", skip_serializing_if = "Option::is_none")]
    pub stop_price_offset: Option<f64>,
    #[serde(rename = "stopType", skip_serializing_if = "Option::is_none")]
    pub stop_type: Option<trader::StopType>,
    #[serde(rename = "priceLinkBasis", skip_serializing_if = "Option::is_none")]
    pub price_link_basis: Option<trader::PriceLinkBasis>,
    #[serde(rename = "priceLinkType", skip_serializing_if = "Option::is_none")]
    pub price_link_type: Option<trader::PriceLinkType>,
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<f64>,
    #[serde(rename = "taxLotMethod", skip_serializing_if = "Option::is_none")]
    pub tax_lot_method: Option<trader::TaxLotMethod>,
    #[serde(rename = "orderLegCollection", skip_serializing_if = "Option::is_none")]
    pub order_leg_collection: Option<Vec<trader::OrderLegCollection>>,
    #[serde(rename = "activationPrice", skip_serializing_if = "Option::is_none")]
    pub activation_price: Option<f64>,
    #[serde(rename = "specialInstruction", skip_serializing_if = "Option::is_none")]
    pub special_instruction: Option<trader::SpecialInstruction>,
    #[serde(rename = "orderStrategyType", skip_serializing_if = "Option::is_none")]
    pub order_strategy_type: Option<trader::OrderStrategyType>,
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    #[serde(rename = "cancelable", skip_serializing_if = "Option::is_none")]
    pub cancelable: Option<bool>,
    #[serde(rename = "editable", skip_serializing_if = "Option::is_none")]
    pub editable: Option<bool>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<trader::Status>,
    #[serde(rename = "enteredTime", skip_serializing_if = "Option::is_none")]
    pub entered_time: Option<String>,
    #[serde(rename = "closeTime", skip_serializing_if = "Option::is_none")]
    pub close_time: Option<String>,
    #[serde(rename = "tag", skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(rename = "accountNumber", skip_serializing_if = "Option::is_none")]
    pub account_number: Option<i64>,
    #[serde(
        rename = "orderActivityCollection",
        skip_serializing_if = "Option::is_none"
    )]
    pub order_activity_collection: Option<Vec<trader::OrderActivity>>,
    #[serde(
        rename = "replacingOrderCollection",
        skip_serializing_if = "Option::is_none"
    )]
    pub replacing_order_collection: Option<Vec<trader::Order>>,
    #[serde(
        rename = "childOrderStrategies",
        skip_serializing_if = "Option::is_none"
    )]
    pub child_order_strategies: Option<Vec<trader::Order>>,
    #[serde(rename = "statusDescription", skip_serializing_if = "Option::is_none")]
    pub status_description: Option<String>,
}
