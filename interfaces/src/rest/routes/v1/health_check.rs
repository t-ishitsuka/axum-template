use axum::{routing::get, Router};
use registry::AppState;

use crate::rest::controllers::v1::health_check_controller::health_check;

///
/// アプリケーションヘルスチェック
///
pub fn build_health_check_router() -> Router<AppState> {
    let router = Router::new().route("/", get(health_check));

    Router::new().nest("/health-check", router)
}
