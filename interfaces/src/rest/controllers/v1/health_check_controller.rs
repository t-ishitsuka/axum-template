use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::rest::responses::global_ok_response::GlobalOkResponse;

///
/// アプリケーションヘルスを返却する
///
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(GlobalOkResponse::default())).into_response()
}
