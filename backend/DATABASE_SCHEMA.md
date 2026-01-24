# Database Schema Documentation

## Overview

This document describes the database schema for the Part-DB Rust backend. The schema is derived from the PHP Doctrine entities and adapted for use with SQLX.

## Entity Relationship Diagram (ERD)

```
┌─────────────────┐
│     Users       │
│─────────────────│
│ id              │◄────────┐
│ name            │         │
│ first_name      │         │
│ last_modified   │         │
└─────────────────┘         │
                            │
┌─────────────────┐         │
│  Categories     │         │
│─────────────────│         │
│ id              │◄────┐   │
│ name            │     │   │
│ parent_id       │─────┘   │
│ part_ipn_prefix │         │
└─────────────────┘         │
        ▲                   │
        │                   │
        │                   │
┌─────────────────┐         │
│     Parts       │         │
│─────────────────│         │
│ id              │         │
│ name            │         │
│ id_category     │─────────┘
│ id_footprint    │─────────┐
│ id_manufacturer │─────┐   │
│ id_part_unit    │     │   │
│ manufacturing_  │     │   │
│   status        │     │   │
└─────────────────┘     │   │
        ▲               │   │
        │               │   │
┌─────────────────┐     │   │
│   Part Lots     │     │   │
│─────────────────│     │   │
│ id              │     │   │
│ id_part         │─────┘   │
│ id_storage_     │         │
│   location      │─────┐   │
│ amount          │     │   │
└─────────────────┘     │   │
                        │   │
┌─────────────────┐     │   │
│Storage Locations│◄────┘   │
│─────────────────│         │
│ id              │         │
│ name            │         │
│ parent_id       │─────┐   │
│ id_owner        │─────┼───┘
│ storage_type_id │     │
└─────────────────┘     │
                        │
┌─────────────────┐     │
│   Footprints    │◄────┘
│─────────────────│
│ id              │
│ name            │
│ parent_id       │
└─────────────────┘

┌─────────────────┐
│ Manufacturers   │
│─────────────────│
│ id              │
│ name            │
│ parent_id       │
│ address         │
│ phone_number    │
└─────────────────┘

┌─────────────────┐
│   Suppliers     │
│─────────────────│
│ id              │
│ name            │
│ parent_id       │
│ shipping_costs  │
│ default_        │
│   currency_id   │
└─────────────────┘
```

## Table Descriptions

### Core Tables

#### users
User accounts with authentication and preferences.

**Key Fields:**
- `id`: Primary key
- `name`: Username (unique, max 180 chars)
- `password`: Hashed password
- `disabled`: Account status
- `config_theme`: UI theme preference

**Constraints:**
- UNIQUE on `name`

---

#### categories
Hierarchical organization of parts by function.

**Key Fields:**
- `id`: Primary key
- `name`: Category name
- `parent_id`: Self-reference for hierarchy
- `part_ipn_prefix`: Prefix for auto-generated IPNs
- `partname_regex`: Validation regex for part names
- `disable_*`: Feature flags for parts in category

**Constraints:**
- FOREIGN KEY `parent_id` → `categories(id)`

**Indexes:**
- `category_idx_name` on (name)
- `category_idx_parent_name` on (parent_id, name)

---

#### footprints
Physical package/footprint specifications for parts.

**Key Fields:**
- `id`: Primary key
- `name`: Footprint name (e.g., "DIP8", "SMD0805")
- `parent_id`: Self-reference for hierarchy

**Constraints:**
- FOREIGN KEY `parent_id` → `footprints(id)`

**Indexes:**
- `footprint_idx_name` on (name)
- `footprint_idx_parent_name` on (parent_id, name)

---

#### manufacturers
Companies that produce parts.

**Key Fields:**
- `id`: Primary key
- `name`: Company name
- `parent_id`: Self-reference for hierarchy
- `address`, `phone_number`, `email_address`, `website`: Contact info

