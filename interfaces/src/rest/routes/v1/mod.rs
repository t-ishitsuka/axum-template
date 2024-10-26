pub mod health_check;

use axum::Router;
use health_check::build_health_check_router;

pub fn build_v1_router() -> Router {
    let router = Router::new().merge(build_health_check_router());

    Router::new().nest("/v1", router)
}
