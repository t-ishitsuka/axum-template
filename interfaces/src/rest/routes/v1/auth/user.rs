use axum::{routing::get, Router};

use crate::rest::controllers::v1::auth::user_controller::user;

///
/// ログインユーザー集約エンドポイント
///
pub fn build_authed_user_router() -> Router {
    let router = Router::new().route("/", get(user));

    Router::new().nest("/user", router)
}