**Constraints:**
- FOREIGN KEY `parent_id` → `manufacturers(id)`

**Indexes:**
- `manufacturer_name` on (name)
- `manufacturer_idx_parent_name` on (parent_id, name)

---

#### storelocations
Physical or logical storage locations for parts.

**Key Fields:**
- `id`: Primary key
- `name`: Location name
- `parent_id`: Self-reference for hierarchy
- `is_full`: No more space available
- `only_single_part`: Only one part type allowed
- `limit_to_existing_parts`: Only increase existing stock
- `id_owner`: User who owns this location
- `part_owner_must_match`: Enforce owner matching
- `storage_type_id`: Measurement unit for this location

**Constraints:**
- FOREIGN KEY `parent_id` → `storelocations(id)`
- FOREIGN KEY `id_owner` → `users(id)`
- FOREIGN KEY `storage_type_id` → `measurement_units(id)`

**Indexes:**
- `location_idx_name` on (name)
- `location_idx_parent_name` on (parent_id, name)

**Business Rules:**
1. If `only_single_part` is true, only one part type can be stored
2. If `limit_to_existing_parts` is true, can only increase existing stock
3. If `part_owner_must_match` is true, part lot owner must match location owner

---

#### suppliers
Companies that sell parts.

**Key Fields:**
- `id`: Primary key
- `name`: Company name
- `parent_id`: Self-reference for hierarchy
- `shipping_costs`: Default shipping cost (f64, note precision)
- `default_currency_id`: Default currency for orders
- Contact fields: address, phone, email, website

**Constraints:**
- FOREIGN KEY `parent_id` → `suppliers(id)`
- FOREIGN KEY `default_currency_id` → `currencies(id)`

**Indexes:**
- `supplier_idx_name` on (name)
- `supplier_idx_parent_name` on (parent_id, name)

**Note:** `shipping_costs` uses f64 (lower precision than BigDecimal in PHP)

---

#### parts
Electronic parts/components in inventory.

**Key Fields:**
- `id`: Primary key
- `name`: Part name
- `description`: What the part does
- `ipn`: Internal Part Number (unique)
- `id_category`: Category (required)
- `id_footprint`: Physical footprint (optional)
- `id_manufacturer`: Manufacturer (optional)
- `id_part_unit`: Measurement unit (optional)
- `manufacturing_status`: Production status enum
- `minamount`: Minimum stock level
- `tags`: Comma-separated tags
- `needs_review`: Flagged for review

**Constraints:**
- FOREIGN KEY `id_category` → `categories(id)` NOT NULL
- FOREIGN KEY `id_footprint` → `footprints(id)`
- FOREIGN KEY `id_manufacturer` → `manufacturers(id)`
- FOREIGN KEY `id_part_unit` → `measurement_units(id)`
- FOREIGN KEY `id_part_custom_state` → `part_custom_states(id)`
- UNIQUE on `ipn`

**Indexes:**
- `parts_idx_name` on (name)
- `parts_idx_ipn` on (ipn)
- `parts_idx_datet_name_last_id_needs` on (datetime_added, name, last_modified, id, needs_review)

**Manufacturing Status Values:**
- `announced`: Announced but not in production
- `active`: Currently in production
- `nrfnd`: Not recommended for new designs
- `eol`: End of life (discontinued soon)
- `discontinued`: No longer produced

---

#### part_lots
Specific lots/batches of parts at storage locations.

**Key Fields:**
- `id`: Primary key
- `id_part`: Reference to part
- `id_storage_location`: Where stored
- `amount`: Quantity in stock
- `instock_unknown`: Amount is unknown
- `needs_refill`: Needs restocking
- `expiration_date`: When lot expires

**Constraints:**
- FOREIGN KEY `id_part` → `parts(id)` ON DELETE CASCADE
- FOREIGN KEY `id_storage_location` → `storelocations(id)`

**Indexes:**
- `part_lots_idx_part` on (id_part)
- `part_lots_idx_storage` on (id_storage_location)

