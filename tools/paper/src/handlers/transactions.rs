use axum::{
    Json,
    extract::{Path, Query, State},
};
use schwab_api::prelude::trader::Transaction;
use schwab_api::types::trader::{GetTransactionByIdParams, GetTransactionsByPathParams};
use serde::Deserialize;
use std::sync::Arc;

use super::error_mapping::{HandlerResult, map_transaction_error};
use crate::AppState;

/// Query parameters for get_transactions_by_path_param
#[derive(Debug, Deserialize)]
pub struct GetTransactionsQuery {
    #[serde(rename = "startDate")]
    pub start_date: String,
    #[serde(rename = "endDate")]
    pub end_date: String,
    pub types: String, // Required field
    pub symbol: Option<String>,
}

/// GET /trader/v1/accounts/{accountNumber}/transactions
/// Get transactions for a specific account
pub async fn get_transactions_by_path_param(
    State(app_state): State<Arc<AppState>>,
    Path(account_hash): Path<String>,
    Query(query): Query<GetTransactionsQuery>,
) -> HandlerResult<Vec<Transaction>> {
    println!(
        "->> {:<12} - get_transactions_by_path_param (account={}, start={}, end={})",
        "HANDLER", account_hash, query.start_date, query.end_date
    );

    let params = GetTransactionsByPathParams {
        account_hash: &account_hash,
        start_date: &query.start_date,
        end_date: &query.end_date,
        types: &query.types, // Now passing &str
        symbol: query.symbol.as_deref(),
    };

    app_state
        .transaction_service
        .get_transactions(params)
        .await
        .map(Json)
        .map_err(map_transaction_error)
}

/// GET /trader/v1/accounts/{accountNumber}/transactions/{transactionId}
/// Get a specific transaction by ID
pub async fn get_transactions_by_id(
    State(app_state): State<Arc<AppState>>,
    Path((account_hash, transaction_id)): Path<(String, i64)>,
) -> HandlerResult<Transaction> {
    println!(
        "->> {:<12} - get_transactions_by_id (account={}, transaction_id={})",
        "HANDLER", account_hash, transaction_id
    );

    let params = GetTransactionByIdParams {
        account_hash: &account_hash,
        transaction_id,
    };

    app_state
        .transaction_service
        .get_transaction(params)
        .await
        .map(Json)
        .map_err(map_transaction_error)
}
