mod controller;
mod service;
mod domain;
mod mapper;
mod engine;

use axum::{Router, routing::get};
use controller::hello_controller::hello_handler;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_handler));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("Rust-Autumn Server running on http://{}", addr);

    // Axum 0.7: hyper::Server 사용
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
