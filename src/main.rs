use std::net::{Ipv4Addr, SocketAddr};

use axum::{routing::get, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello world" }));

    let addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), 8000);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Server listening on the {}", addr);

    axum::serve(listener, app).await.unwrap();
}