---

### Supporting Tables

#### measurement_units
Units for measuring part quantities (pcs, meters, grams, etc.)

**Key Fields:**
- `id`: Primary key
- `name`: Unit name
- `unit`: Unit symbol
- `is_integer`: Whether only integer amounts allowed
- `use_si_prefix`: Allow SI prefixes (k, M, etc.)

---

#### currencies
Currencies for pricing information.

**Key Fields:**
- `id`: Primary key
- `name`: Currency name
- `iso_code`: ISO 4217 code (USD, EUR, etc.)
- `exchange_rate`: Rate relative to base currency

---

#### part_custom_states
Custom workflow states for parts.

**Key Fields:**
- `id`: Primary key
- `name`: State name
- `description`: State description

---

## Common Patterns

### Hierarchical Structures
Many entities support hierarchical organization via `parent_id`:
- Categories
- Footprints
- Manufacturers
- Storage Locations
- Suppliers

Root elements have `parent_id = NULL`.

### Timestamps
All entities have:
- `datetime_added`: Creation timestamp
- `last_modified`: Last update timestamp

Both default to `CURRENT_TIMESTAMP`.

### Soft References
Optional foreign keys (with ON DELETE SET NULL):
- Part → Footprint
- Part → Manufacturer
- Part → Part Unit
- Storage Location → Owner
- Supplier → Currency

### Hard References
Required foreign keys (with ON DELETE RESTRICT):
- Part → Category (required)
- Hierarchical parent relationships

### Cascade Deletes
- PartLots → Part (ON DELETE CASCADE)
- Deleting a part automatically deletes all its lots

---

## Migration Notes

### Initial Migration (0001_init.sql)
Creates all tables with proper indexes and foreign keys.

**Included:**
1. All core tables
2. Supporting tables
3. Indexes for performance
4. Foreign key constraints
5. Default data (anonymous user, root category)

### Data Types
- SQLite: Uses INTEGER, REAL, TEXT, BLOB
- MySQL/MariaDB: Uses INT, DOUBLE, TEXT, VARCHAR
- PostgreSQL: Uses INTEGER, DOUBLE PRECISION, TEXT, VARCHAR

### Compatibility
Schema is designed to work with:
- SQLite (development)
- MySQL/MariaDB (production)
- PostgreSQL (production)

Minor adjustments needed for:
- AUTO_INCREMENT vs AUTOINCREMENT vs SERIAL
- BOOLEAN vs TINYINT(1) vs SMALLINT
- DATETIME vs TIMESTAMP

---

## Performance Considerations

### Indexes
Critical indexes for query performance:
1. Name fields (for searching)
2. Foreign keys (for joins)
3. Composite index on parts (for common queries)

### Query Patterns
Optimized for:
1. Hierarchical queries (category trees, location trees)
2. Part searches by name, IPN, tags
3. Stock level queries
4. Part filtering by category/manufacturer/footprint

---

## Security Considerations

### User Data
- Passwords must be hashed (never stored plain)
- Password field marked as `#[serde(skip_serializing)]` in Rust

### Access Control
- User ownership tracked for storage locations
- `part_owner_must_match` enforces ownership rules

### Data Integrity
- Foreign key constraints prevent orphaned records
- Cascade deletes for dependent data
- Restrict deletes for structural elements

---

## Future Enhancements

### Potential Additions
1. Attachments (files, images)
2. Parameters (custom key-value pairs)
3. Order details and pricing
4. Project BOMs
5. Part associations
6. EDA integration data

### Type Improvements
1. Use Decimal type for financial data
2. Add enum types for status fields
3. Add JSON fields for flexible metadata

---

## References

- PHP Entities: `src/Entity/Parts/`, `src/Entity/UserSystem/`
- Rust Models: `backend/src/models.rs`
- Migration: `backend/migrations/0001_init.sql`
- Schema Report: `SCHEMA_VALIDATION_REPORT.md`
