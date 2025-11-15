use axum::Json;
use schwab_api::prelude::trader::Transaction;

use crate::Result;

pub async fn get_transactions_by_path_param() -> Result<Json<Vec<Transaction>>> {
    println!("->> {:<12} - get_transactions_by_path_param", "HANDLER");
    Ok(Json(vec![]))
}

pub async fn get_transactions_by_id() -> Result<Json<Vec<Transaction>>> {
    println!("->> {:<12} - get_transactions_by_id", "HANDLER");
    Ok(Json(vec![]))
}
