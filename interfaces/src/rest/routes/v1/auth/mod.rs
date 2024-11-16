pub mod user;

use axum::Router;
use registry::AppState;

///
/// 認証集約エンドポイント
///
pub fn build_auth_router() -> Router<AppState> {
    let router = Router::new().merge(user::build_authed_user_router());

    Router::new().nest("/auth", router)
}
