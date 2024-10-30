use axum::{http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;

pub async fn handle_404() -> impl IntoResponse {
    (StatusCode::OK, Json(NotFoundErrorSchema::default())).into_response()
}

#[derive(Debug, Serialize)]
pub struct NotFoundErrorSchema {
    message: String,
}

impl Default for NotFoundErrorSchema {
    fn default() -> Self {
        Self {
            message: "Route not found.".to_string(),
        }
    }
}
