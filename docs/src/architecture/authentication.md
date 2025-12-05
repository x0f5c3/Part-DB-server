# Authentication

Part-DB uses JWT (JSON Web Tokens) for authentication. This page documents the authentication system.

## Overview

The authentication flow:

```
┌─────────┐    POST /api/auth/login     ┌─────────┐
│ Client  │ ─────────────────────────►  │ Backend │
│         │  { username, password }     │         │
│         │                             │         │
│         │ ◄───────────────────────── │         │
│         │  { token, user }            │         │
└─────────┘                             └─────────┘
     │
     │  Authorization: Bearer <token>
     │
     ▼
┌─────────────────────────────────────────────────┐
│              Protected API Endpoints            │
└─────────────────────────────────────────────────┘
```

## Login

### Request

```http
POST /api/auth/login
Content-Type: application/json

{
  "username": "user",
  "password": "password"
}
```

### Response

```json
{
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "user": {
    "id": 1,
    "username": "user",
    "email": "user@example.com",
    "is_admin": false
  }
}
```

### Errors

| Status | Description |
|--------|-------------|
| 400 | Missing username or password |
| 401 | Invalid credentials |

## Token Usage

Include the JWT token in the `Authorization` header for protected endpoints:

```http
GET /api/parts
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## JWT Structure

The JWT token contains:

```json
{
  "sub": "1",           // User ID
  "username": "user",   // Username
  "is_admin": false,    // Admin flag
  "exp": 1701234567,    // Expiration timestamp
  "iat": 1701148167     // Issued at timestamp
}
```

## Implementation Details

### auth.rs

```rust
/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // Subject (user ID)
    pub username: String,
    pub is_admin: bool,
    pub exp: usize,         // Expiration
    pub iat: usize,         // Issued at
}

/// Login handler
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // Validate credentials
    let user = sqlx::query_as!(User, 
        "SELECT * FROM users WHERE username = $1", 
        payload.username
    )
    .fetch_optional(&state.pool)
    .await?
    .ok_or(AppError::Unauthorized)?;

    // Verify password
    if !verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    // Generate JWT
    let token = create_jwt(&user)?;

    Ok(Json(LoginResponse { token, user }))
}

/// Authentication middleware
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from header
    let token = req.headers()
        .get(AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(AppError::Unauthorized)?;

    // Validate token
    let claims = validate_jwt(token)?;
    
    // Add claims to request extensions
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
```

### Password Hashing

Passwords are hashed using bcrypt:

```rust
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::InternalError(e.to_string()))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|e| AppError::InternalError(e.to_string()))
}
```

## Configuration

Configure JWT settings via environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `JWT_SECRET` | Secret key for signing tokens | Auto-generated |
| `JWT_EXPIRY` | Token expiry in seconds | 86400 (24 hours) |

Example:

```env
JWT_SECRET=your-super-secret-key-at-least-32-characters-long
JWT_EXPIRY=3600
```

## Security Considerations

1. **Use HTTPS in production** - Tokens are transmitted in plain text over HTTP
2. **Set strong secrets** - Use a random string of at least 32 characters
3. **Short token expiry** - Use shorter expiry for sensitive operations
4. **Rotate secrets** - Periodically rotate JWT secrets
5. **Validate on every request** - The middleware validates tokens for each request

## Current User

Get the current authenticated user:

```http
GET /api/auth/me
Authorization: Bearer <token>
```

Response:

```json
{
  "id": 1,
  "username": "user",
  "email": "user@example.com",
  "is_admin": false
}
```

This endpoint is useful for:
- Validating that a token is still valid
- Getting user information after page refresh
- Checking user permissions
