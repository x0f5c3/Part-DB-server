# Other Endpoints

This page documents additional read-only API endpoints.

## Footprints

Footprints represent the physical package type of a component.

### List Footprints

```http
GET /api/footprints
Authorization: Bearer <token>
```

#### Response

```json
{
  "items": [
    {
      "id": 1,
      "name": "DIP-28",
      "description": "28-pin Dual In-line Package",
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    },
    {
      "id": 2,
      "name": "TQFP-32",
      "description": "32-pin Thin Quad Flat Package",
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    }
  ],
  "total": 10,
  "page": 1,
  "per_page": 30,
  "total_pages": 1
}
```

### Get Footprint

```http
GET /api/footprints/{id}
Authorization: Bearer <token>
```

#### Response

```json
{
  "id": 1,
  "name": "DIP-28",
  "description": "28-pin Dual In-line Package",
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-10T08:00:00Z"
}
```

---

## Manufacturers

Manufacturers represent companies that produce components.

### List Manufacturers

```http
GET /api/manufacturers
Authorization: Bearer <token>
```

#### Response

```json
{
  "items": [
    {
      "id": 1,
      "name": "Microchip",
      "website": "https://www.microchip.com",
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    },
    {
      "id": 2,
      "name": "Texas Instruments",
      "website": "https://www.ti.com",
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    }
  ],
  "total": 5,
  "page": 1,
  "per_page": 30,
  "total_pages": 1
}
```

### Get Manufacturer

```http
GET /api/manufacturers/{id}
Authorization: Bearer <token>
```

#### Response

```json
{
  "id": 1,
  "name": "Microchip",
  "website": "https://www.microchip.com",
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-10T08:00:00Z"
}
```

---

## Suppliers

Suppliers represent vendors where components can be purchased.

### List Suppliers

```http
GET /api/suppliers
Authorization: Bearer <token>
```

#### Response

```json
{
  "items": [
    {
      "id": 1,
      "name": "DigiKey",
      "website": "https://www.digikey.com",
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    },
    {
      "id": 2,
      "name": "Mouser",
      "website": "https://www.mouser.com",
      "created_at": "2024-01-10T08:00:00Z",
      "updated_at": "2024-01-10T08:00:00Z"
    }
  ],
  "total": 3,
  "page": 1,
  "per_page": 30,
  "total_pages": 1
}
```

### Get Supplier

```http
GET /api/suppliers/{id}
Authorization: Bearer <token>
```

#### Response

```json
{
  "id": 1,
  "name": "DigiKey",
  "website": "https://www.digikey.com",
  "created_at": "2024-01-10T08:00:00Z",
  "updated_at": "2024-01-10T08:00:00Z"
}
```

---

## Users

Users represent accounts in the system.

### List Users

```http
GET /api/users
Authorization: Bearer <token>
```

#### Response

```json
{
  "items": [
    {
      "id": 1,
      "username": "admin",
      "email": "admin@example.com",
      "is_admin": true,
      "created_at": "2024-01-01T00:00:00Z",
      "updated_at": "2024-01-01T00:00:00Z"
    },
    {
      "id": 2,
      "username": "user",
      "email": "user@example.com",
      "is_admin": false,
      "created_at": "2024-01-05T10:00:00Z",
      "updated_at": "2024-01-05T10:00:00Z"
    }
  ],
  "total": 2,
  "page": 1,
  "per_page": 30,
  "total_pages": 1
}
```

### Get User

```http
GET /api/users/{id}
Authorization: Bearer <token>
```

#### Response

```json
{
  "id": 1,
  "username": "admin",
  "email": "admin@example.com",
  "is_admin": true,
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

**Note:** Password hashes are never included in API responses.
