//! Authentication middleware and utilities.
//!
//! This module provides JWT-based authentication for the API.

use axum::{
    extract::{Request, State},
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;

use crate::db::AppState;
use crate::error::AppError;

/// JWT claims structure.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject (user ID)
    pub sub: i32,
    /// Username
    pub username: String,
    /// Expiration timestamp
    pub exp: usize,
    /// Issued at timestamp
    pub iat: usize,
}

/// Login request payload.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}

/// Login response with JWT token.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// JWT access token
    pub token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Token expiration in seconds
    pub expires_in: u64,
}

/// Get the JWT secret from environment or use a default for development.
fn get_jwt_secret() -> String {
    env::var("JWT_SECRET").unwrap_or_else(|_| "development-secret-change-in-production".to_string())
}

/// Generate a JWT token for a user.
pub fn generate_token(user_id: i32, username: &str) -> Result<String, AppError> {
    let secret = get_jwt_secret();
    let now = chrono::Utc::now().timestamp() as usize;
    let expiration = now + 24 * 60 * 60; // 24 hours

    let claims = Claims {
        sub: user_id,
        username: username.to_string(),
        exp: expiration,
        iat: now,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))
}

/// Validate a JWT token and extract claims.
pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let secret = get_jwt_secret();

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
}

/// Authentication middleware that validates JWT tokens.
///
/// This middleware checks for a valid JWT token in the Authorization header.
/// If the token is valid, the request proceeds; otherwise, it returns 401.
pub async fn auth_middleware(
    State(_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    // Extract the Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "Missing or invalid Authorization header",
                    "code": "UNAUTHORIZED"
                })),
            ));
        }
    };

    // Validate the token
    match validate_token(token) {
        Ok(claims) => {
            // Store claims in request extensions for use by handlers
            request.extensions_mut().insert(claims);
            Ok(next.run(request).await)
        }
        Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Invalid or expired token",
                "code": "UNAUTHORIZED"
            })),
        )),
    }
}

/// Optional authentication middleware that doesn't require a token.
///
/// This middleware attempts to validate a token if present, but allows
/// the request to proceed even without authentication.
pub async fn optional_auth_middleware(
    State(_state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    // Extract the Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());

    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let token = &header[7..];
            if let Ok(claims) = validate_token(token) {
                request.extensions_mut().insert(claims);
            }
        }
    }

    next.run(request).await
}

/// Login handler - authenticates a user and returns a JWT token.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Find user by username
    let user = sqlx::query_as::<_, (i32, String, Option<String>)>(
        "SELECT id, name, password FROM users WHERE name = $1 AND disabled = false",
    )
    .bind(&payload.username)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Invalid username or password".to_string()))?;

    let (user_id, username, password_hash) = user;

    // Verify password
    let password_hash = password_hash
        .ok_or_else(|| AppError::Unauthorized("Invalid username or password".to_string()))?;

    let password_valid = bcrypt::verify(&payload.password, &password_hash)
        .map_err(|_| AppError::Unauthorized("Invalid username or password".to_string()))?;

    if !password_valid {
        return Err(AppError::Unauthorized(
            "Invalid username or password".to_string(),
        ));
    }

    // Generate token
    let token = generate_token(user_id, &username)?;

    Ok(Json(LoginResponse {
        token,
        token_type: "Bearer".to_string(),
        expires_in: 24 * 60 * 60,
    }))
}

/// Get current user info from token.
pub async fn get_current_user(
    State(state): State<AppState>,
    claims: axum::Extension<Claims>,
) -> Result<Json<crate::models::User>, AppError> {
    let user = sqlx::query_as::<_, crate::models::User>(
        r#"
        SELECT id, name, first_name, last_name, email, NULL as password, disabled,
               config_theme, datetime_added, last_modified
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound("User not found".to_string()))?;

    Ok(Json(user))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let token = generate_token(1, "testuser").unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, 1);
        assert_eq!(claims.username, "testuser");
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid-token");
        assert!(result.is_err());
    }
}
