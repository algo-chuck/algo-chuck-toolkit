use axum::Json;
use schwab_api::prelude::trader::UserPreference;

use crate::Result;

pub async fn get_user_preference() -> Result<Json<UserPreference>> {
    println!("->> {:<12} - get_user_preference", "HANDLER");
    Ok(Json(UserPreference::new()))
}
