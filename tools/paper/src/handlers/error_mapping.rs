use axum::{http::StatusCode, response::Json};
use schwab_api::prelude::trader::service_error::ServiceErrorItem;
use schwab_api::types::trader::ServiceError;

use crate::services::{
    AccountServiceError, OrderServiceError, TransactionServiceError, UserPreferenceServiceError,
};

/// Type alias for handler responses that can return service errors
pub type HandlerResult<T> = Result<Json<T>, (StatusCode, Json<ServiceError>)>;

/// Convert AccountServiceError to ServiceError response
pub fn map_account_error(err: AccountServiceError) -> (StatusCode, Json<ServiceError>) {
    match err {
        AccountServiceError::NotFound(id) => (
            StatusCode::NOT_FOUND,
            Json(ServiceError {
                message: Some(format!("Account not found: {}", id)),
                errors: Some(vec![ServiceErrorItem {
                    id: Some(id),
                    status: Some(404),
                    title: Some("Not Found".to_string()),
                    detail: Some("The requested account does not exist".to_string()),
                }]),
            }),
        ),
        AccountServiceError::InvalidInput(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ServiceError {
                message: Some("Invalid request".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(400),
                    title: Some("Bad Request".to_string()),
                    detail: Some(msg),
                }]),
            }),
        ),
        AccountServiceError::Repository(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceError {
                message: Some("Internal server error".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(500),
                    title: Some("Internal Server Error".to_string()),
                    detail: Some(format!("Repository error: {}", err)),
                }]),
            }),
        ),
    }
}

/// Convert OrderServiceError to ServiceError response
pub fn map_order_error(err: OrderServiceError) -> (StatusCode, Json<ServiceError>) {
    match err {
        OrderServiceError::NotFound(id) => (
            StatusCode::NOT_FOUND,
            Json(ServiceError {
                message: Some(format!("Order not found: {}", id)),
                errors: Some(vec![ServiceErrorItem {
                    id: Some(id.to_string()),
                    status: Some(404),
                    title: Some("Not Found".to_string()),
                    detail: Some("The requested order does not exist".to_string()),
                }]),
            }),
        ),
        OrderServiceError::InvalidInput(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ServiceError {
                message: Some("Invalid request".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(400),
                    title: Some("Bad Request".to_string()),
                    detail: Some(msg),
                }]),
            }),
        ),
        OrderServiceError::Repository(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceError {
                message: Some("Internal server error".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(500),
                    title: Some("Internal Server Error".to_string()),
                    detail: Some(format!("Repository error: {}", err)),
                }]),
            }),
        ),
    }
}

/// Convert TransactionServiceError to ServiceError response
pub fn map_transaction_error(err: TransactionServiceError) -> (StatusCode, Json<ServiceError>) {
    match err {
        TransactionServiceError::NotFound(id) => (
            StatusCode::NOT_FOUND,
            Json(ServiceError {
                message: Some(format!("Transaction not found: {}", id)),
                errors: Some(vec![ServiceErrorItem {
                    id: Some(id.to_string()),
                    status: Some(404),
                    title: Some("Not Found".to_string()),
                    detail: Some("The requested transaction does not exist".to_string()),
                }]),
            }),
        ),
        TransactionServiceError::InvalidInput(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ServiceError {
                message: Some("Invalid request".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(400),
                    title: Some("Bad Request".to_string()),
                    detail: Some(msg),
                }]),
            }),
        ),
        TransactionServiceError::Repository(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceError {
                message: Some("Internal server error".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(500),
                    title: Some("Internal Server Error".to_string()),
                    detail: Some(format!("Repository error: {}", err)),
                }]),
            }),
        ),
    }
}

/// Convert UserPreferenceServiceError to ServiceError response
pub fn map_user_preference_error(
    err: UserPreferenceServiceError,
) -> (StatusCode, Json<ServiceError>) {
    match err {
        UserPreferenceServiceError::NotFound => (
            StatusCode::NOT_FOUND,
            Json(ServiceError {
                message: Some("User preference not found".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(404),
                    title: Some("Not Found".to_string()),
                    detail: Some("User preference does not exist".to_string()),
                }]),
            }),
        ),
        UserPreferenceServiceError::InvalidInput(msg) => (
            StatusCode::BAD_REQUEST,
            Json(ServiceError {
                message: Some("Invalid request".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(400),
                    title: Some("Bad Request".to_string()),
                    detail: Some(msg),
                }]),
            }),
        ),
        UserPreferenceServiceError::Repository(err) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ServiceError {
                message: Some("Internal server error".to_string()),
                errors: Some(vec![ServiceErrorItem {
                    id: None,
                    status: Some(500),
                    title: Some("Internal Server Error".to_string()),
                    detail: Some(format!("Repository error: {}", err)),
                }]),
            }),
        ),
    }
}
