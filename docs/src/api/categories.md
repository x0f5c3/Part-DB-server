# Categories API

The Categories API allows you to manage part categories for organizing your inventory.

## List Categories

Retrieve a paginated list of all categories.

### Request

```http
GET /api/categories
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
      "name": "Microcontrollers",
      "description": "MCUs and microprocessors",
      "parent_id": null,
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    },
    {
      "id": 2,
      "name": "AVR",
      "description": "Atmel AVR microcontrollers",
      "parent_id": 1,
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    }
  ],
  "total": 20,
  "page": 1,
  "per_page": 30,
  "total_pages": 1
}
```

### Example

```bash
curl http://localhost:3000/api/categories \
  -H "Authorization: Bearer <token>"
```

## Get Category

Retrieve a single category by ID.

### Request

```http
GET /api/categories/{id}
Authorization: Bearer <token>
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Category ID |

### Response

**Success (200 OK)**

```json
{
  "id": 1,
  "name": "Microcontrollers",
  "description": "MCUs and microprocessors",
  "parent_id": null,
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-10T08:00:00Z"
}
```

**Error (404 Not Found)**

```json
{
  "message": "Category with id 999 not found",
  "code": "NOT_FOUND"
}
```

### Example

```bash
curl http://localhost:3000/api/categories/1 \
  -H "Authorization: Bearer <token>"
```

## Create Category

Create a new category.

### Request

```http
POST /api/categories
Authorization: Bearer <token>
Content-Type: application/json

{
  "name": "Microcontrollers",
  "description": "MCUs and microprocessors",
  "parent_id": null
}
```

### Request Body

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `name` | string | Yes | Category name |
| `description` | string | No | Category description |
| `parent_id` | integer | No | Parent category ID for nesting |

### Response

**Success (201 Created)**

```json
{
  "id": 1,
  "name": "Microcontrollers",
  "description": "MCUs and microprocessors",
  "parent_id": null,
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-10T08:00:00Z"
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
curl -X POST http://localhost:3000/api/categories \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name":"Microcontrollers","description":"MCUs and microprocessors"}'
```

## Update Category

Update an existing category.

### Request

```http
PATCH /api/categories/{id}
Authorization: Bearer <token>
Content-Type: application/json

{
  "description": "Updated description"
}
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Category ID |

### Request Body

All fields are optional. Only provided fields will be updated.

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Category name |
| `description` | string | Category description |
| `parent_id` | integer | Parent category ID |

### Response

**Success (200 OK)**

```json
{
  "id": 1,
  "name": "Microcontrollers",
  "description": "Updated description",
  "parent_id": null,
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-20T10:00:00Z"
}
```

**Error (404 Not Found)**

```json
{
  "message": "Category with id 999 not found",
  "code": "NOT_FOUND"
}
```

### Example

```bash
curl -X PATCH http://localhost:3000/api/categories/1 \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"description":"Updated description"}'
```

## Delete Category

Delete a category.

### Request

```http
DELETE /api/categories/{id}
Authorization: Bearer <token>
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Category ID |

### Response

**Success (204 No Content)**

No response body.

**Error (404 Not Found)**

```json
{
  "message": "Category with id 999 not found",
  "code": "NOT_FOUND"
}
```

**Error (409 Conflict)**

```json
{
  "message": "Cannot delete category with existing parts or subcategories",
  "code": "CONFLICT"
}
```

### Example

```bash
curl -X DELETE http://localhost:3000/api/categories/1 \
  -H "Authorization: Bearer <token>"
```

## Category Hierarchy

Categories support nesting via the `parent_id` field:

```json
{
  "id": 1,
  "name": "Semiconductors",
  "parent_id": null
}
{
  "id": 2,
  "name": "Microcontrollers",
  "parent_id": 1
}
{
  "id": 3,
  "name": "AVR",
  "parent_id": 2
}
```

This creates a hierarchy:

```
Semiconductors
└── Microcontrollers
    └── AVR
```
