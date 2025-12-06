# API Parity Report

This document tracks the compatibility between the legacy PHP API and the new Rust API implementation.

Last Updated: 2025-12-06

## Overview

The goal is to achieve **100% API compatibility** between the PHP (Symfony + API Platform) and Rust (Axum) implementations. This means:

- Identical URL paths and HTTP methods
- Identical JSON response structure
- Identical HTTP status codes
- Identical error message formats
- Identical query parameter handling
- Identical request body formats

## Testing Methodology

### Snapshot Testing

We use snapshot-based testing to ensure API parity:

1. Capture responses from PHP API endpoints
2. Compare with responses from equivalent Rust API endpoints
3. Verify field names, types, and structure match exactly
4. Document any intentional deviations

### Test Coverage

| Category | PHP Endpoints | Rust Endpoints | Coverage |
|----------|--------------|----------------|----------|
| Parts | 5 | 5 | 100% |
| Categories | 5 | 5 | 100% |
| Footprints | 5 | 2 | 40% |
| Manufacturers | 5 | 2 | 40% |
| Storage Locations | 5 | 2 | 40% |
| Suppliers | 5 | 2 | 40% |
| Users | 5 | 2 | 40% |
| Authentication | 4 | 0 | 0% |
| Attachments | 5 | 0 | 0% |
| Projects | 5 | 0 | 0% |
| **Total** | **TBD** | **20** | **~15%** |

## Endpoint Comparison

### Parts API

#### `GET /api/parts`

**Status**: ✅ **IMPLEMENTED & TESTED**

**PHP Response Structure**:
```json
{
  "hydra:member": [
    {
      "@id": "/api/parts/1",
      "@type": "Part",
      "id": 1,
      "name": "Resistor 100Ω",
      "description": "1/4W Carbon Film Resistor",
      "category": "/api/categories/5",
      "...": "..."
    }
  ],
  "hydra:totalItems": 150,
  "hydra:view": {
    "@id": "/api/parts?page=1",
    "@type": "hydra:PartialCollectionView",
    "hydra:first": "/api/parts?page=1",
    "hydra:last": "/api/parts?page=5",
    "hydra:next": "/api/parts?page=2"
  }
}
```

**Rust Response Structure**:
```json
{
  "items": [
    {
      "id": 1,
      "name": "Resistor 100Ω",
      "description": "1/4W Carbon Film Resistor",
      "id_category": 5,
      "...": "..."
    }
  ],
  "total": 150,
  "page": 1,
  "per_page": 30,
  "total_pages": 5
}
```

**Differences**:
- ⚠️ **Response format**: PHP uses Hydra format, Rust uses simpler pagination
- ⚠️ **Field names**: PHP uses `category` (IRI), Rust uses `id_category` (integer)
- ⚠️ **Metadata**: PHP includes `@id`, `@type`, Rust does not

**Compatibility Status**: ⚠️ **BREAKING CHANGES IDENTIFIED**

**Action Required**: 
- Option 1: Make Rust response match PHP Hydra format exactly
- Option 2: Update clients to handle both formats during transition
- Option 3: Add API version parameter to support both formats

---

#### `GET /api/parts/{id}`

**Status**: ✅ **IMPLEMENTED**

**PHP Response**:
```json
{
  "@context": "/api/contexts/Part",
  "@id": "/api/parts/1",
  "@type": "Part",
  "id": 1,
  "name": "Resistor 100Ω",
  "description": "1/4W Carbon Film Resistor",
  "comment": "",
  "visible": true,
  "favorite": false,
  "category": "/api/categories/5",
  "footprint": "/api/footprints/12",
  "manufacturer": null,
  "ipn": "R-100-0.25W",
  "tags": "passive,resistor",
  "manufacturerProductNumber": "",
  "manufacturerProductUrl": "",
  "minamount": 10.0,
  "needsReview": false,
  "createdAt": "2023-01-15T10:30:00+00:00",
  "lastModified": "2023-06-20T14:22:00+00:00",
  "partLots": ["/api/part_lots/1", "/api/part_lots/2"],
  "attachments": []
}
```

