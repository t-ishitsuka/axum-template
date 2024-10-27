use axum::Router;
use openapi::ApiDoc;
use rest::routes::build_rest_router;
use shares::config::{config, Config};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

pub mod openapi;
pub mod rest;

///
/// ルーティングを定義する
///
pub fn build_router() -> Router {
    let mut router = Router::new().merge(build_rest_router());

    let is_production: bool = config("app.is_production").into();
    let is_testing: bool = config("app.is_testing").into();

    if !is_production && !is_testing {
        router = router.merge(Redoc::with_url("/openapi", ApiDoc::openapi()));
    }

    return router;
}
