mod controller;
mod service;
mod domain;
mod mapper;
mod engine;

use axum::{Router, routing::get};
use controller::hello_controller::hello_handler;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello_handler));

    println!("Rust-Autumn Server running on http://localhost:8080");
    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
