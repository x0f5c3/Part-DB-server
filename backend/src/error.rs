//! Application error types and conversions.
//!
//! This module provides a centralized error handling system that converts
//! various error types into appropriate HTTP responses.

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

/// API error response body.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// Error message
    pub message: String,
    /// Error code for programmatic handling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Additional details about the error
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<serde_json::Value>,
}

/// Application error type that can be converted to HTTP responses.
#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    /// Resource not found (404)
    NotFound(String),
    /// Bad request / validation error (400)
    BadRequest(String),
    /// Authentication required (401)
    Unauthorized(String),
    /// Access denied (403)
    Forbidden(String),
    /// Conflict (e.g., duplicate resource) (409)
    Conflict(String),
    /// Internal server error (500)
    Internal(String),
    /// Database error
    Database(sqlx::Error),
    /// Serialization error
    Serialization(serde_json::Error),
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound(msg) => write!(f, "Not Found: {}", msg),
            AppError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            AppError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            AppError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            AppError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            AppError::Internal(msg) => write!(f, "Internal Error: {}", msg),
            AppError::Database(e) => write!(f, "Database Error: {}", e),
            AppError::Serialization(e) => write!(f, "Serialization Error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_response) = match &self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    message: msg.clone(),
                    code: Some("NOT_FOUND".to_string()),
                    details: None,
                },
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: msg.clone(),
                    code: Some("BAD_REQUEST".to_string()),
                    details: None,
                },
            ),
            AppError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    message: msg.clone(),
                    code: Some("UNAUTHORIZED".to_string()),
                    details: None,
                },
            ),
            AppError::Forbidden(msg) => (
                StatusCode::FORBIDDEN,
                ErrorResponse {
                    message: msg.clone(),
                    code: Some("FORBIDDEN".to_string()),
                    details: None,
                },
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                ErrorResponse {
                    message: msg.clone(),
                    code: Some("CONFLICT".to_string()),
                    details: None,
                },
            ),
            AppError::Internal(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    message: msg.clone(),
                    code: Some("INTERNAL_ERROR".to_string()),
                    details: None,
                },
            ),
            AppError::Database(e) => {
                // Log the actual error but don't expose it to clients
                eprintln!("Database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        message: "A database error occurred".to_string(),
                        code: Some("DATABASE_ERROR".to_string()),
                        details: None,
                    },
                )
            }
            AppError::Serialization(e) => {
                eprintln!("Serialization error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    ErrorResponse {
                        message: "A serialization error occurred".to_string(),
                        code: Some("SERIALIZATION_ERROR".to_string()),
                        details: None,
                    },
                )
            }
        };

        (status, Json(error_response)).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match &err {
            sqlx::Error::RowNotFound => AppError::NotFound("Resource not found".to_string()),
            _ => AppError::Database(err),
        }
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Serialization(err)
    }
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::Internal(err.to_string())
    }
}

/// Result type alias for route handlers.
pub type AppResult<T> = Result<T, AppError>;
