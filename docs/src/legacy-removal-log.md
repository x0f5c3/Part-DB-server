# Legacy Removal Log

This document tracks all PHP code that has been removed or disabled as part of the migration to Rust + React.

Last Updated: 2025-12-06

## Overview

This log serves as:
- A record of what legacy code has been removed
- Justification for each removal
- References to replacement implementations
- Rollback information if needed

## Removal Process

Before removing any PHP code, we must:

1. ✅ Implement equivalent functionality in Rust/React
2. ✅ Verify API parity through snapshot tests
3. ✅ Verify UI parity through manual testing or screenshots
4. ✅ Update routing configuration
5. ✅ Document the removal in this log
6. ✅ Create a removal commit following Conventional Commits

## Format

Each entry should follow this format:

```
### [Component Name] - YYYY-MM-DD

**Type**: Controller / Service / Template / Route / Entity / etc.

**Files Removed**:
- `path/to/file1.php`
- `path/to/file2.php`

**Replaced By**:
- Rust: `backend/src/path/to/replacement.rs`
- Frontend: `frontend/src/path/to/replacement.tsx`

**Justification**:
Brief explanation of why this was removed.

**Tests Passed**:
- [ ] API snapshot tests
- [ ] Integration tests
- [ ] Manual QA

**Commit**: `abc123def` - "refactor: remove legacy XYZ controller"

**Rollback Plan**:
If needed, run: `git revert abc123def`

**Notes**:
Any additional context, gotchas, or warnings.
```

---

## Removed Components

### Nothing Removed Yet - 2025-12-06

**Status**: Migration in progress - no legacy PHP code has been removed yet.

**Reason**: We are still in the early phases of migration. Rust API does not yet have full feature parity with the PHP implementation.

**Next Removal Target**: To be determined after API parity is achieved.

---

## Disabled Components

### Nothing Disabled Yet - 2025-12-06

**Status**: All PHP components are still active.

**Strategy**: We will disable PHP routes gradually as Rust equivalents are proven stable.

---

## Planned Removals

This section lists PHP components that are candidates for removal once Rust equivalents are ready.

### Phase 1: API Endpoints (Target: TBD)

Once API parity is achieved and snapshot tests pass:

#### Parts API Routes
- `config/routes/api_platform.yaml` - Parts resource definitions
- Related API Platform configuration

**Prerequisites**:
- ✅ Rust CRUD endpoints implemented
- ❌ Snapshot tests passing
- ❌ Authentication middleware working
- ❌ Validation rules implemented
- ❌ Relationship loading working

#### Categories API Routes
- `config/routes/api_platform.yaml` - Categories resource definitions

**Prerequisites**:
- ✅ Rust CRUD endpoints implemented
- ❌ Snapshot tests passing
- ❌ Authentication middleware working
- ❌ Validation rules implemented

### Phase 2: Web Controllers (Target: TBD)

**Note**: Web UI controllers will remain until React frontend achieves full feature parity.

#### PartController (Web UI)
- `src/Controller/PartController.php`
- Related templates in `templates/part/`

**Prerequisites**:
- ❌ React parts management UI complete
- ❌ All part features implemented
- ❌ Visual parity verified
- ❌ User acceptance testing passed

### Phase 3: Entities (Target: TBD)

**Warning**: Entities should only be removed after both API and web UI migrations are complete.

#### Part Entity
- `src/Entity/Parts/Part.php`

**Prerequisites**:
- ❌ All API endpoints migrated
- ❌ All web UI migrated
- ❌ No remaining Doctrine queries referencing this entity
- ❌ Database migration strategy defined

---

## Migration Guidelines

### Before Removing Code

1. **Verify Replacement Exists**
   - Check that Rust/React implementation is complete
   - Ensure all features are implemented
   - Verify edge cases are handled

2. **Run Tests**
   ```bash
   # Backend tests
   cd backend && cargo test
   
   # API compatibility tests
   cd tests/api_compat && ./run_tests.sh
   
   # Frontend tests (when available)
   cd frontend && npm test
   ```

