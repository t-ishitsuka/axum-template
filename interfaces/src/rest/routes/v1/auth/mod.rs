pub mod user;

use axum::Router;

///
/// 認証集約エンドポイント
///
pub fn build_auth_router() -> Router {
    let router = Router::new().merge(user::build_authed_user_router());

    Router::new().nest("/auth", router)
}
