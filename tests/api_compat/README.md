# API Compatibility Tests

This directory contains tests to verify that the new Rust backend produces
responses that are compatible with the original PHP API.

## Structure

- `snapshots/` - JSON snapshots of API responses
  - `php/` - Snapshots from PHP/Symfony backend
  - `rust/` - Snapshots from Rust/Axum backend
- `*.sh` - Shell scripts for snapshot generation and comparison
- `test_api_parity.rs` - Rust-based API compatibility tests

## Quick Start

### Prerequisites

- `jq` - JSON processor (`sudo apt install jq` or `brew install jq`)
- Both PHP and Rust backends must be running

### Running the Tests

1. **Generate PHP snapshots** (from running PHP application):
   ```bash
   # Ensure PHP app is running on port 8000
   ./generate_php_snapshots.sh
   ```

2. **Generate Rust snapshots** (from running Rust application):
   ```bash
   # Ensure Rust app is running on port 3000
   cd ../../backend
   cargo run &
   cd ../tests/api_compat
   ./generate_rust_snapshots.sh
   ```

3. **Compare snapshots**:
   ```bash
   ./compare_snapshots.sh
   ```
   
   This will generate a `comparison_report.txt` file with detailed differences.

### Custom API URLs

By default, the scripts assume:
- PHP API: `http://localhost:8000`
- Rust API: `http://localhost:3000`

You can override these:

```bash
# For PHP snapshots
PHP_API_URL=http://localhost:9000 ./generate_php_snapshots.sh

# For Rust snapshots
RUST_API_URL=http://localhost:4000 ./generate_rust_snapshots.sh
```

## Compatibility Requirements

The Rust backend must:

1. Use the same URL paths as the PHP API (under `/api` prefix)
2. Return the same JSON structure for each endpoint
3. Use the same field names and types
4. Maintain the same HTTP status codes for success/error cases

## Current Implementation Status

### ✅ Implemented Endpoints

| Endpoint | Method | PHP | Rust | Status |
|----------|--------|-----|------|--------|
| `/api/parts` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/parts/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/parts` | POST | ✅ | ✅ | ⚠️ Format differs |
| `/api/parts/{id}` | PATCH | ✅ | ✅ | ⚠️ Format differs |
| `/api/parts/{id}` | DELETE | ✅ | ✅ | ✅ Compatible |
| `/api/categories` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/categories/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/categories` | POST | ✅ | ✅ | ⚠️ Format differs |
| `/api/categories/{id}` | PATCH | ✅ | ✅ | ⚠️ Format differs |
| `/api/categories/{id}` | DELETE | ✅ | ✅ | ✅ Compatible |
| `/api/footprints` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/footprints/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/manufacturers` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/manufacturers/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/storage_locations` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/storage_locations/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/suppliers` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/suppliers/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/users` | GET | ✅ | ✅ | ⚠️ Format differs |
| `/api/users/{id}` | GET | ✅ | ✅ | ⚠️ Format differs |

### Known Differences

1. **Response format**: PHP uses Hydra/JSON-LD format, Rust uses plain JSON
2. **Field naming**: PHP uses camelCase, Rust uses snake_case
3. **Pagination**: PHP uses Hydra pagination, Rust uses simple pagination
4. **Relationships**: PHP uses IRIs, Rust uses foreign key IDs

See [API Parity Report](../../docs/src/api-parity-report.md) for detailed analysis.

## Snapshot Files

Snapshot files are JSON files containing API responses. They are:

- **Version controlled** (add to git to track changes over time)
- **Human-readable** (pretty-printed JSON)
- **Sortable** (fields are in consistent order)

Example snapshot structure:

```json
{
  "items": [
    {
      "id": 1,
      "name": "Resistor 100Ω",
      "description": "1/4W Carbon Film"
    }
  ],
  "total": 150,
  "page": 1,
  "per_page": 10
}
```

## Troubleshooting

### "PHP API is not accessible"

Ensure the PHP development server is running:

```bash
cd /path/to/Part-DB-server
php -S localhost:8000 -t public
```

Or use Docker:

```bash
docker-compose up -d
```

### "Rust API is not accessible"

Ensure the Rust backend is running:

```bash
cd backend
cargo run
```

Check that the database connection is configured in `.env`.

### "jq is required but not installed"

Install jq:

- Ubuntu/Debian: `sudo apt install jq`
- macOS: `brew install jq`
- Windows: Download from https://stedolan.github.io/jq/

### Snapshots show "Part ID 1 may not exist"

This is normal if the database is empty. The scripts continue with other endpoints.
To populate test data:

```bash
php bin/console doctrine:fixtures:load
```

## Contributing

When adding new API endpoints:

1. Update the endpoint lists in `generate_php_snapshots.sh` and `generate_rust_snapshots.sh`
2. Run both snapshot generation scripts
3. Run the comparison script
4. Fix any incompatibilities in the Rust implementation
5. Update the endpoint status table in this README

## Related Documentation

- [Migration Status](../../docs/src/migration-status.md)
- [API Parity Report](../../docs/src/api-parity-report.md)
- [Backend Documentation](../../backend/README.md)
