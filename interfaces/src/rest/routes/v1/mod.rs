pub mod auth;
pub mod health_check;

use auth::build_auth_router;
use axum::Router;
use health_check::build_health_check_router;
use registry::AppState;

///
/// API V1に関するルーティング
///
pub fn build_v1_router() -> Router<AppState> {
    let router = Router::new()
        .merge(build_auth_router())
        .merge(build_health_check_router());

    Router::new().nest("/v1", router)
}
