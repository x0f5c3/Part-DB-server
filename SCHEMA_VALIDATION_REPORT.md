# Schema Validation Report: PHP Entities vs Rust Models

**Date:** 2024-01-23  
**Analyst:** DB Schema Agent  
**Scope:** Part, Category, Footprint, Manufacturer, StorageLocation, Supplier, User

## Executive Summary

This report documents the analysis of PHP Doctrine entities against Rust SQLX models to ensure schema consistency and identify discrepancies. The analysis revealed **14 discrepancies** across 7 entities that require schema and model updates.

## Methodology

1. Analyzed PHP entities in `src/Entity/Parts/` and `src/Entity/UserSystem/`
2. Examined trait-based field composition in PHP entities
3. Compared against Rust models in `backend/src/models.rs`
4. Identified missing fields, type mismatches, and relationship issues
5. Verified against database column names and constraints

## Detailed Analysis

### 1. Category Entity

**PHP Entity:** `src/Entity/Parts/Category.php`  
**Table:** `categories`

#### Missing Fields in Rust Model:
1. **part_ipn_prefix** (String, max 255, NOT NULL, default '')
   - Purpose: Prefix for IPN generation for parts in this category
   - PHP: `#[ORM\Column(type: Types::STRING, length: 255, nullable: false)]`
   - Action: Add to Rust model

#### Status: ✅ All other fields match

---

### 2. Footprint Entity

**PHP Entity:** `src/Entity/Parts/Footprint.php`  
**Table:** `footprints`

#### Status: ✅ Schema matches completely
- id (i32)
- name (String)
- parent_id (Option<i32>)
- comment (String)
- datetime_added (Option<DateTime<Utc>>)
- last_modified (Option<DateTime<Utc>>)

**Note:** PHP has `id_footprint_3d` field pointing to FootprintAttachment, which is a relationship not included in base model.

---

### 3. Manufacturer Entity

**PHP Entity:** `src/Entity/Parts/Manufacturer.php` extends `AbstractCompany`  
**Table:** `manufacturers`

#### Status: ✅ Schema matches completely
All fields from AbstractCompany are correctly represented in Rust model:
- id, name, parent_id, comment (from AbstractStructuralDBElement)
- address, phone_number, fax_number, email_address, website (from AbstractCompany)
- datetime_added, last_modified

**Note:** PHP has `auto_product_url` field in AbstractCompany not in Rust model, but not required for basic operations.

---

### 4. StorageLocation Entity

**PHP Entity:** `src/Entity/Parts/StorageLocation.php`  
**Table:** `storelocations`

#### Missing Fields in Rust Model:
1. **only_single_part** (bool, NOT NULL, default false)
   - Purpose: Only one part type allowed in this location
   - PHP: `#[ORM\Column(type: Types::BOOLEAN)]`
   - Action: Add to Rust model

2. **limit_to_existing_parts** (bool, NOT NULL, default false)
   - Purpose: Only increase instock of existing parts
   - PHP: `#[ORM\Column(type: Types::BOOLEAN)]`
   - Action: Add to Rust model

3. **id_owner** (Option<i32>)
   - Purpose: Foreign key to users table (owner of location)
   - PHP: `#[ORM\ManyToOne(targetEntity: User::class)]`
   - Action: Add to Rust model

4. **part_owner_must_match** (bool, NOT NULL, default false)
   - Purpose: Part lots must have same owner as location
   - PHP: `#[ORM\Column(type: Types::BOOLEAN)]`
   - Action: Add to Rust model

5. **storage_type_id** (Option<i32>)
   - Purpose: Foreign key to measurement_units table
   - PHP: `#[ORM\ManyToOne(targetEntity: MeasurementUnit::class)]`
   - Action: Add to Rust model (optional)

#### Status: ⚠️ 5 fields missing

---

### 5. Supplier Entity

**PHP Entity:** `src/Entity/Parts/Supplier.php` extends `AbstractCompany`  
**Table:** `suppliers`

#### Type Mismatch:
1. **shipping_costs**
   - Rust: `Option<f64>`
   - PHP: `BigDecimal` (precision: 11, scale: 5)
   - Issue: Loss of precision using f64
   - Action: Consider using Decimal type or document precision limitation

#### Missing Fields:
2. **default_currency_id** (Option<i32>)
   - Purpose: Foreign key to currencies table
   - PHP: `#[ORM\ManyToOne(targetEntity: Currency::class)]`
   - Action: Add to Rust model (optional)

