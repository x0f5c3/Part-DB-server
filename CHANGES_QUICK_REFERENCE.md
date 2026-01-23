# Quick Reference: Schema Changes

## Modified Rust Structs

### 1. Category
```rust
pub struct Category {
    // ... existing fields ...
    pub part_ipn_prefix: String,  // NEW: IPN prefix for parts
    // ... rest of fields ...
}
```

### 2. StorageLocation
```rust
pub struct StorageLocation {
    // ... existing fields ...
    pub only_single_part: bool,         // NEW: one part type restriction
    pub limit_to_existing_parts: bool,  // NEW: stock increase restriction
    pub id_owner: Option<i32>,          // NEW: owner user FK
    pub part_owner_must_match: bool,    // NEW: owner matching rule
    pub storage_type_id: Option<i32>,   // NEW: measurement unit FK
    // ... rest of fields ...
}
```

### 3. Supplier
```rust
pub struct Supplier {
    // ... existing fields ...
    pub default_currency_id: Option<i32>,  // NEW: default currency FK
    // ... rest of fields ...
}
```
**Note:** shipping_costs uses f64 (precision limitation vs BigDecimal)

### 4. Part
```rust
pub struct Part {
    // ... existing fields ...
    pub id_part_unit: Option<i32>,           // NEW: measurement unit FK
    pub manufacturing_status: Option<String>, // NEW: production status
    pub id_part_custom_state: Option<i32>,   // NEW: custom state FK
    // ... rest of fields ...
}
```

## Manufacturing Status Values

Valid enum values for `Part.manufacturing_status`:
- `"announced"` - Announced but not in production
- `"active"` - Currently in production
- `"nrfnd"` - Not recommended for new designs
- `"eol"` - End of life (discontinued soon)
- `"discontinued"` - No longer produced

## Database Migration

Apply with:
```bash
cd backend
sqlite3 partdb.db < migrations/0001_init.sql
# or
sqlx migrate run
```

## Testing

Run validation tests:
```bash
cd backend
cargo test --test schema_validation  # Schema-specific tests
cargo test                            # All tests
```

## Key Files

| File | Purpose |
|------|---------|
| `SCHEMA_VALIDATION_REPORT.md` | Detailed analysis report |
| `backend/DATABASE_SCHEMA.md` | Complete schema documentation |
| `backend/migrations/0001_init.sql` | Initial DB migration |
| `backend/tests/schema_validation.rs` | Schema validation tests |
| `backend/src/models.rs` | Updated Rust models |
| `TASK_COMPLETION_SUMMARY.md` | Executive summary |

## Foreign Key Relationships Added

```
parts.id_part_unit → measurement_units.id
parts.id_part_custom_state → part_custom_states.id
storelocations.id_owner → users.id
storelocations.storage_type_id → measurement_units.id
suppliers.default_currency_id → currencies.id
```

## Critical Business Rules

1. **Category**: `part_ipn_prefix` used for auto-generating internal part numbers
2. **StorageLocation**: 
   - `only_single_part=true` → only one part type allowed
   - `limit_to_existing_parts=true` → can only increase existing stock
   - `part_owner_must_match=true` → enforce owner matching
3. **Part**: `id_category` is NOT NULL (required field)
4. **User**: `password` field never serialized (security)

## Next Steps

1. Review changes in `backend/src/models.rs`
2. Apply migration: `backend/migrations/0001_init.sql`
3. Run tests: `cargo test`
4. Update any dependent code that uses these structs
5. Consider implementing decimal type for financial fields

---
Generated: 2024-01-23
