use axum::Router;
use rest::routes::build_rest_router;

pub mod rest;

pub fn build_router() -> Router {
    Router::new().merge(build_rest_router())
}
