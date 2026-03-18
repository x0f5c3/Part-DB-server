//! Authentication middleware and utilities.
//!
//! This module provides JWT-based authentication for the API.
//! Supports both Supabase-issued JWTs (with UUID subjects and email claims)
//! and locally-issued JWTs for self-hosted deployments.

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

/// JWT claims structure – compatible with both Supabase and local JWTs.
///
/// Supabase JWTs carry a UUID string in `sub`, the user's email, and a `role`
/// field.  Locally-issued tokens carry an integer `user_id` and `username` for
/// backwards compatibility.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// Subject – UUID string for Supabase tokens, or stringified integer for local tokens.
    pub sub: String,
    /// Email address (present in Supabase tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Role assigned by Supabase (e.g. "authenticated", "anon").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    /// Username (present in locally-issued tokens).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Audience (Supabase sets this to "authenticated").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<String>,
    /// Expiration timestamp
    pub exp: usize,
    /// Issued at timestamp
    pub iat: usize,
}

/// Login request payload.
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    /// Username
    pub username: String,
    /// Password
    pub password: String,
}

/// Login response with JWT token.
#[derive(Debug, Serialize)]
pub struct LoginResponse {
    /// JWT access token
    pub token: String,
    /// Token type (always "Bearer")
    pub token_type: String,
    /// Token expiration in seconds
    pub expires_in: u64,
}

/// Get the JWT secret from environment.
///
/// For Supabase deployments set `SUPABASE_JWT_SECRET` (found under
/// Project Settings → API → JWT Settings in the Supabase dashboard).
/// For self-hosted deployments the `JWT_SECRET` variable is used instead.
fn get_jwt_secret() -> String {
    env::var("SUPABASE_JWT_SECRET")
        .or_else(|_| env::var("JWT_SECRET"))
        .unwrap_or_else(|_| "development-secret-change-in-production".to_string())
}

/// Generate a locally-issued JWT token for a user.
pub fn generate_token(user_id: i32, username: &str) -> Result<String, AppError> {
    let secret = get_jwt_secret();
    let now = chrono::Utc::now().timestamp() as usize;
    let expiration = now + 24 * 60 * 60; // 24 hours

    let claims = Claims {
        sub: user_id.to_string(),
        email: None,
        role: Some("authenticated".to_string()),
        username: Some(username.to_string()),
        aud: Some("authenticated".to_string()),
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
///
/// Accepts both Supabase-issued JWTs and locally-issued ones by validating
/// against the shared secret.  Audience validation is relaxed so that both
/// `"authenticated"` (Supabase) and plain tokens work without extra config.
pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let secret = get_jwt_secret();

    let mut validation = Validation::default();
    // Accept tokens without an audience claim (locally-issued) as well as
    // Supabase tokens whose audience is "authenticated".
    validation.validate_aud = false;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &validation,
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
    // For Supabase tokens, `sub` is a UUID string.  For local tokens it is a
    // stringified integer.  Attempt integer parse first; fall back to a name
    // lookup using the email claim.
    let user = if let Ok(user_id) = claims.sub.parse::<i32>() {
        sqlx::query_as::<_, crate::models::User>(
            r#"
            SELECT id, name, first_name, last_name, email, NULL as password, disabled,
                   config_theme, datetime_added, last_modified
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await?
    } else if let Some(email) = &claims.email {
        // Supabase UUID sub — look up by email
        sqlx::query_as::<_, crate::models::User>(
            r#"
            SELECT id, name, first_name, last_name, email, NULL as password, disabled,
                   config_theme, datetime_added, last_modified
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&state.pool)
        .await?
    } else {
        None
    };

    user.ok_or_else(|| AppError::NotFound("User not found".to_string()))
        .map(Json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_and_validate_token() {
        let token = generate_token(1, "testuser").unwrap();
        let claims = validate_token(&token).unwrap();

        assert_eq!(claims.sub, "1");
        assert_eq!(claims.username.as_deref(), Some("testuser"));
    }

    #[test]
    fn test_invalid_token() {
        let result = validate_token("invalid-token");
        assert!(result.is_err());
    }
}
