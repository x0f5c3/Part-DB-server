//! Part-DB Backend Server
//!
//! This is the main entry point for the Part-DB Rust backend server.
//! It provides a REST API compatible with the original PHP application.

use anyhow::Result;
use axum::{routing::get, Router};
use std::env;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

mod db;
mod error;
mod models;
mod routes;

use db::{AppState, DbConfig};

/// Creates the application router with all routes.
fn create_router(state: AppState) -> Router {
    // CORS configuration for development
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // API routes under /api prefix (matching the PHP API structure)
    let api_routes = Router::new()
        // API info
        .route("/", get(routes::api_info))
        // Parts
        .route("/parts", get(routes::list_parts).post(routes::create_part))
        .route(
            "/parts/{id}",
            get(routes::get_part)
                .patch(routes::update_part)
                .delete(routes::delete_part),
        )
        // Categories
        .route(
            "/categories",
            get(routes::list_categories).post(routes::create_category),
        )
        .route(
            "/categories/{id}",
            get(routes::get_category)
                .patch(routes::update_category)
                .delete(routes::delete_category),
        )
        // Footprints
        .route("/footprints", get(routes::list_footprints))
        .route("/footprints/{id}", get(routes::get_footprint))
        // Manufacturers
        .route("/manufacturers", get(routes::list_manufacturers))
        .route("/manufacturers/{id}", get(routes::get_manufacturer))
        // Storage Locations
        .route("/storage_locations", get(routes::list_storage_locations))
        .route("/storage_locations/{id}", get(routes::get_storage_location))
        // Suppliers
        .route("/suppliers", get(routes::list_suppliers))
        .route("/suppliers/{id}", get(routes::get_supplier))
        // Users
        .route("/users", get(routes::list_users))
        .route("/users/{id}", get(routes::get_user));

    // Main router
    Router::new()
        .route("/health", get(routes::health_check))
        .nest("/api", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Initialize logging
    tracing_subscriber::fmt::init();

    // Get server configuration
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{}:{}", host, port);

    // Create database connection pool
    let db_config = DbConfig::from_env()?;

    // Check if we should skip database connection (for basic health checks)
    let skip_db = env::var("SKIP_DB").is_ok();

    let app = if skip_db {
        // Create a minimal router without database
        println!("Starting in no-database mode (health check only)");
        Router::new()
            .route("/health", get(routes::health_check))
            .route("/api", get(routes::api_info))
    } else {
        println!("Connecting to database...");
        let pool = db::create_pool(&db_config).await?;
        println!("Database connection established");

        let state = AppState::new(pool);
        create_router(state)
    };

    // Start the server
    println!("Part-DB Backend starting on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
