use axum::Router;
use v1::build_v1_router;

pub mod v1;

pub fn build_rest_router() -> Router {
    let router = Router::new().merge(build_v1_router());

    Router::new().nest("/api", router)
}
