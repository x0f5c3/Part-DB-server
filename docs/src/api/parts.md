# Parts API

The Parts API allows you to manage electronic components in your inventory.

## List Parts

Retrieve a paginated list of all parts.

### Request

```http
GET /api/parts
Authorization: Bearer <token>
```

### Query Parameters

| Parameter | Type | Default | Description |
|-----------|------|---------|-------------|
| `page` | integer | 1 | Page number |
| `per_page` | integer | 30 | Items per page (max 100) |

### Response

**Success (200 OK)**

```json
{
  "items": [
    {
      "id": 1,
      "name": "ATmega328P",
      "description": "8-bit AVR Microcontroller",
      "category_id": 5,
      "footprint_id": 2,
      "manufacturer_id": 1,
      "storage_location_id": 10,
      "quantity": 25,
      "minimum_quantity": 5,
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-20T14:45:00Z"
    }
  ],
  "total": 150,
  "page": 1,
  "per_page": 30,
  "total_pages": 5
}
```

### Example

```bash
curl http://localhost:3000/api/parts?page=1&per_page=10 \
  -H "Authorization: Bearer <token>"
```

## Get Part

Retrieve a single part by ID.

### Request

```http
GET /api/parts/{id}
Authorization: Bearer <token>
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Part ID |

### Response

**Success (200 OK)**

```json
{
  "id": 1,
  "name": "ATmega328P",
  "description": "8-bit AVR Microcontroller",
  "category_id": 5,
  "footprint_id": 2,
  "manufacturer_id": 1,
  "storage_location_id": 10,
  "quantity": 25,
  "minimum_quantity": 5,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-20T14:45:00Z"
}
```

**Error (404 Not Found)**

```json
{
  "message": "Part with id 999 not found",
  "code": "NOT_FOUND"
}
```

### Example

```bash
curl http://localhost:3000/api/parts/1 \
  -H "Authorization: Bearer <token>"
```

## Create Part

Create a new part.

### Request

```http
POST /api/parts
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "ATmega328P",
  "description": "8-bit AVR Microcontroller",
  "category_id": 5,
  "footprint_id": 2,
  "manufacturer_id": 1,
  "storage_location_id": 10,
  "quantity": 25,
  "minimum_quantity": 5
}
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Part name |
| `description` | string | No | Part description |
| `category_id` | integer | No | Category ID |
| `footprint_id` | integer | No | Footprint ID |
| `manufacturer_id` | integer | No | Manufacturer ID |
| `storage_location_id` | integer | No | Storage location ID |
| `quantity` | integer | No | Current quantity (default: 0) |
| `minimum_quantity` | integer | No | Minimum stock level (default: 0) |

### Response

**Success (201 Created)**

```json
{
  "id": 1,
  "name": "ATmega328P",
  "description": "8-bit AVR Microcontroller",
  "category_id": 5,
  "footprint_id": 2,
  "manufacturer_id": 1,
  "storage_location_id": 10,
  "quantity": 25,
  "minimum_quantity": 5,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T10:30:00Z"
}
```

**Error (400 Bad Request)**

```json
{
  "message": "Name is required",
  "code": "BAD_REQUEST"
}
```

### Example

```bash
curl -X POST http://localhost:3000/api/parts \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"ATmega328P","quantity":25}'
```

## Update Part

Update an existing part.

### Request

```http
PATCH /api/parts/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "quantity": 30,
  "description": "Updated description"
}
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Part ID |

### Request Body

All fields are optional. Only provided fields will be updated.

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Part name |
| `description` | string | Part description |
| `category_id` | integer | Category ID |
| `footprint_id` | integer | Footprint ID |
| `manufacturer_id` | integer | Manufacturer ID |
| `storage_location_id` | integer | Storage location ID |
| `quantity` | integer | Current quantity |
| `minimum_quantity` | integer | Minimum stock level |

### Response

**Success (200 OK)**

```json
{
  "id": 1,
  "name": "ATmega328P",
  "description": "Updated description",
  "category_id": 5,
  "footprint_id": 2,
  "manufacturer_id": 1,
  "storage_location_id": 10,
  "quantity": 30,
  "minimum_quantity": 5,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-20T14:45:00Z"
}
```

**Error (404 Not Found)**

```json
{
  "message": "Part with id 999 not found",
  "code": "NOT_FOUND"
}
```

### Example

```bash
curl -X PATCH http://localhost:3000/api/parts/1 \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"quantity":30}'
```

## Delete Part

Delete a part.

### Request

```http
DELETE /api/parts/{id}
Authorization: Bearer <token>
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Part ID |

### Response

**Success (204 No Content)**

No response body.

**Error (404 Not Found)**

```json
{
  "message": "Part with id 999 not found",
  "code": "NOT_FOUND"
}
```

### Example

```bash
curl -X DELETE http://localhost:3000/api/parts/1 \
  -H "Authorization: Bearer <token>"
```
