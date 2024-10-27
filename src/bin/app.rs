use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Error, Result};

use interfaces::build_router;
use shares::config::{config, load_env};

///
/// Main, Entrypoint
///
#[tokio::main]
async fn main() -> Result<()> {
    load_env();
    bootstrap().await
}

///
/// アプリケーションサーバーの起動
///
async fn bootstrap() -> Result<()> {
    let app = build_router();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), config("app.port").into());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening on the {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
