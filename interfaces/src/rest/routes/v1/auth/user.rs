use axum::{routing::get, Router};
use registry::AppState;

use crate::rest::controllers::v1::auth::user_controller::user;

///
/// ログインユーザー集約エンドポイント
///
pub fn build_authed_user_router() -> Router<AppState> {
    let router = Router::new().route("/", get(user));

    Router::new().nest("/user", router)
}
