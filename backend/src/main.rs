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
    println!("Backend running on http://{addr}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}