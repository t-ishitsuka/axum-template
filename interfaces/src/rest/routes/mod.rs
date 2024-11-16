use axum::Router;
use registry::AppState;
use v1::build_v1_router;

pub mod v1;

///
/// REST APIに関するルーティングを定義する
///
pub fn build_rest_router() -> Router<AppState> {
    let router = Router::new().merge(build_v1_router());

    Router::new().nest("/api", router)
}
