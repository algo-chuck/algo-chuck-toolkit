use axum::Json;
use schwab_api::prelude::trader::{Account, AccountNumberHash};

use crate::Result;

pub async fn get_account_numbers() -> Result<Json<Vec<AccountNumberHash>>> {
    println!("->> {:<12} - get_account_numbers", "HANDLER");
    Ok(Json(vec![AccountNumberHash::new()]))
}

pub async fn get_accounts() -> Result<Json<Vec<Account>>> {
    println!("->> {:<12} - get_accounts", "HANDLER");
    Ok(Json(vec![Account::new()]))
}

pub async fn get_account() -> Result<Json<Account>> {
    println!("->> {:<12} - get_account", "HANDLER");
    Ok(Json(Account::new()))
}
