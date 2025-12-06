# API Routes

The Part-DB backend exposes a RESTful API for managing inventory data. All endpoints are prefixed with `/api`.

## Route Structure

### Public Routes (No Authentication)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check endpoint |
| GET | `/api` | API information |
| GET | `/api-docs/openapi.json` | OpenAPI specification |
| POST | `/api/auth/login` | User login |

### Protected Routes (Authentication Required)

All protected routes require a valid JWT token in the `Authorization` header.

#### Parts

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/parts` | List all parts (paginated) |
| GET | `/api/parts/{id}` | Get part by ID |
| POST | `/api/parts` | Create a new part |
| PATCH | `/api/parts/{id}` | Update a part |
| DELETE | `/api/parts/{id}` | Delete a part |

#### Categories

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/categories` | List all categories |
| GET | `/api/categories/{id}` | Get category by ID |
| POST | `/api/categories` | Create a new category |
| PATCH | `/api/categories/{id}` | Update a category |
| DELETE | `/api/categories/{id}` | Delete a category |

#### Other Resources (Read-only)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/footprints` | List all footprints |
| GET | `/api/footprints/{id}` | Get footprint by ID |
| GET | `/api/manufacturers` | List all manufacturers |
| GET | `/api/manufacturers/{id}` | Get manufacturer by ID |
| GET | `/api/storage_locations` | List all storage locations |
| GET | `/api/storage_locations/{id}` | Get storage location by ID |
| GET | `/api/suppliers` | List all suppliers |
| GET | `/api/suppliers/{id}` | Get supplier by ID |
| GET | `/api/users` | List all users |
| GET | `/api/users/{id}` | Get user by ID |

## Route Handler Implementation

Routes are defined in `backend/src/main.rs` and handlers are in `backend/src/routes.rs`.

### Router Definition

```rust
fn create_router(state: AppState) -> Router {
    let public_routes = Router::new()
        .route("/", get(routes::api_info))
        .route("/auth/login", post(auth::login));

    let protected_routes = Router::new()
        .route("/parts", get(routes::list_parts).post(routes::create_part))
        .route("/parts/{id}", 
            get(routes::get_part)
                .patch(routes::update_part)
                .delete(routes::delete_part))
        // ... more routes
        .layer(middleware::from_fn_with_state(
            state.clone(), 
            auth::auth_middleware
        ));

    Router::new()
        .route("/health", get(routes::health_check))
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(state)
}
```

### Handler Example

```rust
/// List all parts with pagination
pub async fn list_parts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<PaginatedResponse<Part>>, AppError> {
    let page = params.page.unwrap_or(1);
    let per_page = params.per_page.unwrap_or(30);
    let offset = (page - 1) * per_page;

    let parts = sqlx::query_as!(
        Part,
        r#"SELECT * FROM parts ORDER BY id LIMIT $1 OFFSET $2"#,
        per_page as i64,
        offset as i64
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| AppError::InternalError(e.to_string()))?;

    let total = sqlx::query_scalar!("SELECT COUNT(*) FROM parts")
        .fetch_one(&state.pool)
        .await
        .map_err(|e| AppError::InternalError(e.to_string()))?
        .unwrap_or(0);

    Ok(Json(PaginatedResponse {
        items: parts,
        total: total as i32,
        page,
        per_page,
        total_pages: ((total as f64) / (per_page as f64)).ceil() as i32,
    }))
}
```

## Query Parameters

### Pagination

List endpoints support pagination:

```
GET /api/parts?page=1&per_page=30
```

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number |
| `per_page` | integer | 30 | Items per page (max 100) |

### Response Format

Paginated responses include metadata:

```json
{
  "items": [...],
  "total": 100,
  "page": 1,
  "per_page": 30,
  "total_pages": 4
}
```

## Error Responses

All errors return a consistent JSON structure:

```json
{
  "message": "Resource not found",
  "code": "NOT_FOUND"
}
```

See [Error Handling](./error-handling.md) for more details.
