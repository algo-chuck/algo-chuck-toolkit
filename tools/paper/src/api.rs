use axum::{
    Router,
    routing::{delete, get, post},
};
use std::sync::Arc;

use crate::AppState;
use crate::handlers::*;

pub fn trader_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/accounts/accountNumbers", get(get_account_numbers))
        .route("/accounts", get(get_accounts))
        .route("/accounts/{accountNumber}", get(get_account))
        .route(
            "/accounts/{accountNumber}/orders",
            get(get_orders_by_path_param).post(place_order),
        )
        .route(
            "/accounts/{accountNumber}/orders/{orderId}",
            get(get_order).delete(cancel_order).put(replace_order),
        )
        .route("/orders", get(get_orders_by_query_param))
        .route(
            "/accounts/{accountNumber}/previewOrder",
            post(preview_order),
        )
        .route(
            "/accounts/{accountNumber}/transactions",
            get(get_transactions_by_path_param),
        )
        .route(
            "/accounts/{accountNumber}/transactions/{transactionId}",
            get(get_transactions_by_id),
        )
        .route("/userPreference", get(get_user_preference))
}

pub fn admin_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/accounts", post(create_account))
        .route("/accounts/{hashValue}", delete(delete_account))
        .route("/accounts/{hashValue}/reset", post(reset_account))
}
