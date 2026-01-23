# Next Steps: API Compatibility Implementation

This document outlines the concrete implementation steps needed to achieve 100% API compatibility between the PHP and Rust backends.

Last Updated: 2025-12-06

## Critical Path Items

### 1. Response Format Compatibility (CRITICAL - Week 1)

**Problem**: PHP uses Hydra/JSON-LD format, Rust uses plain JSON

**Solution**: Implement content negotiation middleware

**Implementation Steps**:

1. Create Hydra response types in `backend/src/models/hydra.rs`:
   ```rust
   pub struct HydraCollection<T> {
       #[serde(rename = "@context")]
       context: String,
       #[serde(rename = "@id")]
       id: String,
       #[serde(rename = "@type")]
       type_: String,
       #[serde(rename = "hydra:member")]
       member: Vec<HydraResource<T>>,
       #[serde(rename = "hydra:totalItems")]
       total_items: i64,
       #[serde(rename = "hydra:view")]
       view: Option<HydraView>,
   }
   ```

2. Create content negotiation middleware in `backend/src/middleware/content_negotiation.rs`:
   - Check `Accept` header
   - If `application/ld+json` → return Hydra format
   - If `application/json` → return plain JSON (current behavior)

3. Update route handlers to use wrapper type that supports both formats

**Files to Modify**:
- `backend/src/models.rs` (add Hydra types)
- `backend/src/main.rs` (add middleware)
- `backend/src/routes.rs` (update return types)

**Testing**:
- Generate snapshots with both Accept headers
- Verify PHP clients work with Hydra responses
- Verify new clients work with JSON responses

**Estimated Effort**: 2-3 days

---

### 2. Field Naming Convention (HIGH - Week 1)

**Problem**: PHP uses camelCase, Rust uses snake_case

**Solution**: Add serde rename attributes

**Implementation Steps**:

1. Update all model structs with `#[serde(rename = "camelCase")]`:
   ```rust
   #[derive(Serialize, Deserialize)]
   pub struct Part {
       pub id: i32,
       pub name: String,
       #[serde(rename = "idCategory")]
       pub id_category: i32,
       #[serde(rename = "idFootprint")]
       pub id_footprint: Option<i32>,
       // ... etc
   }
   ```

2. Alternatively, use serde's `rename_all` attribute:
   ```rust
   #[derive(Serialize, Deserialize)]
   #[serde(rename_all = "camelCase")]
   pub struct Part {
       pub id: i32,
       pub name: String,
       pub id_category: i32,
       pub id_footprint: Option<i32>,
   }
   ```

**Files to Modify**:
- `backend/src/models.rs` (all struct definitions)

**Testing**:
- Run snapshot tests
- Verify field names match PHP exactly

**Estimated Effort**: 1 day

---

### 3. IRI Generation for Relationships (HIGH - Week 1-2)

**Problem**: PHP uses IRIs (`/api/categories/5`), Rust uses IDs (`5`)

**Solution**: Add IRI fields to models

**Implementation Steps**:

1. Create IRI generation helpers:
   ```rust
   fn generate_iri(resource: &str, id: i32) -> String {
       format!("/api/{}/{}", resource, id)
   }
   ```

2. Update model structs to include both ID and IRI:
   ```rust
   #[derive(Serialize)]
   pub struct Part {
       #[serde(rename = "@id")]
       pub iri: String,
       pub id: i32,
       pub category: String,  // IRI
       #[serde(skip_serializing)]
       pub id_category: i32,  // Used internally
   }
   ```

3. Update database queries to populate IRIs

**Files to Modify**:
- `backend/src/models.rs` (add IRI fields)
- `backend/src/routes.rs` (generate IRIs in queries)

**Testing**:
- Verify IRI format matches PHP
- Test relationship navigation

**Estimated Effort**: 2-3 days

---

### 4. Complete CRUD Operations (MEDIUM - Week 2)

**Problem**: Several entities are read-only

**Solution**: Implement POST/PATCH/DELETE for all entities

**Implementation Steps**:

1. Implement CRUD for Footprints:
   - POST `/api/footprints`
   - PATCH `/api/footprints/{id}`
   - DELETE `/api/footprints/{id}`

2. Repeat for Manufacturers, StorageLocations, Suppliers

3. Add validation matching PHP rules

**Files to Modify**:
- `backend/src/routes.rs` (add handlers)
- `backend/src/models.rs` (add DTOs)

**Testing**:
- Write integration tests for each endpoint
- Verify against PHP behavior

**Estimated Effort**: 3-4 days

---

### 5. Authentication & Authorization (HIGH - Week 2-3)

