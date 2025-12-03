//! Database connection and initialization helpers.
//!
//! This module provides utilities for setting up the database connection pool
//! and handling database configuration.

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::env;
use std::time::Duration;

/// Database configuration loaded from environment variables.
#[derive(Debug, Clone)]
pub struct DbConfig {
    /// Database connection URL
    pub database_url: String,
    /// Maximum number of connections in the pool
    pub max_connections: u32,
    /// Connection timeout in seconds
    pub connect_timeout: u64,
}

impl DbConfig {
    /// Creates a new database configuration from environment variables.
    ///
    /// Required environment variables:
    /// - `DATABASE_URL`: The PostgreSQL connection URL
    ///
    /// Optional environment variables:
    /// - `DB_MAX_CONNECTIONS`: Maximum pool connections (default: 10)
    /// - `DB_CONNECT_TIMEOUT`: Connection timeout in seconds (default: 30)
    pub fn from_env() -> Result<Self> {
        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgres://localhost/partdb".to_string());
        
        let max_connections = env::var("DB_MAX_CONNECTIONS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(10);
        
        let connect_timeout = env::var("DB_CONNECT_TIMEOUT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(30);

        Ok(Self {
            database_url,
            max_connections,
            connect_timeout,
        })
    }
}

/// Creates a new PostgreSQL connection pool.
///
/// This function reads configuration from environment variables and creates
/// a connection pool with the specified settings.
///
/// # Errors
///
/// Returns an error if the database connection cannot be established.
pub async fn create_pool(config: &DbConfig) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(Duration::from_secs(config.connect_timeout))
        .connect(&config.database_url)
        .await?;
    
    Ok(pool)
}

/// State shared across all route handlers.
#[derive(Clone)]
pub struct AppState {
    /// Database connection pool
    pub pool: PgPool,
}

impl AppState {
    /// Creates a new application state with the given database pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_config_defaults() {
        // Clear any existing env vars for clean test
        env::remove_var("DATABASE_URL");
        env::remove_var("DB_MAX_CONNECTIONS");
        env::remove_var("DB_CONNECT_TIMEOUT");
        
        let config = DbConfig::from_env().unwrap();
        assert_eq!(config.database_url, "postgres://localhost/partdb");
        assert_eq!(config.max_connections, 10);
        assert_eq!(config.connect_timeout, 30);
    }
}