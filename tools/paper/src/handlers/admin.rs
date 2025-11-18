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
use rand::Rng;
use schwab_api::prelude::trader::{
    CashAccount, CashBalance, CashInitialBalance, SecuritiesAccount,
    service_error::{ServiceError, ServiceErrorItem},
};
use sha2::{Digest, Sha256};
use std::sync::Arc;

use crate::{AppState, Created};

/// Create a new paper trading account
///
/// POST /admin/v1/accounts
#[axum::debug_handler]
pub async fn create_account(
    State(app_state): State<Arc<AppState>>,
) -> Result<Created, (StatusCode, Json<ServiceError>)> {
    println!("->> {:<12} - create_account", "HANDLER");

    // Fetch existing accounts first (before creating RNG)
    let existing_accounts = match app_state.account_service.get_account_numbers().await {
        Ok(accounts) => accounts,
        Err(e) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ServiceError {
                    message: Some("There was a problem creating the account".to_string()),
                    errors: Some(vec![ServiceErrorItem {
                        id: None,
                        status: Some(500),
                        title: Some("Internal Server Error".to_string()),
                        detail: Some(e.to_string()),
                    }]),
                }),
            ));
        }
    };

    // Generate unique random account number (after await, so no Send issue)
    let account_number = {
        let mut rng = rand::thread_rng();
        loop {
            let num = rng.gen_range(10000000..100000000);
            let num_str = num.to_string();

            // Check if this number is already in use
            if !existing_accounts
                .iter()
                .any(|acc| acc.account_number.as_ref() == Some(&num_str))
            {
                break num_str;
            }
            // If collision (very rare with 90M possible numbers), try again
        }
    }; // rng is dropped here

    // Generate SHA256 hash value (64 uppercase hex characters)
    let hash_value = generate_hash(&account_number);

    // Fixed initial balance of $200,000
    let initial_balance = 200_000.0;

    // Create the CASH account data structure
    let account_data = create_cash_account(&account_number, initial_balance);

    // Call service to create the account
    if let Err(e) = app_state
        .account_service
        .create_account(&account_number, &hash_value, &account_data)
        .await
    {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceError {
                message: Some("Failed to create account".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(500),
                    title: Some("Internal Server Error".to_string()),
                    detail: Some(e.to_string()),
                }]),
            }),
        ));
    }

    // Return success response
    Ok(Created {})
}

/// Delete an account by account number
///
/// DELETE /admin/v1/accounts/{accountNumber}
pub async fn delete_account(
    State(_app_state): State<Arc<AppState>>,
    Path(_account_number): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ServiceError>)> {
    println!("->> {:<12} - delete_account", "HANDLER");

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
/// POST /admin/v1/accounts/{accountNumber}/reset
pub async fn reset_account(
    State(_app_state): State<Arc<AppState>>,
    Path(_account_number): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ServiceError>)> {
    println!("->> {:<12} - reset_account", "HANDLER");

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

/// Generate SHA256 hash of account number (64 uppercase hex characters)
fn generate_hash(account_number: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(account_number.as_bytes());
    let result = hasher.finalize();
    format!("{:X}", result)
}

fn create_cash_account(account_number: &str, initial_balance: f64) -> SecuritiesAccount {
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
        r#type: None,
        account_number: Some(account_number.to_string()),
        round_trips: Some(0),
        is_day_trader: Some(false),
        is_closing_only_restricted: Some(false),
        pfcb_flag: Some(false),
        positions: None,
    }))
}
