# Database Models

Part-DB uses SQLX for type-safe database access. This page documents the data models and their relationships.

## Entity-Relationship Overview

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Category  │────<│    Part     │>────│  Footprint  │
└─────────────┘     └──────┬──────┘     └─────────────┘
                          │ │
                          │ └──────────┐
                          │            │
                   ┌──────▼──────┐  ┌──▼────────────┐
                   │Manufacturer │  │StorageLocation│
                   └─────────────┘  └───────────────┘
```

## Core Models

### Part

The central entity representing an electronic component.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Part {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub footprint_id: Option<i32>,
    pub manufacturer_id: Option<i32>,
    pub storage_location_id: Option<i32>,
    pub quantity: i32,
    pub minimum_quantity: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Category

Hierarchical organization for parts.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Footprint

Physical package type of a component.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Footprint {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Manufacturer

Company that produces the component.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Manufacturer {
    pub id: i32,
    pub name: String,
    pub website: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### Storage Location

Physical location where parts are stored.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct StorageLocation {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### User

Application user for authentication.

```rust
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub email: Option<String>,
    pub is_admin: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Data Transfer Objects (DTOs)

DTOs are used for API requests and responses.

### Create DTOs

```rust
#[derive(Debug, Deserialize)]
pub struct CreatePart {
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub footprint_id: Option<i32>,
    pub manufacturer_id: Option<i32>,
    pub storage_location_id: Option<i32>,
    pub quantity: Option<i32>,
    pub minimum_quantity: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct CreateCategory {
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<i32>,
}
```

### Update DTOs

```rust
#[derive(Debug, Deserialize)]
pub struct UpdatePart {
    pub name: Option<String>,
    pub description: Option<String>,
    pub category_id: Option<i32>,
    pub footprint_id: Option<i32>,
    pub manufacturer_id: Option<i32>,
    pub storage_location_id: Option<i32>,
    pub quantity: Option<i32>,
    pub minimum_quantity: Option<i32>,
}
```

### Pagination

```rust
#[derive(Debug, Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub per_page: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: i32,
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
}
```

## Database Queries

SQLX provides compile-time checked SQL queries:

```rust
// Fetch a single part
let part = sqlx::query_as!(
    Part,
    r#"SELECT * FROM parts WHERE id = $1"#,
    id
)
.fetch_optional(&pool)
.await?;

// Fetch all parts with pagination
let parts = sqlx::query_as!(
    Part,
    r#"SELECT * FROM parts ORDER BY id LIMIT $1 OFFSET $2"#,
    per_page,
    offset
)
.fetch_all(&pool)
.await?;

// Insert a new part
let result = sqlx::query!(
    r#"INSERT INTO parts (name, description, category_id) 
       VALUES ($1, $2, $3) 
       RETURNING id"#,
    create.name,
    create.description,
    create.category_id
)
.fetch_one(&pool)
.await?;
```

## Database Schema

The database schema is defined in migrations. Here's a simplified example:

```sql
CREATE TABLE categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    parent_id INTEGER REFERENCES categories(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE parts (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    category_id INTEGER REFERENCES categories(id),
    footprint_id INTEGER REFERENCES footprints(id),
    manufacturer_id INTEGER REFERENCES manufacturers(id),
    storage_location_id INTEGER REFERENCES storage_locations(id),
    quantity INTEGER DEFAULT 0,
    minimum_quantity INTEGER DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);
```

## Type Safety

SQLX ensures type safety at compile time by checking SQL queries against the database schema. This prevents:

- Column name typos
- Type mismatches
- Missing required fields
- Invalid SQL syntax
