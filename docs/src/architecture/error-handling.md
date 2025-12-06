# Error Handling

Part-DB uses a structured approach to error handling that provides consistent error responses across all API endpoints.

## Error Response Format

All errors return a consistent JSON structure:

```json
{
  "message": "Human-readable error message",
  "code": "ERROR_CODE"
}
```

## HTTP Status Codes

| Status | Code | Description |
|--------|------|-------------|
| 400 | `BAD_REQUEST` | Invalid request data |
| 401 | `UNAUTHORIZED` | Authentication required or failed |
| 403 | `FORBIDDEN` | Insufficient permissions |
| 404 | `NOT_FOUND` | Resource not found |
| 409 | `CONFLICT` | Resource conflict (e.g., duplicate) |
| 500 | `INTERNAL_ERROR` | Server error |

## Error Type Implementation

The `AppError` enum defines all application errors:

```rust
// error.rs
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    Forbidden,
    Conflict(String),
    InternalError(String),
}

#[derive(Serialize)]
struct ErrorResponse {
    message: String,
    code: &'static str,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message) = match self {
            AppError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                "NOT_FOUND",
                msg,
            ),
            AppError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                "BAD_REQUEST",
                msg,
            ),
            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                "UNAUTHORIZED",
                "Authentication required".to_string(),
            ),
            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                "FORBIDDEN",
                "Insufficient permissions".to_string(),
            ),
            AppError::Conflict(msg) => (
                StatusCode::CONFLICT,
                "CONFLICT",
                msg,
            ),
            AppError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "INTERNAL_ERROR",
                msg,
            ),
        };

        (status, Json(ErrorResponse { message, code })).into_response()
    }
}
```

## Error Conversion

The error type implements conversions for common error types:

```rust
// From SQLX errors
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => {
                AppError::NotFound("Resource not found".to_string())
            }
            _ => AppError::InternalError(err.to_string()),
        }
    }
}

// From JWT errors
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(_err: jsonwebtoken::errors::Error) -> Self {
        AppError::Unauthorized
    }
}

// From bcrypt errors
impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> Self {
        AppError::InternalError(err.to_string())
    }
}
```

## Usage in Handlers

Route handlers return `Result<T, AppError>`:

```rust
pub async fn get_part(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Part>, AppError> {
    let part = sqlx::query_as!(Part, "SELECT * FROM parts WHERE id = $1", id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Part with id {} not found", id)))?;

    Ok(Json(part))
}

pub async fn create_part(
    State(state): State<AppState>,
    Json(payload): Json<CreatePart>,
) -> Result<(StatusCode, Json<Part>), AppError> {
    // Validate input
    if payload.name.is_empty() {
        return Err(AppError::BadRequest("Name is required".to_string()));
    }

    // Create part
    let part = sqlx::query_as!(Part, 
        "INSERT INTO parts (name, description) VALUES ($1, $2) RETURNING *",
        payload.name, payload.description
    )
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(part)))
}
```

## Validation Errors

For complex validation, return detailed error messages:

```rust
pub async fn create_part(
    State(state): State<AppState>,
    Json(payload): Json<CreatePart>,
) -> Result<(StatusCode, Json<Part>), AppError> {
    // Collect validation errors
    let mut errors = Vec::new();

    if payload.name.is_empty() {
        errors.push("Name is required");
    }
    if payload.name.len() > 255 {
        errors.push("Name must be 255 characters or less");
    }
    if payload.quantity.map(|q| q < 0).unwrap_or(false) {
        errors.push("Quantity cannot be negative");
    }

    if !errors.is_empty() {
        return Err(AppError::BadRequest(errors.join("; ")));
    }

    // Continue with creation...
}
```

## Logging Errors

Errors are logged for debugging:

```rust
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log internal errors
        if let AppError::InternalError(ref msg) = self {
            tracing::error!("Internal error: {}", msg);
        }

        // Convert to response...
    }
}
```

## Client-Side Handling

The frontend can handle errors consistently:

```typescript
async function fetchPart(id: number): Promise<Part> {
  const response = await fetch(`/api/parts/${id}`);
  
  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.message);
  }
  
  return response.json();
}

// Usage
try {
  const part = await fetchPart(123);
} catch (error) {
  console.error('Failed to fetch part:', error.message);
  // Display error to user
}
```
