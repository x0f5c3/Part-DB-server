# Part-DB Backend

A modern Rust implementation of the Part-DB API server using the Axum web framework
and SQLX for database access.

## Features

- **REST API** compatible with the original PHP Part-DB application
- **PostgreSQL** database support via SQLX
- **Type-safe** models with Serde serialization
- **Error handling** with proper HTTP status codes
- **Pagination** support for list endpoints
- **Health check** endpoint for monitoring

## Tech Stack

- **Rust 2021 Edition**
- **Axum 0.8** - Web framework
- **SQLX 0.8** - Database toolkit
- **Tokio** - Async runtime
- **Serde** - Serialization/deserialization
- **Tower-HTTP** - HTTP middleware (CORS, tracing)

## Getting Started

### Prerequisites

- Rust 1.70+ (install via [rustup](https://rustup.rs/))
- PostgreSQL 14+

### Configuration

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://user:password@localhost/partdb
DB_MAX_CONNECTIONS=10
DB_CONNECT_TIMEOUT=30
HOST=0.0.0.0
PORT=3000
```

### Running

```bash
# Development
cargo run

# Production
cargo build --release
./target/release/backend
```

### Testing

```bash
cargo test
```

## API Endpoints

### Health & Info

| Method | Path | Description |
|--------|------|-------------|
| GET | `/health` | Health check |
| GET | `/api` | API information |

### Parts

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/parts` | List all parts (paginated) |
| GET | `/api/parts/{id}` | Get part by ID |
| POST | `/api/parts` | Create a new part |
| PATCH | `/api/parts/{id}` | Update a part |
| DELETE | `/api/parts/{id}` | Delete a part |

### Categories

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/categories` | List all categories |
| GET | `/api/categories/{id}` | Get category by ID |
| POST | `/api/categories` | Create a new category |
| PATCH | `/api/categories/{id}` | Update a category |
| DELETE | `/api/categories/{id}` | Delete a category |

### Other Resources (Read-only)

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

## Pagination

List endpoints support pagination via query parameters:

```
GET /api/parts?page=1&per_page=30
```

Response includes pagination metadata:

```json
{
  "items": [...],
  "total": 100,
  "page": 1,
  "per_page": 30,
  "total_pages": 4
}
```

## Error Handling

All errors return a consistent JSON structure:

```json
{
  "message": "Resource not found",
  "code": "NOT_FOUND"
}
```

HTTP status codes:
- `200` - Success
- `201` - Created
- `204` - No Content (delete)
- `400` - Bad Request
- `401` - Unauthorized
- `403` - Forbidden
- `404` - Not Found
- `409` - Conflict
- `500` - Internal Server Error

## Project Structure

```
backend/
├── Cargo.toml          # Dependencies
├── src/
│   ├── main.rs         # Entry point & server setup
│   ├── lib.rs          # Library exports for tests
│   ├── db.rs           # Database connection pool
│   ├── error.rs        # Error types & handling
│   ├── models.rs       # Data models & DTOs
│   └── routes.rs       # API route handlers
└── tests/
    └── api_tests.rs    # Integration tests
```

## License

This project is licensed under AGPL-3.0, same as the original Part-DB project.