**Rust Response**:
```json
{
  "id": 1,
  "name": "Resistor 100Ω",
  "description": "1/4W Carbon Film Resistor",
  "comment": "",
  "visible": true,
  "favorite": false,
  "id_category": 5,
  "id_footprint": 12,
  "id_manufacturer": null,
  "ipn": "R-100-0.25W",
  "mass": null,
  "tags": "passive,resistor",
  "manufacturer_product_number": "",
  "manufacturer_product_url": "",
  "minamount": 10.0,
  "needs_review": false,
  "datetime_added": "2023-01-15T10:30:00Z",
  "last_modified": "2023-06-20T14:22:00Z"
}

```

**Differences**:
- ⚠️ **JSON-LD context**: PHP includes `@context`, `@id`, `@type`
- ⚠️ **Field naming**: PHP uses camelCase, Rust uses snake_case
- ⚠️ **Relationships**: PHP includes IRIs to related resources, Rust uses foreign key IDs
- ⚠️ **Missing fields**: Rust doesn't include `partLots`, `attachments` arrays

**Compatibility Status**: ❌ **INCOMPATIBLE**

---

#### `POST /api/parts`

**Status**: ✅ **IMPLEMENTED**

**PHP Request**:
```json
{
  "name": "Capacitor 100µF",
  "description": "Electrolytic capacitor",
  "category": "/api/categories/3",
  "footprint": "/api/footprints/8"
}
```

**Rust Request**:
```json
{
  "name": "Capacitor 100µF",
  "description": "Electrolytic capacitor",
  "category_id": 3,
  "footprint_id": 8
}
```

**Differences**:
- ⚠️ **Request format**: PHP uses IRIs, Rust uses integer IDs
- ⚠️ **Field naming**: Different conventions

**Compatibility Status**: ❌ **INCOMPATIBLE**

---

#### `PATCH /api/parts/{id}`

**Status**: ✅ **IMPLEMENTED**

**Compatibility Status**: ❌ **INCOMPATIBLE** (Same issues as POST)

---

#### `DELETE /api/parts/{id}`

**Status**: ✅ **IMPLEMENTED**

**PHP Response**: 204 No Content

**Rust Response**: 204 No Content

**Compatibility Status**: ✅ **COMPATIBLE**

---

### Categories API

Similar structure to Parts API with same compatibility issues.

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/categories` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/categories/{id}` | GET | ✅ | ✅ | ❌ Format differs |
| `/api/categories` | POST | ✅ | ✅ | ❌ Format differs |
| `/api/categories/{id}` | PATCH | ✅ | ✅ | ❌ Format differs |
| `/api/categories/{id}` | DELETE | ✅ | ✅ | ✅ Compatible |

---

### Footprints API

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/footprints` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/footprints/{id}` | GET | ✅ | ✅ | ❌ Format differs |
| `/api/footprints` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/footprints/{id}` | PATCH | ✅ | ❌ | ❌ Not implemented |
| `/api/footprints/{id}` | DELETE | ✅ | ❌ | ❌ Not implemented |

---

### Manufacturers API

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/manufacturers` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/manufacturers/{id}` | GET | ✅ | ✅ | ❌ Format differs |
| `/api/manufacturers` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/manufacturers/{id}` | PATCH | ✅ | ❌ | ❌ Not implemented |
| `/api/manufacturers/{id}` | DELETE | ✅ | ❌ | ❌ Not implemented |

---

### Storage Locations API

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/storage_locations` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/storage_locations/{id}` | GET | ✅ | ✅ | ❌ Format differs |
| `/api/storage_locations` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/storage_locations/{id}` | PATCH | ✅ | ❌ | ❌ Not implemented |
| `/api/storage_locations/{id}` | DELETE | ✅ | ❌ | ❌ Not implemented |

---

### Suppliers API

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/suppliers` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/suppliers/{id}` | GET | ✅ | ✅ | ❌ Format differs |
| `/api/suppliers` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/suppliers/{id}` | PATCH | ✅ | ❌ | ❌ Not implemented |
| `/api/suppliers/{id}` | DELETE | ✅ | ❌ | ❌ Not implemented |

---

