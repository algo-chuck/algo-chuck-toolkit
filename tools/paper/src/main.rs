use axum::{Router, middleware::map_response, response::Response};
use std::sync::Arc;

mod api;
mod db;
mod error;
mod handlers;
mod response;
mod services;

pub use self::error::{Error, Result};
pub use self::response::{Created, EmptyOK};

use db::repositories::{
    AccountRepository, OrderRepository, TransactionRepository, UserPreferenceRepository,
};
use services::{AccountService, OrderService, TransactionService, UserPreferenceService};

/// Application state containing all services
#[derive(Clone)]
pub struct AppState {
    pub account_service: Arc<AccountService>,
    pub order_service: Arc<OrderService>,
    pub transaction_service: Arc<TransactionService>,
    pub user_preference_service: Arc<UserPreferenceService>,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database connection
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite::memory:".to_string());

    println!("->> Connecting to database: {}", database_url);

    let pool = sqlx::SqlitePool::connect(&database_url)
        .await
        .map_err(|err| format!("Cannot connect to database. \nCause: {err}"))?;

    println!("->> Running database migrations...");
    sqlx::migrate!("./src/db/migrations")
        .run(&pool)
        .await
        .map_err(|err| format!("Cannot run migrations. \nCause: {err}"))?;

    println!("->> Database ready");

    // Create repositories
    let account_repo = AccountRepository::new(pool.clone());
    let order_repo = OrderRepository::new(pool.clone());
    let transaction_repo = TransactionRepository::new(pool.clone());
    let user_preference_repo = UserPreferenceRepository::new(pool.clone());

    // Create services
    let account_service = Arc::new(AccountService::new(account_repo));
    let order_service = Arc::new(OrderService::new(order_repo));
    let transaction_service = Arc::new(TransactionService::new(transaction_repo));
    let user_preference_service = Arc::new(UserPreferenceService::new(user_preference_repo));

    // Create app state
    let app_state = AppState {
        account_service,
        order_service,
        transaction_service,
        user_preference_service,
    };

    // Build router with state
    let app = Router::new()
        .nest("/trader/v1", api::router())
        .with_state(Arc::new(app_state))
        .layer(map_response(main_response_mapper));

    const HOST: &str = "127.0.0.1";
    const PORT: u16 = 9000;
    let addr = format!("{HOST}:{PORT}");
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|err| format!("Cannot start TcpListener. \nCause: {err}"))?;

    println!("->> LISTENING on {:?}\n", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .map_err(|err| format!("Cannot start axum::serve. \nCause:{err}"))?;

    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}
