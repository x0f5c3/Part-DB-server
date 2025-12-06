# Backend Architecture

The Part-DB backend is built with Rust using the Axum web framework. This page documents the internal architecture and design decisions.

## Overview

The backend follows a layered architecture:

```
┌─────────────────────────────────────────┐
│              Route Handlers             │
│  (routes.rs - HTTP request handling)    │
├─────────────────────────────────────────┤
│            Business Logic               │
│  (auth.rs, validation, processing)      │
├─────────────────────────────────────────┤
│             Data Models                 │
│  (models.rs - DTOs and entities)        │
├─────────────────────────────────────────┤
│          Database Access                │
│  (db.rs - SQLX queries and pool)        │
└─────────────────────────────────────────┘
```

## Core Components

### Entry Point (main.rs)

The main entry point handles:

- Loading environment configuration
- Initializing the database connection pool
- Setting up middleware (CORS, tracing)
- Configuring routes
- Starting the HTTP server

```rust
#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    let db_config = DbConfig::from_env()?;
    let pool = db::create_pool(&db_config).await?;
    let state = AppState::new(pool);
    
    let app = create_router(state);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

### Router Configuration

Routes are organized into public and protected groups:

```rust
fn create_router(state: AppState) -> Router {
    // Public routes (no auth required)
    let public_routes = Router::new()
        .route("/", get(routes::api_info))
        .route("/auth/login", post(auth::login));

    // Protected routes (auth required)
    let protected_routes = Router::new()
        .route("/parts", get(list_parts).post(create_part))
        .route("/parts/{id}", get(get_part).patch(update_part).delete(delete_part))
        // ... more routes
        .layer(middleware::from_fn_with_state(state.clone(), auth::auth_middleware));

    Router::new()
        .route("/health", get(health_check))
        .nest("/api", public_routes.merge(protected_routes))
        .layer(CorsLayer::new().allow_origin(Any))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
```

### Application State

The `AppState` struct holds shared resources:

```rust
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
```

## Key Modules

### db.rs - Database Access

Manages the database connection pool and configuration:

- `DbConfig` - Configuration struct from environment variables
- `create_pool()` - Creates and configures the connection pool
- `AppState` - Holds the pool for handler access

### models.rs - Data Models

Defines the data structures used throughout the application:

- Entity structs (Part, Category, etc.)
- DTOs for API requests/responses
- Pagination structures

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Part {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    // ...
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePart {
    pub name: String,
    pub description: Option<String>,
    // ...
}
```

### routes.rs - Route Handlers

Contains the HTTP handler functions:

```rust
pub async fn list_parts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Part>>, AppError> {
    let parts = sqlx::query_as!(Part, "SELECT * FROM parts LIMIT $1 OFFSET $2", ...)
        .fetch_all(&state.pool)
        .await?;
    
    Ok(Json(PaginatedResponse { items: parts, ... }))
}
```

### error.rs - Error Handling

Defines application error types and HTTP response conversion:

```rust
pub enum AppError {
    NotFound(String),
    BadRequest(String),
    Unauthorized,
    InternalError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            // ...
        };
        
        (status, Json(ErrorResponse { message })).into_response()
    }
}
```

### auth.rs - Authentication

Handles user authentication and authorization:

- JWT token generation and validation
- Login endpoint
- Authentication middleware

See [Authentication](./authentication.md) for details.

## Middleware Stack

The application uses several middleware layers:

1. **TraceLayer** - Request/response logging
2. **CorsLayer** - Cross-Origin Resource Sharing
3. **Auth Middleware** - JWT token validation (protected routes only)

## Dependencies

Key dependencies from `Cargo.toml`:

```toml
[dependencies]
axum = "0.8"           # Web framework
tokio = "1"            # Async runtime
sqlx = "0.8"           # Database
serde = "1"            # Serialization
tower-http = "0.6"     # HTTP middleware
jsonwebtoken = "9"     # JWT handling
utoipa = "5"           # OpenAPI docs
```

## Testing

The backend includes unit and integration tests:

```bash
# Run all tests
cargo test

# Run with logging
RUST_LOG=debug cargo test -- --nocapture
```

See [Testing](../development/testing.md) for more details.
