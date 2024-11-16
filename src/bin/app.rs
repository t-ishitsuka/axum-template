use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Error, Result};
use axum::Router;

use registry::AppState;
use shares::config::{config, load_env};

use interfaces::openapi::ApiDoc;
use interfaces::rest::routes::build_rest_router;
use shares::error::error_404;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

///
/// Main, Entrypoint
///
#[tokio::main]
async fn main() -> Result<()> {
    load_env();
    bootstrap().await
}

///
/// ルーティングを定義する
///
fn build_router() -> Router {
    let state = AppState::new();

    let mut router = Router::new().merge(build_rest_router());

    let is_production: bool = config("app.is_production").into();
    let is_testing: bool = config("app.is_testing").into();

    if !is_production && !is_testing {
        router = router.merge(Redoc::with_url("/openapi", ApiDoc::openapi()));
    }

    let router = router.with_state(state).fallback(error_404::handle_404);

    return router;
}

///
/// アプリケーションサーバーの起動
///
async fn bootstrap() -> Result<()> {
    let app = build_router();

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), config("app.port").into());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening on the {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
