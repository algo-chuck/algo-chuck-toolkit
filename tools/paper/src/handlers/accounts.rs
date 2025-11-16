use axum::{
    Json,
    extract::{Path, Query, State},
};
use schwab_api::prelude::trader::AccountNumberHash;
use schwab_api::types::trader::{GetAccountParams, GetAccountsParams, SecuritiesAccount};
use serde::Deserialize;
use std::sync::Arc;

use super::error_mapping::{HandlerResult, map_account_error};
use crate::AppState;

/// Query parameters for get_accounts (owned version for axum Query extractor)
#[derive(Debug, Deserialize)]
pub struct GetAccountsQuery {
    pub fields: Option<String>,
}

/// Query parameters for get_account (owned version for axum Query extractor)
#[derive(Debug, Deserialize)]
pub struct GetAccountQuery {
    pub fields: Option<String>,
}

/// GET /trader/v1/accounts/accountNumbers
/// Returns list of account numbers and encrypted hashes
pub async fn get_account_numbers(
    State(app_state): State<Arc<AppState>>,
) -> HandlerResult<Vec<AccountNumberHash>> {
    println!("->> {:<12} - get_account_numbers", "HANDLER");

    app_state
        .account_service
        .get_account_numbers()
        .await
        .map(Json)
        .map_err(map_account_error)
}

/// GET /trader/v1/accounts?fields=positions
/// Returns list of all accounts with optional field filtering
pub async fn get_accounts(
    State(app_state): State<Arc<AppState>>,
    Query(query): Query<GetAccountsQuery>,
) -> HandlerResult<Vec<SecuritiesAccount>> {
    println!(
        "->> {:<12} - get_accounts (fields={:?})",
        "HANDLER", query.fields
    );

    // Convert owned query params to borrowed params
    let params = GetAccountsParams {
        fields: query.fields.as_deref(),
    };

    app_state
        .account_service
        .get_accounts(params)
        .await
        .map(Json)
        .map_err(map_account_error)
}

/// GET /trader/v1/accounts/{accountNumber}?fields=positions
/// Returns a specific account by hash/number with optional field filtering
pub async fn get_account(
    State(app_state): State<Arc<AppState>>,
    Path(account_hash): Path<String>,
    Query(query): Query<GetAccountQuery>,
) -> HandlerResult<SecuritiesAccount> {
    println!(
        "->> {:<12} - get_account (hash={}, fields={:?})",
        "HANDLER", account_hash, query.fields
    );

    // Convert owned query params to borrowed params
    let params = GetAccountParams {
        account_hash: &account_hash,
        fields: query.fields.as_deref(),
    };

    app_state
        .account_service
        .get_account(params)
        .await
        .map(Json)
        .map_err(map_account_error)
}
