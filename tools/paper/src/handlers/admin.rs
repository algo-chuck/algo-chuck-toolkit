//! Admin handlers for account management operations
//!
//! These endpoints are for administrative purposes only and should not be
//! exposed in production. They allow creating, deleting, and resetting accounts
//! for development and testing purposes.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use schwab_api::prelude::trader::service_error::{ServiceError, ServiceErrorItem};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::AppState;

/// Request payload for creating a new account
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountRequest {
    /// Account type: "CASH" or "MARGIN"
    pub account_type: AccountType,

    /// Initial cash balance (default: $100,000)
    #[serde(default = "default_balance")]
    pub initial_balance: f64,

    /// Optional initial positions
    #[serde(default)]
    pub initial_positions: Vec<InitialPosition>,
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    Cash,
    Margin,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitialPosition {
    pub symbol: String,
    pub quantity: f64,
    pub average_price: f64,
}

fn default_balance() -> f64 {
    100_000.0
}

/// Response after creating an account
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAccountResponse {
    pub account_number: String,
    pub hash_value: String,
    pub account_type: String,
    pub initial_balance: f64,
}

/// Create a new paper trading account
///
/// POST /admin/accounts
pub async fn create_account(
    State(app_state): State<Arc<AppState>>,
    Json(request): Json<CreateAccountRequest>,
) -> Result<(StatusCode, Json<CreateAccountResponse>), (StatusCode, Json<ServiceError>)> {
    // Validate initial balance
    if request.initial_balance <= 0.0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ServiceError {
                message: Some("Invalid initial balance".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(400),
                    title: Some("Bad Request".to_string()),
                    detail: Some("Initial balance must be greater than 0".to_string()),
                }]),
            }),
        ));
    }

    // Generate account number (8-digit number starting from 10000000)
    let account_numbers = app_state
        .account_service
        .get_account_numbers()
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ServiceError {
                    message: Some("Failed to generate account number".to_string()),
                    errors: Some(vec![ServiceErrorItem {
                        id: None,
                        status: Some(500),
                        title: Some("Internal Server Error".to_string()),
                        detail: Some(e.to_string()),
                    }]),
                }),
            )
        })?;

    let next_number = if account_numbers.is_empty() {
        10000000
    } else {
        // Find the highest account number and add 1
        account_numbers
            .iter()
            .filter_map(|anh| anh.account_number.as_ref())
            .filter_map(|n| n.parse::<i64>().ok())
            .max()
            .unwrap_or(10000000)
            + 1
    };

    let account_number = next_number.to_string();

    // Generate hash value (simple hash for testing - in production would be more secure)
    let hash_value = format!("HASH{:08X}", next_number);

    // Create the account data structure
    let account_data = match request.account_type {
        AccountType::Cash => create_cash_account(
            &account_number,
            request.initial_balance,
            &request.initial_positions,
        ),
        AccountType::Margin => create_margin_account(
            &account_number,
            request.initial_balance,
            &request.initial_positions,
        ),
    };

    // TODO: Call account service to create the account
    // For now, return error saying not implemented

    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ServiceError {
            message: Some("Account creation not yet implemented".to_string()),
            errors: Some(vec![ServiceErrorItem {
                id: None,
                status: Some(501),
                title: Some("Not Implemented".to_string()),
                detail: Some(
                    "Admin account creation will be implemented in the service layer".to_string(),
                ),
            }]),
        }),
    ))
}

/// Delete an account by account number
///
/// DELETE /admin/accounts/{accountNumber}
pub async fn delete_account(
    State(_app_state): State<Arc<AppState>>,
    Path(_account_number): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ServiceError>)> {
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ServiceError {
            message: Some("Account deletion not yet implemented".to_string()),
            errors: Some(vec![ServiceErrorItem {
                id: None,
                status: Some(501),
                title: Some("Not Implemented".to_string()),
                detail: Some(
                    "Admin account deletion will be implemented in the service layer".to_string(),
                ),
            }]),
        }),
    ))
}

/// Reset an account to its initial state
///
/// POST /admin/accounts/{accountNumber}/reset
pub async fn reset_account(
    State(_app_state): State<Arc<AppState>>,
    Path(_account_number): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ServiceError>)> {
    Err((
        StatusCode::NOT_IMPLEMENTED,
        Json(ServiceError {
            message: Some("Account reset not yet implemented".to_string()),
            errors: Some(vec![ServiceErrorItem {
                id: None,
                status: Some(501),
                title: Some("Not Implemented".to_string()),
                detail: Some(
                    "Admin account reset will be implemented in the service layer".to_string(),
                ),
            }]),
        }),
    ))
}

