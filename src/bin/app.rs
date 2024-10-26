use std::net::{Ipv4Addr, SocketAddr};

use anyhow::{Error, Result};
use axum::Router;

use interfaces::rest::routes::build_rest_router;

/**
 * Main
 */
#[tokio::main]
async fn main() -> Result<()> {
    bootstrap().await
}

/**
 * Start application server
 */
async fn bootstrap() -> Result<()> {
    let app = Router::new().merge(build_rest_router());

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening on the {}", addr);

    axum::serve(listener, app).await.map_err(Error::from)
}
