use axum::{http::StatusCode, routing::get, Router};

pub fn build_health_check_router() -> Router {
    let router = Router::new().route("/", get(health_check));

    Router::new().nest("/health-check", router)
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
