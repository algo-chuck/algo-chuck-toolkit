use axum::{Json, extract::State};
use schwab_api::prelude::trader::UserPreference;
use std::sync::Arc;

use super::error_mapping::{HandlerResult, map_user_preference_error};
use crate::AppState;

/// GET /trader/v1/userPreference
/// Get user preferences
pub async fn get_user_preference(
    State(app_state): State<Arc<AppState>>,
) -> HandlerResult<UserPreference> {
    println!("->> {:<12} - get_user_preference", "HANDLER");

    app_state
        .user_preference_service
        .get_user_preference()
        .await
        .map(Json)
        .map_err(map_user_preference_error)
}
