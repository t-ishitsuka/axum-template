use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Error, Result};

use interfaces::build_router;

///
/// Main, Entrypoint
///
#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

///
/// アプリケーションサーバーの起動
///
async fn bootstrap() -> Result<()> {
    let app = build_router();

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening on the {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