**Problem**: No authentication endpoints, all routes unprotected

**Solution**: Implement JWT-based auth

**Implementation Steps**:

1. Implement login endpoint:
   ```rust
   POST /api/auth/login
   {
       "username": "admin",
       "password": "password"
   }
   Response: { "token": "jwt..." }
   ```

2. Implement token refresh endpoint

3. Add permission checking in middleware

4. Integrate with existing User table

**Files to Modify**:
- `backend/src/auth.rs` (complete implementation)
- `backend/src/routes.rs` (add auth routes)
- `backend/src/main.rs` (configure auth middleware)

**Testing**:
- Test login flow
- Test unauthorized access
- Test token expiration

**Estimated Effort**: 4-5 days

---

### 6. Error Response Format (MEDIUM - Week 3)

**Problem**: Error responses may not match PHP format

**Solution**: Implement Hydra error format

**Implementation Steps**:

1. Create Hydra error response type:
   ```rust
   #[derive(Serialize)]
   struct HydraError {
       #[serde(rename = "@context")]
       context: String,
       #[serde(rename = "@type")]
       type_: String,
       #[serde(rename = "hydra:title")]
       title: String,
       #[serde(rename = "hydra:description")]
       description: String,
   }
   ```

2. Update error handling middleware

3. Match HTTP status codes with PHP

**Files to Modify**:
- `backend/src/error.rs` (add Hydra errors)

**Testing**:
- Test various error scenarios
- Compare with PHP error responses

**Estimated Effort**: 1-2 days

---

## Implementation Order (Recommended)

**Week 1** (High Priority):
1. Field naming convention (1 day)
2. Response format compatibility (2-3 days)
3. IRI generation (2-3 days)

**Week 2** (Medium Priority):
1. Complete CRUD operations (3-4 days)
2. Start authentication (2 days)

**Week 3** (Complete Auth & Polish):
1. Finish authentication (2-3 days)
2. Error response format (1-2 days)
3. Integration testing (2 days)

**Week 4** (Testing & Verification):
1. Run full snapshot test suite
2. Fix discovered issues
3. Performance testing
4. Documentation updates

---

## Testing Strategy

### For Each Implementation:

1. **Unit Tests**: Test individual functions
2. **Integration Tests**: Test full request/response cycle
3. **Snapshot Tests**: Compare with PHP API
4. **Manual Testing**: Test with actual PHP clients

### Continuous Validation:

```bash
# After each change:
cd backend
cargo test

# Generate snapshots:
cd ../tests/api_compat
./generate_rust_snapshots.sh
./compare_snapshots.sh

# Review differences in comparison_report.txt
```

---

## Success Criteria

For each completed item, all of the following must be true:

- ✅ Implementation complete and tested
- ✅ Unit tests passing
- ✅ Integration tests passing
- ✅ Snapshot tests passing (or differences documented)
- ✅ Code reviewed
- ✅ Documentation updated
- ✅ Committed with conventional commit message

---

## Risk Mitigation

### High Risk Items:

1. **Breaking existing clients**: Mitigation - support both formats via content negotiation
2. **Database schema changes**: Mitigation - coordinate with PHP version
3. **Performance degradation**: Mitigation - benchmark before and after

### Rollback Plan:

If implementation causes issues:
1. Revert commits
2. Disable new middleware
3. Fall back to current plain JSON format

---

## Resource Requirements

**Development Time**: ~3-4 weeks (1 developer)

**Infrastructure**:
- Development database (shared with PHP)
- Testing environment
- CI/CD pipeline

**Tools**:
- Rust toolchain
- PostgreSQL client
- jq (for snapshot comparison)
- HTTP client (curl/Postman)

---

## Documentation to Update

After implementation:

- [ ] API Parity Report - Mark items as compatible
- [ ] Migration Status - Update progress
- [ ] Backend README - Document new features
- [ ] OpenAPI spec - Add new endpoints
- [ ] Deployment guide - Update configuration

---

## Questions to Resolve

1. **Should we maintain 100% Hydra compatibility or only key features?**
   - Recommendation: Key features only (member, totalItems, view)

2. **How to handle deprecated PHP endpoints?**
   - Recommendation: Implement but mark as deprecated

3. **What about new features not in PHP?**
   - Recommendation: Add under `/api/v2` prefix

4. **Should we support XML/other formats?**
   - Recommendation: No, JSON only for now

---

## Contact & Support

For questions or blockers:
- Review [Migration Status](../docs/src/migration-status.md)
- Check [API Parity Report](../docs/src/api-parity-report.md)
- See existing tests in `backend/tests/`