### Users API

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/users` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/users/{id}` | GET | ✅ | ✅ | ❌ Format differs |
| `/api/users` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/users/{id}` | PATCH | ✅ | ❌ | ❌ Not implemented |
| `/api/users/{id}` | DELETE | ✅ | ❌ | ❌ Not implemented |

---

### Authentication API

#### Summary Table

| Endpoint | Method | PHP | Rust | Compatible |
|----------|--------|-----|------|------------|
| `/api/login` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/logout` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/token/refresh` | POST | ✅ | ❌ | ❌ Not implemented |
| `/api/me` | GET | ✅ | ❌ | ❌ Not implemented |

---

## Critical Compatibility Issues

### 1. Response Format Mismatch

**Issue**: PHP uses JSON-LD/Hydra format, Rust uses plain JSON

**Impact**: 🔴 **CRITICAL** - All existing API clients will break

**Solutions**:
- A) Implement Hydra format in Rust (full compatibility)
- B) Add content negotiation to support both formats
- C) Version the API (v1 = PHP format, v2 = Rust format)

**Recommendation**: Option B - Support both formats with Accept header

---

### 2. Field Naming Convention

**Issue**: PHP uses camelCase, Rust uses snake_case

**Impact**: 🔴 **CRITICAL** - Field references will break

**Solutions**:
- A) Use serde rename attributes to output camelCase
- B) Support both naming conventions
- C) Document as breaking change

**Recommendation**: Option A - Make Rust output camelCase

---

### 3. Relationship Representation

**Issue**: PHP uses IRIs (`/api/categories/5`), Rust uses IDs (`5`)

**Impact**: 🟡 **HIGH** - Client code needs modification

**Solutions**:
- A) Generate IRIs in Rust responses
- B) Provide both IRI and ID fields
- C) Document as breaking change

**Recommendation**: Option A - Generate IRIs to match PHP

---

### 4. Missing Relationships

**Issue**: Rust doesn't include related resources (partLots, attachments, etc.)

**Impact**: 🟡 **HIGH** - Incomplete data for clients

**Solutions**:
- A) Implement relationship loading in Rust
- B) Provide separate endpoints for relationships
- C) Implement in phases

**Recommendation**: Option C - Implement progressively

---

## Snapshot Test Results

### Test Infrastructure Status

- ⚠️ Test directory exists: `/tests/api_compat/`
- ❌ No snapshot files generated yet
- ❌ No test runner implemented yet
- ❌ No automated comparison tool

### Proposed Test Structure

```
tests/api_compat/
├── README.md
├── snapshots/
│   ├── php/
│   │   ├── parts_list.json
│   │   ├── parts_get_1.json
│   │   ├── categories_list.json
│   │   └── ...
│   └── rust/
│       ├── parts_list.json
│       ├── parts_get_1.json
│       ├── categories_list.json
│       └── ...
├── generate_snapshots.sh
├── compare_snapshots.sh
└── test_suite.rs
```

---

## Action Items

### Immediate (Week 1)

1. ❌ Implement Hydra/JSON-LD format support in Rust
2. ❌ Convert field names to camelCase in Rust responses
3. ❌ Generate IRI fields for relationships
4. ❌ Create snapshot test infrastructure
5. ❌ Generate baseline snapshots from PHP API

### Short-term (Week 2-3)

6. ❌ Implement all CRUD operations for read-only entities
7. ❌ Add relationship loading support
8. ❌ Implement content negotiation
9. ❌ Run full snapshot test suite
10. ❌ Document all intentional deviations

### Medium-term (Month 2)

11. ❌ Implement authentication endpoints
12. ❌ Add missing entities (attachments, projects, etc.)
13. ❌ Performance testing and comparison
14. ❌ Error response format alignment

---

## Success Metrics

API parity will be considered achieved when:

1. ✅ All implemented endpoints pass snapshot tests
2. ✅ Response structure matches PHP exactly (or by design)
3. ✅ HTTP status codes match for all scenarios
4. ✅ Error responses match format and content
5. ✅ Query parameter handling is identical
6. ✅ Content negotiation works correctly
7. ✅ Performance is equal or better than PHP
8. ✅ Documentation is complete and accurate

**Current Score: 0/8** ❌

---

## References

- [API Platform Documentation](https://api-platform.com/docs/)
- [Hydra Core Vocabulary](https://www.hydra-cg.com/spec/latest/core/)
- [JSON-LD Specification](https://json-ld.org/)
- [Backend API Routes](./architecture/api-routes.md)
