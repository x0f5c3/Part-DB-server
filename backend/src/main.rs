use axum::{Router, routing::get};
use std::net::SocketAddr;

mod routes;
mod models;
mod db;
mod error;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // Build application with a single health route.
    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    // Bind to 0.0.0.0:3000 by default. Adjust as necessary.
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Backend running on http://{addr}");
    axum::serve(listener, app).await.unwrap();
}