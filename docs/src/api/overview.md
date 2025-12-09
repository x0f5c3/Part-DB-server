# API Overview

Part-DB provides a RESTful API for managing electronic component inventory. This section documents all available endpoints.

## Base URL

```
http://localhost:3000/api
```

## Authentication

Most endpoints require authentication via JWT token. See [Authentication](./authentication.md) for details.

Include the token in the `Authorization` header:

```http
Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
```

## Content Type

All requests and responses use JSON:

```http
Content-Type: application/json
```

## Endpoints Overview

### Public Endpoints (No Authentication)

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/health` | Health check |
| GET | `/api` | API information |
| GET | `/api-docs/openapi.json` | OpenAPI specification |
| POST | `/api/auth/login` | User login |

### Protected Endpoints (Authentication Required)

| Resource | Endpoints | Operations |
|----------|-----------|------------|
| [Parts](./parts.md) | `/api/parts` | List, Create, Read, Update, Delete |
| [Categories](./categories.md) | `/api/categories` | List, Create, Read, Update, Delete |
| [Storage Locations](./storage-locations.md) | `/api/storage_locations` | List, Read |
| [Footprints](./other-endpoints.md) | `/api/footprints` | List, Read |
| [Manufacturers](./other-endpoints.md) | `/api/manufacturers` | List, Read |
| [Suppliers](./other-endpoints.md) | `/api/suppliers` | List, Read |
| [Users](./other-endpoints.md) | `/api/users` | List, Read |

## Pagination

List endpoints support pagination with query parameters:

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number (1-indexed) |
| `per_page` | integer | 30 | Items per page (max 100) |

Example:

```http
GET /api/parts?page=2&per_page=50
```

Response includes pagination metadata:

```json
{
  "items": [...],
  "total": 150,
  "page": 2,
  "per_page": 50,
  "total_pages": 3
}
```

## Error Responses

All errors return a consistent JSON structure:

```json
{
  "message": "Human-readable error message",
  "code": "ERROR_CODE"
}
```

### HTTP Status Codes

| Status | Code | Description |
|--------|------|-------------|
| 200 | - | Success |
| 201 | - | Created |
| 204 | - | No Content (successful delete) |
| 400 | `BAD_REQUEST` | Invalid request data |
| 401 | `UNAUTHORIZED` | Authentication required |
| 403 | `FORBIDDEN` | Insufficient permissions |
| 404 | `NOT_FOUND` | Resource not found |
| 409 | `CONFLICT` | Resource conflict |
| 500 | `INTERNAL_ERROR` | Server error |

## Health Check

```http
GET /health
```

Response:

```json
{
  "status": "ok"
}
```

## API Information

```http
GET /api
```

Response:

```json
{
  "name": "Part-DB API",
  "version": "2.0.0",
  "description": "REST API for Part-DB inventory management"
}
```

## OpenAPI Specification

The complete API specification is available at:

```
GET /api-docs/openapi.json
```

This can be imported into tools like Swagger UI or Postman for interactive exploration.

## Rate Limiting

The API does not currently implement rate limiting. However, for production deployments, it's recommended to implement rate limiting at the reverse proxy level.

## Versioning

The current API version is v2. Future breaking changes will be introduced under new version prefixes (e.g., `/api/v3`).