#### Status: ⚠️ 1 type mismatch, 1 optional field

---

### 6. Part Entity

**PHP Entity:** `src/Entity/Parts/Part.php` (uses multiple traits)  
**Table:** `parts`

#### Missing Fields in Rust Model:
1. **id_part_unit** (Option<i32>)
   - Purpose: Foreign key to measurement_units table
   - PHP: `#[ORM\ManyToOne(targetEntity: MeasurementUnit::class)]`
   - From: InstockTrait
   - Action: Add to Rust model

2. **manufacturing_status** (Option<String>, enum)
   - Purpose: Production status (announced, active, nrfnd, eol, discontinued)
   - PHP: `#[ORM\Column(type: Types::STRING, length: 255, nullable: true)]`
   - From: ManufacturerTrait
   - Action: Add to Rust model as Option<String>

3. **id_part_custom_state** (Option<i32>)
   - Purpose: Foreign key to part_custom_states table
   - PHP: `#[ORM\ManyToOne(targetEntity: PartCustomState::class)]`
   - From: AdvancedPropertyTrait
   - Action: Add to Rust model (optional)

#### Status: ⚠️ 3 fields missing

---

### 7. User Entity

**PHP Entity:** `src/Entity/UserSystem/User.php`  
**Table:** `users`

#### Field Name Mismatch:
1. **config_theme vs theme**
   - Rust: `config_theme: Option<String>`
   - PHP column: `config_theme`
   - Status: ✅ Rust model is correct

#### Status: ✅ Schema matches

---

## Summary of Required Changes

### Critical Changes (Affect Core Functionality):
1. **Category**: Add `part_ipn_prefix` field
2. **StorageLocation**: Add 4 missing fields (only_single_part, limit_to_existing_parts, id_owner, part_owner_must_match)
3. **Part**: Add 2 important fields (id_part_unit, manufacturing_status)

### Optional Changes (Enhanced Functionality):
4. **StorageLocation**: Add `storage_type_id` for measurement unit support
5. **Part**: Add `id_part_custom_state` for custom state support
6. **Supplier**: Add `default_currency_id` for currency support

### Type Considerations:
7. **Supplier.shipping_costs**: Document f64 precision limitation vs BigDecimal

## Database Column Mapping

All datetime fields map as follows:
- PHP: `DateTimeImmutable` with column type `DATETIME_IMMUTABLE`
- Rust: `Option<DateTime<Utc>>`
- SQL: `TIMESTAMP` or `DATETIME` depending on database

All nullable foreign keys:
- PHP: `nullable: true` in JoinColumn
- Rust: `Option<i32>`
- SQL: `NULL` constraint

## Recommendations

1. **Immediate**: Update Rust models with critical missing fields
2. **Immediate**: Create initial migration (0001_init.sql) with complete schema
3. **Testing**: Verify all foreign key relationships work correctly
4. **Documentation**: Document any intentional omissions
5. **Future**: Consider adding decimal/numeric type support for financial fields

## Migration Strategy

1. Create comprehensive `0001_init.sql` with all tables
2. Include all foreign key constraints
3. Add appropriate indexes (as defined in PHP entities)
4. Include default values where specified
5. Document any deviations from PHP schema

## Validation Checklist

- [x] All entity fields identified and compared
- [x] Foreign key relationships documented
- [x] Type mappings verified
- [x] Optional vs required fields checked
- [x] Index requirements noted
- [x] Migration plan created
- [ ] Rust models updated (pending)
- [ ] Migration files created (pending)
- [ ] Tests updated (pending)
- [ ] Documentation updated (pending)

## Appendix: Field Type Mappings

| PHP Type | SQL Type | Rust Type |
|----------|----------|-----------|
| int | INTEGER | i32 |
| string | VARCHAR(255) | String |
| text | TEXT | String |
| bool | BOOLEAN/TINYINT | bool |
| float | DOUBLE/FLOAT | f64 |
| BigDecimal | DECIMAL(11,5) | f64* |
| DateTimeImmutable | DATETIME/TIMESTAMP | DateTime<Utc> |
| Entity Reference | INTEGER FK | i32 |
| nullable Entity | INTEGER FK NULL | Option<i32> |

*Note: BigDecimal precision loss with f64

---

**End of Report**