3. **Manual Verification**
   - Test all user flows
   - Verify error handling
   - Check performance
   - Test authentication/authorization

4. **Document in This Log**
   - Create entry with all required fields
   - Link to relevant commits
   - Provide rollback instructions

5. **Create Removal Commit**
   ```bash
   git add <files>
   git commit -m "refactor: remove legacy <component>"
   ```

### After Removing Code

1. **Monitor for Issues**
   - Watch error logs for 24-48 hours
   - Monitor user feedback
   - Check performance metrics

2. **Update Documentation**
   - Update architecture docs
   - Update API docs
   - Update deployment guides

3. **Clean Up Dependencies**
   - Remove unused Composer packages (if no longer needed)
   - Clean up unused imports
   - Remove unused configuration

---

## Statistics

### Overall Progress

| Category | Total Components | Removed | Percentage |
|----------|-----------------|---------|------------|
| Controllers | ~30 | 0 | 0% |
| API Resources | ~15 | 0 | 0% |
| Templates | ~100+ | 0 | 0% |
| Entities | ~30 | 0 | 0% |
| Services | ~50+ | 0 | 0% |
| **Total** | **~225+** | **0** | **0%** |

### By Migration Phase

| Phase | Components | Status |
|-------|-----------|--------|
| Phase 1: Read-Only API | 14 routes | ✅ Rust implemented, PHP still active |
| Phase 2: Full CRUD API | 21 routes | ⏳ Partially implemented |
| Phase 3: Authentication | 4 routes | ❌ Not started |
| Phase 4: Advanced Features | 50+ routes | ❌ Not started |
| Phase 5: Web UI | 100+ templates | ❌ Not started |
| Phase 6: Entities | 30+ classes | ❌ Not started |

---

## Rollback Procedures

### Full Rollback

If the migration needs to be rolled back entirely:

```bash
# 1. Stop Rust backend
systemctl stop partdb-backend  # or equivalent

# 2. Ensure PHP backend is running
systemctl start php-fpm
systemctl start nginx

# 3. Revert routing changes
git checkout origin/master -- config/routes/

# 4. Clear caches
php bin/console cache:clear
```

### Partial Rollback

To rollback specific components:

```bash
# 1. Find the removal commit hash from this log
# 2. Revert that specific commit
git revert <commit-hash>

# 3. Test the restoration
php bin/console cache:clear
# Run tests

# 4. Deploy
git push origin <branch>
```

---

## Risk Assessment

### Low Risk Removals ✅

Components that can be safely removed once replacements are proven:

- API endpoint routes (easily reversible)
- Individual controller methods
- Unused service classes
- Orphaned templates

### Medium Risk Removals ⚠️

Components that require careful testing before removal:

- Entities (database schema changes)
- Authentication services
- Core business logic services
- Event listeners/subscribers

### High Risk Removals 🔴

Components that should be removed last:

- Database migrations
- Core kernel modifications
- Authentication system
- File handling infrastructure

---

## Notes

### Why Keep This Log?

1. **Auditability**: Track exactly what was changed and when
2. **Reversibility**: Quick rollback if issues are discovered
3. **Knowledge Transfer**: Help team understand migration progress
4. **Compliance**: Some organizations require change logs
5. **Debugging**: Trace issues back to specific changes

### Conventional Commit Prefixes for Removals

- `refactor:` - Removing code as part of refactoring
- `remove:` - Explicitly removing deprecated code
- `chore:` - Removing build/config artifacts

### Related Documentation

- [Migration Status](./migration-status.md) - Overall migration progress
- [API Parity Report](./api-parity-report.md) - API compatibility status
- [Architecture Overview](./architecture/structure.md) - System architecture

---

## Change History

| Date | Change | Author |
|------|--------|--------|
| 2025-12-06 | Created initial legacy removal log | Migration System |

---

## Future Enhancements

Ideas for improving this process:

1. Automated detection of unused PHP code
2. Dependency graph visualization
3. Automated test coverage reporting
4. Risk score calculation for removals
5. Automated rollback testing
