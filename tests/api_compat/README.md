# API Compatibility Tests

This directory contains tests to verify that the new Rust backend produces
responses that are compatible with the original PHP API.

## Structure

- `snapshots/` - JSON snapshots of expected API responses
- `rust_backend/` - Tests for the Rust/Axum backend

## Running Tests

### Rust Backend Tests

```bash
cd backend
cargo test
```

### Snapshot Tests

Snapshot tests compare the response structure and key fields between the
PHP and Rust implementations to ensure API compatibility.

## Compatibility Requirements

The Rust backend must:

1. Use the same URL paths as the PHP API (under `/api` prefix)
2. Return the same JSON structure for each endpoint
3. Use the same field names and types
4. Maintain the same HTTP status codes for success/error cases

## Endpoints to Test

| Endpoint | Method | Status |
|----------|--------|--------|
| GET /api/parts | GET | ✅ Implemented |
| GET /api/parts/{id} | GET | ✅ Implemented |
| POST /api/parts | POST | ✅ Implemented |
| PATCH /api/parts/{id} | PATCH | ✅ Implemented |
| DELETE /api/parts/{id} | DELETE | ✅ Implemented |
| GET /api/categories | GET | ✅ Implemented |
| GET /api/categories/{id} | GET | ✅ Implemented |
| POST /api/categories | POST | ✅ Implemented |
| PATCH /api/categories/{id} | PATCH | ✅ Implemented |
| DELETE /api/categories/{id} | DELETE | ✅ Implemented |
| GET /api/footprints | GET | ✅ Implemented |
| GET /api/footprints/{id} | GET | ✅ Implemented |
| GET /api/manufacturers | GET | ✅ Implemented |
| GET /api/manufacturers/{id} | GET | ✅ Implemented |
| GET /api/storage_locations | GET | ✅ Implemented |
| GET /api/storage_locations/{id} | GET | ✅ Implemented |
| GET /api/suppliers | GET | ✅ Implemented |
| GET /api/suppliers/{id} | GET | ✅ Implemented |
| GET /api/users | GET | ✅ Implemented |
| GET /api/users/{id} | GET | ✅ Implemented |
