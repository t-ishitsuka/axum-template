use axum::Router;
use rest::routes::build_rest_router;

use openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

pub mod openapi;
pub mod rest;

///
/// ルーティングを定義する
///
pub fn build_router() -> Router {
    let router = Router::new().merge(build_rest_router());

    // TODO 環境変数からproduction以外とする or dev build 時は#[debug_assertions]とする?
    #[cfg(debug_assertions)]
    let router = router.merge(Redoc::with_url("/openapi", ApiDoc::openapi()));

    return router;
}
