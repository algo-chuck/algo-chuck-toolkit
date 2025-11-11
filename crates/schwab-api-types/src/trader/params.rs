//! Parameter types for Schwab Trader API endpoints.
//!
//! These structs provide type-safe parameter construction for Trader API operations.
//! All structs implement `Serialize` for URL encoding and request body serialization.

use serde::Serialize;

use super::{OrderRequest, PreviewOrder};

/// Parameters for fetching multiple accounts.
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountsParams<'a> {
    /// Fields to include in the response (e.g., "positions")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
}

/// Parameters for fetching a single account.
#[derive(Debug, Clone, Serialize)]
pub struct GetAccountParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// Fields to include in the response (e.g., "positions")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<&'a str>,
}

/// Parameters for fetching orders by account (path parameter variant).
#[derive(Debug, Clone, Serialize)]
pub struct GetOrdersByPathParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// Start date for order search (ISO-8601 format)
    #[serde(rename = "fromEnteredTime")]
    pub from_entered_time: &'a str,
    /// End date for order search (ISO-8601 format)
    #[serde(rename = "toEnteredTime")]
    pub to_entered_time: &'a str,
    /// Maximum number of orders to return
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// Filter by order status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

/// Parameters for fetching a single order.
#[derive(Debug, Clone, Serialize)]
pub struct GetOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order ID
    pub order_id: i64,
}

/// Parameters for placing an order.
#[derive(Debug, Clone, Serialize)]
pub struct PlaceOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order details
    #[serde(skip)]
    pub order: &'a OrderRequest,
}

/// Parameters for canceling an order.
#[derive(Debug, Clone, Serialize)]
pub struct CancelOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order ID to cancel
    pub order_id: i64,
}

/// Parameters for replacing an order.
#[derive(Debug, Clone, Serialize)]
pub struct ReplaceOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order ID to replace
    pub order_id: i64,
    /// The new order details
    #[serde(skip)]
    pub order: &'a OrderRequest,
}

/// Parameters for fetching orders across all accounts (query parameter variant).
#[derive(Debug, Clone, Serialize)]
pub struct GetOrdersByQueryParams<'a> {
    /// Start date for order search (ISO-8601 format)
    #[serde(rename = "fromEnteredTime")]
    pub from_entered_time: &'a str,
    /// End date for order search (ISO-8601 format)
    #[serde(rename = "toEnteredTime")]
    pub to_entered_time: &'a str,
    /// Maximum number of orders to return
    #[serde(rename = "maxResults", skip_serializing_if = "Option::is_none")]
    pub max_results: Option<i32>,
    /// Filter by order status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<&'a str>,
}

/// Parameters for previewing an order.
#[derive(Debug, Clone, Serialize)]
pub struct PreviewOrderParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The order to preview
    #[serde(skip)]
    pub order: &'a PreviewOrder,
}

/// Parameters for fetching transactions by account (path parameter variant).
#[derive(Debug, Clone, Serialize)]
pub struct GetTransactionsByPathParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// Start date for transaction search (ISO-8601 format)
    #[serde(rename = "startDate")]
    pub start_date: &'a str,
    /// End date for transaction search (ISO-8601 format)
    #[serde(rename = "endDate")]
    pub end_date: &'a str,
    /// Transaction types to include (comma-separated)
    pub types: &'a str,
    /// Filter by symbol
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<&'a str>,
}

/// Parameters for fetching a single transaction by ID.
#[derive(Debug, Clone, Serialize)]
pub struct GetTransactionByIdParams<'a> {
    /// The encrypted account ID
    pub account_hash: &'a str,
    /// The transaction ID
    pub transaction_id: i64,
}
