# Storage Locations API

The Storage Locations API allows you to manage physical locations where parts are stored.

## List Storage Locations

Retrieve a paginated list of all storage locations.

### Request

```http
GET /api/storage_locations
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
      "name": "Cabinet A",
      "description": "Main storage cabinet",
      "parent_id": null,
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    },
    {
      "id": 2,
      "name": "Drawer 1",
      "description": "Top drawer",
      "parent_id": 1,
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    }
  ],
  "total": 15,
  "page": 1,
  "per_page": 30,
  "total_pages": 1
}
```

### Example

```bash
curl http://localhost:3000/api/storage_locations \
  -H "Authorization: Bearer <token>"
```

## Get Storage Location

Retrieve a single storage location by ID.

### Request

```http
GET /api/storage_locations/{id}
Authorization: Bearer <token>
```

### Path Parameters

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | integer | Storage location ID |

### Response

**Success (200 OK)**

```json
{
  "id": 1,
  "name": "Cabinet A",
  "description": "Main storage cabinet",
  "parent_id": null,
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-10T08:00:00Z"
}
```

**Error (404 Not Found)**

```json
{
  "message": "Storage location with id 999 not found",
  "code": "NOT_FOUND"
}
```

### Example

```bash
curl http://localhost:3000/api/storage_locations/1 \
  -H "Authorization: Bearer <token>"
```

## Storage Location Hierarchy

Storage locations support nesting via the `parent_id` field:

```json
{
  "id": 1,
  "name": "Cabinet A",
  "parent_id": null
}
{
  "id": 2,
  "name": "Drawer 1",
  "parent_id": 1
}
{
  "id": 3,
  "name": "Compartment 1A",
  "parent_id": 2
}
```

This creates a hierarchy:

```
Cabinet A
└── Drawer 1
    └── Compartment 1A
```

## Use Cases

### Organizing by Physical Location

```
Workshop
├── Bench 1
│   ├── Drawer A
│   └── Drawer B
└── Shelf Unit
    ├── Shelf 1
    └── Shelf 2
```

### Organizing by Container Type

```
SMD Storage
├── Box 1 (0402)
├── Box 2 (0603)
└── Box 3 (0805)
```

## Assigning Parts to Locations

When creating or updating a part, set the `storage_location_id`:

```http
POST /api/parts
Content-Type: application/json
Authorization: Bearer <token>

{
  "name": "100Ω Resistor",
  "storage_location_id": 3,
  "quantity": 100
}
```
