# Agent Orchestration Plan: Migration Execution

This document provides the complete execution plan for coordinating specialized agents to complete the PHP → Rust + React migration.

**Created:** 2025-12-20  
**Status:** Ready for Execution

---

## Available Agents

1. **db-schema-agent** - Database schema validation and mapping
2. **rust-backend-agent** - Rust/Axum backend implementation
3. **api-compat-agent** - API compatibility testing and validation
4. **react-frontend-agent** - React/Next.js frontend with shadcn/ui
5. **main-agent** - Coordinator (this document's author)

---

## Execution Plan

### Phase 1: Backend API Compatibility (Week 1-2)

#### Task 1.1: Database Schema Validation (PREREQUISITE)
**Agent:** `db-schema-agent`  
**Priority:** CRITICAL - Must complete first  
**Duration:** 1 day  
**Can parallelize:** No

**Prompt:**
```
Analyze the PHP entities in src/Entity/Parts/ and validate that the Rust models in backend/src/models.rs accurately represent the database schema. Document any discrepancies and ensure all fields, types, and relationships are correctly mapped. Pay special attention to:
- Part, Category, Footprint, Manufacturer, StorageLocation, Supplier, User entities
- Field types (especially DateTime, nullable fields, foreign keys)
- Missing fields in Rust models
Create a schema validation report.
```

**Success Criteria:**
- ✅ Schema validation report created
- ✅ All discrepancies documented
- ✅ Rust models updated if needed

---

#### Task 1.2: Field Naming Convention
**Agent:** `rust-backend-agent`  
**Priority:** HIGH  
**Duration:** 1 day  
**Dependencies:** Task 1.1 must complete first  
**Can parallelize:** No (blocks Tasks 1.3 and 1.4)

**Prompt:**
```
Implement field naming convention compatibility in backend/src/models.rs:
1. Add #[serde(rename_all = "camelCase")] to all model structs (Part, Category, Footprint, Manufacturer, StorageLocation, Supplier, User, and all DTOs)
2. Ensure database queries still use snake_case field names
3. Test that JSON serialization outputs camelCase
4. Run all backend tests to verify no breakage
Commit with message: "feat: add camelCase field naming for API compatibility"
```

**Files to Modify:**
- `backend/src/models.rs`

**Success Criteria:**
- ✅ All structs have camelCase serialization
- ✅ Tests pass (25/25)
- ✅ Database queries still work
- ✅ JSON responses use camelCase

---

#### Task 1.3: Hydra/JSON-LD Format
**Agent:** `rust-backend-agent`  
**Priority:** CRITICAL  
**Duration:** 2-3 days  
**Dependencies:** Task 1.2 must complete first  
**Can parallelize:** Yes (with Task 1.4)

**Prompt:**
```
Implement Hydra/JSON-LD response format support:
1. Create backend/src/models/hydra.rs with HydraCollection, HydraResource, HydraView types
2. Create backend/src/middleware/content_negotiation.rs to check Accept header
3. Update route handlers in backend/src/routes.rs to support both formats:
   - application/ld+json → Hydra format
   - application/json → plain JSON (current)
4. Generate @id IRIs for all resources (@id: "/api/parts/1")
5. Add tests for both response formats
Commit with message: "feat: add Hydra/JSON-LD format with content negotiation"
```

**Files to Create:**
- `backend/src/models/hydra.rs`
- `backend/src/middleware/content_negotiation.rs`

**Files to Modify:**
- `backend/src/routes.rs`
- `backend/src/main.rs` (add middleware)

**Success Criteria:**
- ✅ Content negotiation middleware works
- ✅ Accept: application/ld+json returns Hydra format
- ✅ Accept: application/json returns plain JSON
- ✅ All resources have @id fields
- ✅ Tests pass

---

#### Task 1.4: IRI Generation for Relationships
**Agent:** `rust-backend-agent`  
**Priority:** HIGH  
**Duration:** 2-3 days  
**Dependencies:** Task 1.2 must complete first  
**Can parallelize:** Yes (with Task 1.3)

**Prompt:**
```
Add IRI (Internationalized Resource Identifier) support for relationships:
1. Add helper function in backend/src/models.rs: fn generate_iri(resource: &str, id: i32) -> String
2. Update model structs to include relationship IRIs (e.g., Part.category as "/api/categories/5")
3. Maintain integer IDs for internal use but serialize relationship fields as IRIs
4. Update all route handlers to populate IRI fields
5. Test with snapshot tests
Commit with message: "feat: add IRI generation for resource relationships"
```

**Files to Modify:**
- `backend/src/models.rs`
- `backend/src/routes.rs`

**Success Criteria:**
- ✅ Helper function implemented
- ✅ All relationships serialize as IRIs
- ✅ Integer IDs still work internally
- ✅ Tests pass

---

### Phase 2: Complete CRUD Operations (Week 2)

#### Task 2.1: Footprints CRUD
**Agent:** `rust-backend-agent`  
**Priority:** MEDIUM  
**Duration:** 1 day  
**Dependencies:** Tasks 1.3 and 1.4 must complete first  
**Can parallelize:** Yes (with Tasks 2.2, 2.3)

**Prompt:**
```
Implement full CRUD operations for Footprints in backend/src/routes.rs:
- POST /api/footprints (create_footprint handler)
- PATCH /api/footprints/{id} (update_footprint handler)
- DELETE /api/footprints/{id} (delete_footprint handler)
Add CreateFootprint and UpdateFootprint DTOs in models.rs
Add integration tests for all operations
Commit with message: "feat: implement Footprints CRUD operations"
```

**Files to Modify:**
- `backend/src/routes.rs`
- `backend/src/models.rs`
- `backend/src/main.rs` (add routes)

**Success Criteria:**
- ✅ POST endpoint works
- ✅ PATCH endpoint works
- ✅ DELETE endpoint works
- ✅ DTOs created
- ✅ Integration tests pass

---

#### Task 2.2: Manufacturers CRUD
**Agent:** `rust-backend-agent`  
**Priority:** MEDIUM  
**Duration:** 1 day  
**Dependencies:** Tasks 1.3 and 1.4 must complete first  
**Can parallelize:** Yes (with Tasks 2.1, 2.3)

**Prompt:**
```
Implement full CRUD operations for Manufacturers in backend/src/routes.rs:
- POST /api/manufacturers (create_manufacturer handler)
- PATCH /api/manufacturers/{id} (update_manufacturer handler)
- DELETE /api/manufacturers/{id} (delete_manufacturer handler)
Add CreateManufacturer and UpdateManufacturer DTOs in models.rs
Add integration tests
Commit with message: "feat: implement Manufacturers CRUD operations"
```

**Files to Modify:**
- `backend/src/routes.rs`
- `backend/src/models.rs`
- `backend/src/main.rs` (add routes)

**Success Criteria:**
- ✅ POST endpoint works
- ✅ PATCH endpoint works
- ✅ DELETE endpoint works
- ✅ DTOs created
- ✅ Integration tests pass

---

#### Task 2.3: Storage Locations & Suppliers CRUD
**Agent:** `rust-backend-agent`  
**Priority:** MEDIUM  
**Duration:** 1-2 days  
**Dependencies:** Tasks 1.3 and 1.4 must complete first  
**Can parallelize:** Yes (with Tasks 2.1, 2.2)

**Prompt:**
```
Implement full CRUD for StorageLocations and Suppliers:
StorageLocations: POST, PATCH, DELETE endpoints with DTOs
Suppliers: POST, PATCH, DELETE endpoints with DTOs
Add integration tests for both
Commit with message: "feat: implement StorageLocations and Suppliers CRUD"
```

**Files to Modify:**
- `backend/src/routes.rs`
- `backend/src/models.rs`
- `backend/src/main.rs` (add routes)

**Success Criteria:**
- ✅ All StorageLocation endpoints work
- ✅ All Supplier endpoints work
- ✅ DTOs created for both
- ✅ Integration tests pass

---

### Phase 3: API Compatibility Validation (After Phase 2)

#### Task 3.1: Snapshot Testing & Validation
**Agent:** `api-compat-agent`  
**Priority:** CRITICAL  
**Duration:** 1 day  
**Dependencies:** All Phase 2 tasks must complete first  
**Can parallelize:** No

**Prompt:**
```
Generate and compare API snapshots:
1. Ensure PHP backend is running on localhost:8000
2. Run tests/api_compat/generate_php_snapshots.sh
3. Ensure Rust backend is running on localhost:3000
4. Run tests/api_compat/generate_rust_snapshots.sh
5. Run tests/api_compat/compare_snapshots.sh
6. Document all differences in docs/src/api-parity-report.md
7. Create GitHub issues for any incompatibilities found
```

**Success Criteria:**
- ✅ PHP snapshots captured
- ✅ Rust snapshots captured
- ✅ Comparison report generated
- ✅ All differences documented
- ✅ Issues created for incompatibilities

---

### Phase 4: Frontend Migration (Week 3-6)

#### Task 4.1: Parts Management UI
**Agent:** `react-frontend-agent`  
**Priority:** HIGH  
**Duration:** 3-4 days  
**Dependencies:** Tasks 1.2, 1.3, 1.4 should complete first  
**Can parallelize:** Yes (can start after Task 1.2, parallel with Phase 2)

**Prompt:**
```
Implement Parts management UI using shadcn/ui:
1. Create frontend/src/app/parts/page.tsx with DataTable component
2. Add Create/Edit/Delete modals using Dialog component
3. Implement Zustand store actions in frontend/src/store/parts.ts
4. Add form validation with react-hook-form
5. Ensure visual parity with PHP Twig templates in templates/part/
6. Add loading states and error boundaries
Test all CRUD operations against Rust API
Commit with message: "feat: implement Parts management UI with shadcn/ui"
```

**Files to Create:**
- `frontend/src/app/parts/page.tsx`
- `frontend/src/components/parts/PartsTable.tsx`
- `frontend/src/components/parts/PartDialog.tsx`

**Files to Modify:**
- `frontend/src/store/parts.ts`
- `frontend/src/lib/api.ts`

**Success Criteria:**
- ✅ List view works
- ✅ Create modal works
- ✅ Edit modal works
- ✅ Delete confirmation works
- ✅ Visual parity achieved
- ✅ Error handling works

---

#### Task 4.2: Other Entity UIs
**Agent:** `react-frontend-agent`  
**Priority:** MEDIUM  
**Duration:** 4-5 days  
**Dependencies:** Task 4.1 should complete first (for reusable components)  
**Can parallelize:** Partially (after Task 4.1 creates reusable components)

**Prompt:**
```
Implement UI for Categories, Footprints, Manufacturers, StorageLocations, Suppliers:
1. Create pages for each entity following Parts pattern
2. Reuse DataTable and Modal components
3. Update Zustand stores
4. Ensure consistent UI/UX across all entity types
5. Add breadcrumbs and navigation
Commit with message: "feat: implement entity management UIs"
```

**Files to Create:**
- `frontend/src/app/categories/page.tsx`
- `frontend/src/app/footprints/page.tsx`
- `frontend/src/app/manufacturers/page.tsx`
- `frontend/src/app/storage-locations/page.tsx`
- `frontend/src/app/suppliers/page.tsx`

**Files to Modify:**
- `frontend/src/store/*.ts`
- `frontend/src/lib/api.ts`

**Success Criteria:**
- ✅ All entity pages work
- ✅ Consistent UI/UX
- ✅ Navigation works
- ✅ All CRUD operations functional

---

### Phase 5: Authentication (Week 2-3)

#### Task 5.1: Backend Authentication System
**Agent:** `rust-backend-agent`  
**Priority:** HIGH  
**Duration:** 4-5 days  
**Dependencies:** Phase 3 should complete first  
**Can parallelize:** Yes (with Task 5.2 after basic endpoints done)

**Prompt:**
```
Implement JWT-based authentication:
1. Complete backend/src/auth.rs with login/logout endpoints
2. Add POST /api/auth/login handler
3. Add POST /api/auth/logout handler  
4. Add GET /api/auth/me handler (current user)
5. Implement token refresh mechanism
6. Add permission checking in auth_middleware
7. Integrate with users table
8. Add comprehensive auth tests
Commit with message: "feat: implement JWT authentication system"
```

**Files to Modify:**
- `backend/src/auth.rs`
- `backend/src/routes.rs`
- `backend/src/main.rs`

**Success Criteria:**
- ✅ Login endpoint works
- ✅ Logout endpoint works
- ✅ Current user endpoint works
- ✅ Token refresh works
- ✅ Permission checking works
- ✅ Auth tests pass

---

#### Task 5.2: Frontend Authentication UI
**Agent:** `react-frontend-agent`  
**Priority:** HIGH  
**Duration:** 2-3 days  
**Dependencies:** Task 5.1 login endpoint must be ready  
**Can parallelize:** Partially (can start once login endpoint exists)

**Prompt:**
```
Implement authentication UI:
1. Create frontend/src/app/login/page.tsx with shadcn/ui Form
2. Update frontend/src/store/auth.ts with login/logout actions
3. Add JWT token storage and refresh logic
4. Implement protected route wrapper
5. Add user profile page
6. Test full authentication flow
Commit with message: "feat: implement authentication UI"
```

**Files to Create:**
- `frontend/src/app/login/page.tsx`
- `frontend/src/app/profile/page.tsx`
- `frontend/src/components/auth/ProtectedRoute.tsx`

**Files to Modify:**
- `frontend/src/store/auth.ts`
- `frontend/src/lib/api.ts`
- `frontend/src/app/layout.tsx`

**Success Criteria:**
- ✅ Login page works
- ✅ Token storage works
- ✅ Token refresh works
- ✅ Protected routes work
- ✅ Logout works
- ✅ Profile page works

---

## Execution Timeline

### Sequential Path (Single Agent)
- **Week 1:** Database Schema (1d) → Field Naming (1d) → Hydra Format (3d) → IRI (3d)
- **Week 2:** Footprints (1d) → Manufacturers (1d) → Storage/Suppliers (2d) → API Validation (1d)
- **Week 3-4:** Parts UI (4d) → Other UIs (5d)
- **Week 5:** Backend Auth (5d)
- **Week 6:** Frontend Auth (3d)
- **Total: 6 weeks**

### Parallel Path (Multiple Agents)
- **Week 1:**
  - Day 1: Database Schema (db-schema-agent)
  - Day 2: Field Naming (rust-backend-agent)
  - Days 3-5: Hydra Format + IRI (2 rust-backend-agent instances)
  
- **Week 2:**
  - Days 1-3: Footprints + Manufacturers + Storage/Suppliers (3 rust-backend-agent instances)
  - Day 4: API Validation (api-compat-agent)
  - Days 1-5: Parts UI starts (react-frontend-agent, parallel)

- **Week 3:**
  - Days 1-5: Other UIs (react-frontend-agent)
  - Days 1-5: Backend Auth starts (rust-backend-agent, parallel)

- **Week 4:**
  - Days 1-3: Frontend Auth (react-frontend-agent)
  - **Total: 3-4 weeks with parallelization**

---

## Parallelization Matrix

| Task | Can Start After | Can Run Parallel With |
|------|----------------|----------------------|
| 1.1 Database Schema | - | Nothing |
| 1.2 Field Naming | 1.1 | Nothing |
| 1.3 Hydra Format | 1.2 | 1.4 |
| 1.4 IRI Generation | 1.2 | 1.3 |
| 2.1 Footprints CRUD | 1.3, 1.4 | 2.2, 2.3 |
| 2.2 Manufacturers CRUD | 1.3, 1.4 | 2.1, 2.3 |
| 2.3 Storage/Suppliers CRUD | 1.3, 1.4 | 2.1, 2.2 |
| 3.1 API Validation | All Phase 2 | Nothing |
| 4.1 Parts UI | 1.2 | All Phase 2 |
| 4.2 Other UIs | 4.1 | Phase 5 |
| 5.1 Backend Auth | 3.1 | 4.2 |
| 5.2 Frontend Auth | 5.1 (partial) | Nothing |

---

## Critical Path

The critical path (tasks that block the most other tasks):

1. **Database Schema (1.1)** → Blocks everything
2. **Field Naming (1.2)** → Blocks all API work
3. **Hydra Format (1.3)** → Blocks CRUD operations
4. **IRI Generation (1.4)** → Blocks CRUD operations
5. **All CRUD (2.x)** → Blocks API validation
6. **API Validation (3.1)** → Blocks authentication
7. **Backend Auth (5.1)** → Blocks frontend auth

**Minimum Duration (Critical Path Only): ~2.5 weeks**

---

## Agent Prompts Summary

For quick copy-paste, here are all prompts in order:

### 1. Database Schema
```
Analyze the PHP entities in src/Entity/Parts/ and validate that the Rust models in backend/src/models.rs accurately represent the database schema. Document any discrepancies and ensure all fields, types, and relationships are correctly mapped. Pay special attention to: Part, Category, Footprint, Manufacturer, StorageLocation, Supplier, User entities. Field types (especially DateTime, nullable fields, foreign keys). Missing fields in Rust models. Create a schema validation report.
```

### 2. Field Naming
```
Implement field naming convention compatibility in backend/src/models.rs: Add #[serde(rename_all = "camelCase")] to all model structs. Ensure database queries still use snake_case. Test JSON serialization outputs camelCase. Run all backend tests. Commit: "feat: add camelCase field naming for API compatibility"
```

### 3. Hydra Format
```
Implement Hydra/JSON-LD response format support: Create backend/src/models/hydra.rs with types. Create backend/src/middleware/content_negotiation.rs. Update route handlers for both formats. Generate @id IRIs. Add tests. Commit: "feat: add Hydra/JSON-LD format with content negotiation"
```

### 4. IRI Generation
```
Add IRI support: Helper function in models.rs. Update structs for relationship IRIs. Maintain integer IDs internally. Update handlers. Test. Commit: "feat: add IRI generation for resource relationships"
```

### 5-7. CRUD Operations
```
Implement full CRUD for [Footprints/Manufacturers/StorageLocations/Suppliers]: POST, PATCH, DELETE endpoints. Add DTOs. Add tests. Commit: "feat: implement [Entity] CRUD operations"
```

### 8. API Validation
```
Generate snapshots: Run PHP and Rust snapshot scripts. Compare. Document differences. Create issues for incompatibilities.
```

### 9. Parts UI
```
Implement Parts UI with shadcn/ui: DataTable, modals, Zustand store, validation, visual parity. Test CRUD. Commit: "feat: implement Parts management UI with shadcn/ui"
```

### 10. Other UIs
```
Implement UIs for all entities: Reuse components. Update stores. Consistent UX. Navigation. Commit: "feat: implement entity management UIs"
```

### 11. Backend Auth
```
Implement JWT auth: Login/logout/me endpoints. Token refresh. Permission checking. Tests. Commit: "feat: implement JWT authentication system"
```

### 12. Frontend Auth
```
Implement auth UI: Login page. Token storage/refresh. Protected routes. Profile page. Test. Commit: "feat: implement authentication UI"
```

---

## Success Metrics

After all tasks complete:
- ✅ 100% API parity with PHP backend
- ✅ All 35+ endpoints functional
- ✅ Frontend feature parity achieved
- ✅ Authentication working end-to-end
- ✅ All tests passing
- ✅ Documentation updated
- ✅ Ready for legacy PHP removal

---

## Notes

- Each agent should commit frequently with conventional commit messages
- Run tests after each significant change
- Update documentation as work progresses
- Report blockers immediately
- Create issues for discovered problems
- Keep migration-progress.md updated

---

**Last Updated:** 2025-12-20  
**Document Version:** 1.0  
**Status:** Ready for Execution
