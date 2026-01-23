-- Initial schema migration for Part-DB Rust backend
-- Generated from PHP Doctrine entities analysis
-- Date: 2024-01-23

-- ============================================================================
-- Users Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(180) UNIQUE NOT NULL,
    first_name VARCHAR(255),
    last_name VARCHAR(255),
    email VARCHAR(255),
    password VARCHAR(255),
    disabled BOOLEAN NOT NULL DEFAULT 0,
    config_theme VARCHAR(255),
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS user_idx_username ON users(name);

-- ============================================================================
-- Categories Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    parent_id INTEGER,
    comment TEXT NOT NULL DEFAULT '',
    partname_hint TEXT NOT NULL DEFAULT '',
    partname_regex TEXT NOT NULL DEFAULT '',
    part_ipn_prefix VARCHAR(255) NOT NULL DEFAULT '',
    disable_footprints BOOLEAN NOT NULL DEFAULT 0,
    disable_manufacturers BOOLEAN NOT NULL DEFAULT 0,
    disable_autodatasheets BOOLEAN NOT NULL DEFAULT 0,
    disable_properties BOOLEAN NOT NULL DEFAULT 0,
    default_description TEXT NOT NULL DEFAULT '',
    default_comment TEXT NOT NULL DEFAULT '',
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES categories(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS category_idx_name ON categories(name);
CREATE INDEX IF NOT EXISTS category_idx_parent_name ON categories(parent_id, name);

-- ============================================================================
-- Footprints Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS footprints (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    parent_id INTEGER,
    comment TEXT NOT NULL DEFAULT '',
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES footprints(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS footprint_idx_name ON footprints(name);
CREATE INDEX IF NOT EXISTS footprint_idx_parent_name ON footprints(parent_id, name);

-- ============================================================================
-- Manufacturers Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS manufacturers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    parent_id INTEGER,
    comment TEXT NOT NULL DEFAULT '',
    address VARCHAR(255) NOT NULL DEFAULT '',
    phone_number VARCHAR(255) NOT NULL DEFAULT '',
    fax_number VARCHAR(255) NOT NULL DEFAULT '',
    email_address VARCHAR(255) NOT NULL DEFAULT '',
    website VARCHAR(255) NOT NULL DEFAULT '',
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES manufacturers(id) ON DELETE RESTRICT
);

CREATE INDEX IF NOT EXISTS manufacturer_name ON manufacturers(name);
CREATE INDEX IF NOT EXISTS manufacturer_idx_parent_name ON manufacturers(parent_id, name);

-- ============================================================================
-- Storage Locations Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS storelocations (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    parent_id INTEGER,
    comment TEXT NOT NULL DEFAULT '',
    is_full BOOLEAN NOT NULL DEFAULT 0,
    only_single_part BOOLEAN NOT NULL DEFAULT 0,
    limit_to_existing_parts BOOLEAN NOT NULL DEFAULT 0,
    id_owner INTEGER,
    part_owner_must_match BOOLEAN NOT NULL DEFAULT 0,
    storage_type_id INTEGER,
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES storelocations(id) ON DELETE RESTRICT,
    FOREIGN KEY (id_owner) REFERENCES users(id) ON DELETE SET NULL,
    FOREIGN KEY (storage_type_id) REFERENCES measurement_units(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS location_idx_name ON storelocations(name);
CREATE INDEX IF NOT EXISTS location_idx_parent_name ON storelocations(parent_id, name);

-- ============================================================================
-- Suppliers Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS suppliers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    parent_id INTEGER,
    comment TEXT NOT NULL DEFAULT '',
    address VARCHAR(255) NOT NULL DEFAULT '',
    phone_number VARCHAR(255) NOT NULL DEFAULT '',
    fax_number VARCHAR(255) NOT NULL DEFAULT '',
    email_address VARCHAR(255) NOT NULL DEFAULT '',
    website VARCHAR(255) NOT NULL DEFAULT '',
    shipping_costs REAL,
    default_currency_id INTEGER,
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (parent_id) REFERENCES suppliers(id) ON DELETE RESTRICT,
    FOREIGN KEY (default_currency_id) REFERENCES currencies(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS supplier_idx_name ON suppliers(name);
CREATE INDEX IF NOT EXISTS supplier_idx_parent_name ON suppliers(parent_id, name);

-- ============================================================================
-- Parts Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS parts (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    comment TEXT NOT NULL DEFAULT '',
    visible BOOLEAN NOT NULL DEFAULT 1,
    favorite BOOLEAN NOT NULL DEFAULT 0,
    id_category INTEGER NOT NULL,
    id_footprint INTEGER,
    id_manufacturer INTEGER,
    id_part_unit INTEGER,
    ipn VARCHAR(100) UNIQUE,
    mass REAL,
    tags TEXT NOT NULL DEFAULT '',
    manufacturer_product_number VARCHAR(255) NOT NULL DEFAULT '',
    manufacturer_product_url TEXT NOT NULL DEFAULT '',
    manufacturing_status VARCHAR(255),
    minamount REAL NOT NULL DEFAULT 0,
    needs_review BOOLEAN NOT NULL DEFAULT 0,
    id_part_custom_state INTEGER,
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_category) REFERENCES categories(id) ON DELETE RESTRICT,
    FOREIGN KEY (id_footprint) REFERENCES footprints(id) ON DELETE SET NULL,
    FOREIGN KEY (id_manufacturer) REFERENCES manufacturers(id) ON DELETE SET NULL,
    FOREIGN KEY (id_part_unit) REFERENCES measurement_units(id) ON DELETE SET NULL,
    FOREIGN KEY (id_part_custom_state) REFERENCES part_custom_states(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS parts_idx_name ON parts(name);
CREATE INDEX IF NOT EXISTS parts_idx_ipn ON parts(ipn);
CREATE INDEX IF NOT EXISTS parts_idx_datet_name_last_id_needs ON parts(
    datetime_added, name, last_modified, id, needs_review
);

-- ============================================================================
-- Part Lots Table
-- ============================================================================
CREATE TABLE IF NOT EXISTS part_lots (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    id_part INTEGER NOT NULL,
    id_storage_location INTEGER,
    description TEXT NOT NULL DEFAULT '',
    comment TEXT NOT NULL DEFAULT '',
    expiration_date DATETIME,
    amount REAL NOT NULL DEFAULT 0,
    instock_unknown BOOLEAN NOT NULL DEFAULT 0,
    needs_refill BOOLEAN NOT NULL DEFAULT 0,
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (id_part) REFERENCES parts(id) ON DELETE CASCADE,
    FOREIGN KEY (id_storage_location) REFERENCES storelocations(id) ON DELETE SET NULL
);

CREATE INDEX IF NOT EXISTS part_lots_idx_part ON part_lots(id_part);
CREATE INDEX IF NOT EXISTS part_lots_idx_storage ON part_lots(id_storage_location);

-- ============================================================================
-- Supporting Tables (Referenced by Foreign Keys)
-- ============================================================================

-- Measurement Units Table
CREATE TABLE IF NOT EXISTS measurement_units (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    unit VARCHAR(50) NOT NULL,
    is_integer BOOLEAN NOT NULL DEFAULT 0,
    use_si_prefix BOOLEAN NOT NULL DEFAULT 0,
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Currencies Table
CREATE TABLE IF NOT EXISTS currencies (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    iso_code VARCHAR(3) NOT NULL,
    exchange_rate REAL NOT NULL DEFAULT 1.0,
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Part Custom States Table
CREATE TABLE IF NOT EXISTS part_custom_states (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    datetime_added DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_modified DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- ============================================================================
-- Initial Data
-- ============================================================================

-- Insert default anonymous user (required by Part-DB)
INSERT OR IGNORE INTO users (id, name, disabled, datetime_added, last_modified)
VALUES (1, 'anonymous', 1, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- Insert default root category
INSERT OR IGNORE INTO categories (id, name, comment, datetime_added, last_modified)
VALUES (1, 'Root', 'Root category', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP);

-- ============================================================================
-- Notes and Documentation
-- ============================================================================

-- FIELD MAPPING NOTES:
-- 1. All datetime fields use DATETIME type (compatible with DateTimeImmutable in PHP)
-- 2. Boolean fields use BOOLEAN type (maps to TINYINT in MySQL/MariaDB, INTEGER in SQLite)
-- 3. Foreign keys use ON DELETE RESTRICT for structural elements (categories, etc.)
-- 4. Foreign keys use ON DELETE CASCADE for dependent data (part_lots)
-- 5. Foreign keys use ON DELETE SET NULL for optional relationships
-- 6. UNIQUE constraint on parts.ipn for internal part numbers
-- 7. Indexes match those defined in PHP Doctrine entities

-- TYPE PRECISION NOTES:
-- 1. shipping_costs uses REAL (f64) - note: lower precision than BigDecimal(11,5)
-- 2. Consider using DECIMAL type in production databases for financial data

-- OPTIONAL FIELDS (Not Critical for Base Operations):
-- 1. storelocations.storage_type_id - for measurement unit support
-- 2. parts.id_part_custom_state - for custom state support
-- 3. suppliers.default_currency_id - for multi-currency support
-- 4. parts.id_part_unit - for part measurement units

-- COMPATIBILITY:
-- This schema is compatible with SQLite, MySQL, MariaDB, and PostgreSQL
-- with minor adjustments for auto-increment syntax and data types.
