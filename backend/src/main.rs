use axum::{Router, routing::get};
use anyhow::Result;

mod routes;
mod models;
mod db;
mod error;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    // Build application with a single health route.
    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    // Bind to 0.0.0.0:3000 by default. Adjust as necessary.
    println!("Backend running on http://0.0.0.0:3000/");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
