# Changelog

This document tracks all significant changes made to the Part-DB project during development and migration sessions.

## Format

Each entry should include:
- **Date**: When the change was made
- **Session/PR**: Reference to the Copilot session or PR
- **Changes**: Summary of what was done
- **Impact**: Any breaking changes or important notes

---

## 2024-12-06

### Documentation Migration

**Session**: Replace Jekyll documentation with mdbook

**Changes Made**:
- Replaced Jekyll-based documentation with mdbook
- Created comprehensive documentation structure covering:
  - Getting Started (installation, Docker, configuration)
  - Architecture (backend, frontend, API routes, database models)
  - API Reference (all endpoints documented)
  - Development guides (setup, building, testing, contributing)
  - Deployment guides (production, GitHub Actions)
- Added GitHub Actions workflow for automatic documentation deployment to GitHub Pages

**Impact**: Documentation is now built with mdbook and automatically deployed via GitHub Actions. Old Jekyll files have been removed.

---

## Template for New Entries

```markdown
## YYYY-MM-DD

### Brief Title

**Session**: [Session ID or PR reference]

**Changes Made**:
- List of changes

**Impact**: Any breaking changes or important notes
```
