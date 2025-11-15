use axum::{
    http::StatusCode,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;

#[derive(Serialize)]
pub struct Created {}

impl IntoResponse for Created {
    fn into_response(self) -> Response {
        (StatusCode::CREATED, Json(Created {})).into_response()
    }
}

#[derive(Serialize)]
pub struct EmptyOK {}

impl IntoResponse for EmptyOK {
    fn into_response(self) -> Response {
        (StatusCode::OK, Json(EmptyOK {})).into_response()
    }
}