// Helper functions to create account structures

use schwab_api::types::trader::account_equity::AssetType;
use schwab_api::types::trader::{
    AccountEquity, AccountsInstrument, CashAccount, CashBalance, CashInitialBalance, MarginAccount,
    MarginBalance, MarginInitialBalance, Position, SecuritiesAccount,
};

fn create_cash_account(
    account_number: &str,
    initial_balance: f64,
    initial_positions: &[InitialPosition],
) -> SecuritiesAccount {
    let positions = create_positions(initial_positions);

    let initial_balances = Box::new(CashInitialBalance {
        cash_available_for_trading: Some(initial_balance),
        cash_balance: Some(initial_balance),
        ..Default::default()
    });

    let current_balances = Box::new(CashBalance {
        cash_available_for_trading: Some(initial_balance),
        total_cash: Some(initial_balance),
        ..Default::default()
    });

    SecuritiesAccount::Cash(Box::new(CashAccount {
        initial_balances: Some(initial_balances),
        current_balances: Some(current_balances.clone()),
        projected_balances: Some(current_balances),
        r#type: Some(schwab_api::types::trader::cash_account::Type::Cash),
        account_number: Some(account_number.to_string()),
        round_trips: Some(0),
        is_day_trader: Some(false),
        is_closing_only_restricted: Some(false),
        pfcb_flag: Some(false),
        positions: if positions.is_empty() {
            None
        } else {
            Some(positions)
        },
    }))
}

fn create_margin_account(
    account_number: &str,
    initial_balance: f64,
    initial_positions: &[InitialPosition],
) -> SecuritiesAccount {
    let positions = create_positions(initial_positions);

    // Margin account has 2x buying power
    let buying_power = initial_balance * 2.0;

    let initial_balances = Box::new(MarginInitialBalance {
        cash_balance: Some(initial_balance),
        buying_power: Some(buying_power),
        cash_available_for_trading: Some(initial_balance),
        ..Default::default()
    });

    let current_balances = Box::new(MarginBalance {
        available_funds: Some(initial_balance),
        buying_power: Some(buying_power),
        ..Default::default()
    });

    SecuritiesAccount::Margin(Box::new(MarginAccount {
        initial_balances: Some(initial_balances),
        current_balances: Some(current_balances.clone()),
        projected_balances: Some(current_balances),
        r#type: Some(schwab_api::types::trader::margin_account::Type::Margin),
        account_number: Some(account_number.to_string()),
        round_trips: Some(0),
        is_day_trader: Some(false),
        is_closing_only_restricted: Some(false),
        pfcb_flag: Some(false),
        positions: if positions.is_empty() {
            None
        } else {
            Some(positions)
        },
    }))
}

fn create_positions(initial_positions: &[InitialPosition]) -> Vec<Position> {
    initial_positions
        .iter()
        .map(|p| {
            let market_value = p.quantity * p.average_price;
            let instrument = AccountsInstrument::Equity(Box::new(AccountEquity {
                asset_type: AssetType::Equity,
                symbol: Some(p.symbol.clone()),
                description: Some(p.symbol.clone()),
                ..Default::default()
            }));

            Position {
                short_quantity: Some(0.0),
                average_price: Some(p.average_price),
                current_day_profit_loss: Some(0.0),
                current_day_profit_loss_percentage: Some(0.0),
                long_quantity: Some(p.quantity),
                settled_long_quantity: Some(p.quantity),
                settled_short_quantity: Some(0.0),
                aged_quantity: Some(0.0),
                instrument: Some(Box::new(instrument)),
                market_value: Some(market_value),
                maintenance_requirement: Some(0.0),
                average_long_price: Some(p.average_price),
                average_short_price: Some(0.0),
                tax_lot_average_long_price: Some(p.average_price),
                tax_lot_average_short_price: Some(0.0),
                long_open_profit_loss: Some(0.0),
                short_open_profit_loss: Some(0.0),
                previous_session_long_quantity: Some(p.quantity),
                previous_session_short_quantity: Some(0.0),
                current_day_cost: Some(market_value),
            }
        })
        .collect()
}
