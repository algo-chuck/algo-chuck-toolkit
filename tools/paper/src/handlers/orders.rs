use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use schwab_api::prelude::trader::{Order, PreviewOrder};
use schwab_api::types::trader::{
    CancelOrderParams, GetOrderParams, GetOrdersByPathParams, GetOrdersByQueryParams, OrderRequest,
    PlaceOrderParams, PreviewOrderParams, ReplaceOrderParams,
};
use serde::Deserialize;
use std::sync::Arc;

use super::error_mapping::{HandlerResult, map_order_error};
use crate::{AppState, Created, EmptyOK};

/// Query parameters for get_orders_by_path_param
#[derive(Debug, Deserialize)]
pub struct GetOrdersByPathQuery {
    #[serde(rename = "fromEnteredTime")]
    pub from_entered_time: String,
    #[serde(rename = "toEnteredTime")]
    pub to_entered_time: String,
    #[serde(rename = "maxResults")]
    pub max_results: Option<i32>,
    pub status: Option<String>,
}

/// Query parameters for get_orders_by_query_param  
#[derive(Debug, Deserialize)]
pub struct GetOrdersByQueryQuery {
    #[serde(rename = "fromEnteredTime")]
    pub from_entered_time: String,
    #[serde(rename = "toEnteredTime")]
    pub to_entered_time: String,
    #[serde(rename = "maxResults")]
    pub max_results: Option<i32>,
    pub status: Option<String>,
}

/// GET /trader/v1/accounts/{accountNumber}/orders
/// Get orders for a specific account
pub async fn get_orders_by_path_param(
    State(app_state): State<Arc<AppState>>,
    Path(account_hash): Path<String>,
    Query(query): Query<GetOrdersByPathQuery>,
) -> HandlerResult<Vec<Order>> {
    println!(
        "->> {:<12} - get_orders_by_path_param (account={}, from={}, to={})",
        "HANDLER", account_hash, query.from_entered_time, query.to_entered_time
    );

    let params = GetOrdersByPathParams {
        account_hash: &account_hash,
        from_entered_time: &query.from_entered_time,
        to_entered_time: &query.to_entered_time,
        max_results: query.max_results,
        status: query.status.as_deref(),
    };

    app_state
        .order_service
        .get_orders_by_path(params)
        .await
        .map(Json)
        .map_err(map_order_error)
}

/// POST /trader/v1/accounts/{accountNumber}/orders
/// Place a new order
pub async fn place_order(
    State(app_state): State<Arc<AppState>>,
    Path(account_hash): Path<String>,
    Json(order_request): Json<OrderRequest>,
) -> Result<Created, (StatusCode, Json<schwab_api::types::trader::ServiceError>)> {
    println!(
        "->> {:<12} - place_order (account={})",
        "HANDLER", account_hash
    );

    let params = PlaceOrderParams {
        account_hash: &account_hash,
        order: &order_request,
    };

    app_state
        .order_service
        .place_order(params, order_request.clone())
        .await
        .map(|_order_id| Created {})
        .map_err(map_order_error)
}

/// GET /trader/v1/accounts/{accountNumber}/orders/{orderId}
/// Get a specific order by ID
pub async fn get_order(
    State(app_state): State<Arc<AppState>>,
    Path((account_hash, order_id)): Path<(String, i64)>,
) -> HandlerResult<Order> {
    println!(
        "->> {:<12} - get_order (account={}, order_id={})",
        "HANDLER", account_hash, order_id
    );

    let params = GetOrderParams {
        account_hash: &account_hash,
        order_id,
    };

    app_state
        .order_service
        .get_order(params)
        .await
        .map(Json)
        .map_err(map_order_error)
}

/// DELETE /trader/v1/accounts/{accountNumber}/orders/{orderId}
/// Cancel an order
pub async fn cancel_order(
    State(app_state): State<Arc<AppState>>,
    Path((account_hash, order_id)): Path<(String, i64)>,
) -> Result<EmptyOK, (StatusCode, Json<schwab_api::types::trader::ServiceError>)> {
    println!(
        "->> {:<12} - cancel_order (account={}, order_id={})",
        "HANDLER", account_hash, order_id
    );

    let params = CancelOrderParams {
        account_hash: &account_hash,
        order_id,
    };

    app_state
        .order_service
        .cancel_order(params)
        .await
        .map(|_| EmptyOK {})
        .map_err(map_order_error)
}

/// PUT /trader/v1/accounts/{accountNumber}/orders/{orderId}
/// Replace an existing order
pub async fn replace_order(
    State(app_state): State<Arc<AppState>>,
    Path((account_hash, order_id)): Path<(String, i64)>,
    Json(order_request): Json<OrderRequest>,
) -> Result<Created, (StatusCode, Json<schwab_api::types::trader::ServiceError>)> {
    println!(
        "->> {:<12} - replace_order (account={}, order_id={})",
        "HANDLER", account_hash, order_id
    );

    let params = ReplaceOrderParams {
        account_hash: &account_hash,
        order_id,
        order: &order_request,
    };

    app_state
        .order_service
        .replace_order(params, order_request.clone())
        .await
        .map(|_new_order_id| Created {})
        .map_err(map_order_error)
}

/// GET /trader/v1/orders
/// Get orders across all accounts
pub async fn get_orders_by_query_param(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<GetOrdersByQueryQuery>,
) -> HandlerResult<Vec<Order>> {
    println!(
        "->> {:<12} - get_orders_by_query_param (from={}, to={})",
        "HANDLER", query.from_entered_time, query.to_entered_time
    );

    let params = GetOrdersByQueryParams {
        from_entered_time: &query.from_entered_time,
        to_entered_time: &query.to_entered_time,
        max_results: query.max_results,
        status: query.status.as_deref(),
    };

    app_state
        .order_service
        .get_orders_by_query(params)
        .await
        .map(Json)
        .map_err(map_order_error)
}

/// POST /trader/v1/accounts/{accountNumber}/previewOrder
/// Preview an order (not implemented yet - returns mock data)
pub async fn preview_order(
    Path(account_hash): Path<String>,
    Json(_order_request): Json<OrderRequest>,
) -> HandlerResult<PreviewOrder> {
    println!(
        "->> {:<12} - preview_order (account={}) [NOT IMPLEMENTED]",
        "HANDLER", account_hash
    );

    // Return mock preview order for now
    Ok(Json(PreviewOrder::new()))
}
