#!/bin/bash
# Generate API response snapshots from the Rust backend
#
# This script captures JSON responses from various API endpoints
# and saves them as snapshot files for comparison with the PHP implementation.

set -e

# Configuration
RUST_BASE_URL="${RUST_API_URL:-http://localhost:3000}"
OUTPUT_DIR="$(dirname "$0")/snapshots/rust"
TIMEOUT=10

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Logging functions
log_info() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Create output directory
mkdir -p "$OUTPUT_DIR"

# Helper function to fetch and save endpoint response
fetch_endpoint() {
    local name=$1
    local endpoint=$2
    local output_file="$OUTPUT_DIR/${name}.json"
    
    log_info "Fetching $name from $endpoint..."
    
    if curl -s -f --max-time $TIMEOUT \
        -H "Accept: application/json" \
        "$RUST_BASE_URL$endpoint" \
        -o "$output_file"; then
        
        # Pretty-print the JSON
        if command -v jq &> /dev/null; then
            jq . "$output_file" > "${output_file}.tmp" && mv "${output_file}.tmp" "$output_file"
        fi
        
        log_info "✓ Saved snapshot to $output_file"
        return 0
    else
        log_error "✗ Failed to fetch $endpoint"
        return 1
    fi
}

# Main execution
log_info "Starting Rust API snapshot generation..."
log_info "Rust API URL: $RUST_BASE_URL"
log_info "Output directory: $OUTPUT_DIR"
echo ""

# Check if Rust API is accessible
if ! curl -s -f --max-time $TIMEOUT "$RUST_BASE_URL/api" > /dev/null; then
    log_error "Rust API is not accessible at $RUST_BASE_URL"
    log_error "Please ensure the Rust application is running"
    log_error ""
    log_error "To start the Rust backend:"
    log_error "  cd backend"
    log_error "  cargo run"
    exit 1
fi

log_info "Rust API is accessible ✓"
echo ""

# Parts endpoints
log_info "=== Parts Endpoints ==="
fetch_endpoint "parts_list" "/api/parts?page=1&per_page=10"
fetch_endpoint "parts_get_1" "/api/parts/1" || log_warn "Part ID 1 may not exist"
echo ""

# Categories endpoints
log_info "=== Categories Endpoints ==="
fetch_endpoint "categories_list" "/api/categories?page=1&per_page=10"
fetch_endpoint "categories_get_1" "/api/categories/1" || log_warn "Category ID 1 may not exist"
echo ""

# Footprints endpoints
log_info "=== Footprints Endpoints ==="
fetch_endpoint "footprints_list" "/api/footprints?page=1&per_page=10"
fetch_endpoint "footprints_get_1" "/api/footprints/1" || log_warn "Footprint ID 1 may not exist"
echo ""

# Manufacturers endpoints
log_info "=== Manufacturers Endpoints ==="
fetch_endpoint "manufacturers_list" "/api/manufacturers?page=1&per_page=10"
fetch_endpoint "manufacturers_get_1" "/api/manufacturers/1" || log_warn "Manufacturer ID 1 may not exist"
echo ""

# Storage Locations endpoints
log_info "=== Storage Locations Endpoints ==="
fetch_endpoint "storage_locations_list" "/api/storage_locations?page=1&per_page=10"
fetch_endpoint "storage_locations_get_1" "/api/storage_locations/1" || log_warn "Storage location ID 1 may not exist"
echo ""

# Suppliers endpoints
log_info "=== Suppliers Endpoints ==="
fetch_endpoint "suppliers_list" "/api/suppliers?page=1&per_page=10"
fetch_endpoint "suppliers_get_1" "/api/suppliers/1" || log_warn "Supplier ID 1 may not exist"
echo ""

# Users endpoints
log_info "=== Users Endpoints ==="
fetch_endpoint "users_list" "/api/users?page=1&per_page=10"
fetch_endpoint "users_get_1" "/api/users/1" || log_warn "User ID 1 may not exist"
echo ""

log_info "=== Snapshot Generation Complete ==="
log_info "Snapshots saved to: $OUTPUT_DIR"
log_info ""
log_info "Next steps:"
log_info "1. Run compare_snapshots.sh to identify differences"
