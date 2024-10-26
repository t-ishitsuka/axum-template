use axum::{routing::get, Router};

use crate::rest::controllers::v1::health_check_controller::health_check;

pub fn build_health_check_router() -> Router {
    let router = Router::new().route("/", get(health_check));

    Router::new().nest("/health-check", router)
}
