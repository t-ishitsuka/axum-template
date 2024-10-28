use axum::{http::StatusCode, response::IntoResponse, Json};

use crate::rest::responses::global_ok_response::GlobalOkResponse;

///
/// アプリケーションヘルスを返却する
///
#[cfg_attr(
    debug_assertions,
    utoipa::path(
        get,
        path = "/api/v1/health-check",
        responses(
            (status = StatusCode::OK, description = "アプリケーションは健全", body = GlobalOkResponse)
        )
    )
)]
pub async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, Json(GlobalOkResponse::default())).into_response()
}
