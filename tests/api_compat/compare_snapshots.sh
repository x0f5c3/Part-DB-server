#!/bin/bash
# Compare API response snapshots between PHP and Rust implementations
#
# This script analyzes differences in JSON structure, field names,
# and data types between the two implementations.

set -e

# Configuration
SCRIPT_DIR="$(dirname "$0")"
PHP_SNAPSHOTS="$SCRIPT_DIR/snapshots/php"
RUST_SNAPSHOTS="$SCRIPT_DIR/snapshots/rust"
REPORT_FILE="$SCRIPT_DIR/comparison_report.txt"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
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

log_section() {
    echo -e "${BLUE}=== $1 ===${NC}"
}

# Initialize report
echo "API Compatibility Comparison Report" > "$REPORT_FILE"
echo "Generated: $(date)" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Check if jq is available
if ! command -v jq &> /dev/null; then
    log_error "jq is required but not installed. Please install jq."
    exit 1
fi

# Check if snapshot directories exist
if [ ! -d "$PHP_SNAPSHOTS" ]; then
    log_error "PHP snapshots directory not found: $PHP_SNAPSHOTS"
    log_error "Please run generate_php_snapshots.sh first"
    exit 1
fi

if [ ! -d "$RUST_SNAPSHOTS" ]; then
    log_error "Rust snapshots directory not found: $RUST_SNAPSHOTS"
    log_error "Please run generate_rust_snapshots.sh first"
    exit 1
fi

# Function to extract keys from JSON
get_json_keys() {
    local file=$1
    jq -r 'paths(scalars) | join(".")' "$file" | sort
}

# Function to compare two snapshot files
compare_snapshots() {
    local name=$1
    local php_file="$PHP_SNAPSHOTS/${name}.json"
    local rust_file="$RUST_SNAPSHOTS/${name}.json"
    
    log_section "$name"
    
    if [ ! -f "$php_file" ]; then
        log_warn "PHP snapshot not found: $php_file"
        echo "[SKIP] $name - PHP snapshot missing" >> "$REPORT_FILE"
        return 1
    fi
    
    if [ ! -f "$rust_file" ]; then
        log_warn "Rust snapshot not found: $rust_file"
        echo "[SKIP] $name - Rust snapshot missing" >> "$REPORT_FILE"
        return 1
    fi
    
    # Extract keys from both files
    local php_keys=$(get_json_keys "$php_file")
    local rust_keys=$(get_json_keys "$rust_file")
    
    # Find differences
    local only_in_php=$(comm -23 <(echo "$php_keys") <(echo "$rust_keys"))
    local only_in_rust=$(comm -13 <(echo "$php_keys") <(echo "$rust_keys"))
    local common=$(comm -12 <(echo "$php_keys") <(echo "$rust_keys"))
    
    local has_differences=0
    
    echo "" >> "$REPORT_FILE"
    echo "## $name" >> "$REPORT_FILE"
    echo "" >> "$REPORT_FILE"
    
    if [ -n "$only_in_php" ]; then
        has_differences=1
        log_error "Fields only in PHP:"
        echo "$only_in_php" | while read -r field; do
            log_error "  - $field"
            echo "  [MISSING IN RUST] $field" >> "$REPORT_FILE"
        done
    fi
    
    if [ -n "$only_in_rust" ]; then
        has_differences=1
        log_warn "Fields only in Rust:"
        echo "$only_in_rust" | while read -r field; do
            log_warn "  + $field"
            echo "  [EXTRA IN RUST] $field" >> "$REPORT_FILE"
        done
    fi
    
    if [ $has_differences -eq 0 ]; then
        log_info "✓ Structure matches"
        echo "  [OK] Structures match" >> "$REPORT_FILE"
    fi
    
    echo "" >> "$REPORT_FILE"
    echo ""
    
    return $has_differences
}

# Main execution
log_info "Starting API compatibility comparison..."
log_info "PHP snapshots: $PHP_SNAPSHOTS"
log_info "Rust snapshots: $RUST_SNAPSHOTS"
log_info "Report will be saved to: $REPORT_FILE"
echo ""

# Counter for differences
total_tests=0
failed_tests=0

# Compare all endpoint types
endpoints=(
    "parts_list"
    "parts_get_1"
    "categories_list"
    "categories_get_1"
    "footprints_list"
    "footprints_get_1"
    "manufacturers_list"
    "manufacturers_get_1"
    "storage_locations_list"
    "storage_locations_get_1"
    "suppliers_list"
    "suppliers_get_1"
    "users_list"
    "users_get_1"
)

for endpoint in "${endpoints[@]}"; do
    total_tests=$((total_tests + 1))
    if ! compare_snapshots "$endpoint"; then
        failed_tests=$((failed_tests + 1))
    fi
done

# Summary
log_section "Summary"
echo "" >> "$REPORT_FILE"
echo "## Summary" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "Total endpoints tested: $total_tests" >> "$REPORT_FILE"
echo "Compatible endpoints: $((total_tests - failed_tests))" >> "$REPORT_FILE"
echo "Incompatible endpoints: $failed_tests" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

log_info "Total endpoints tested: $total_tests"
log_info "Compatible endpoints: $((total_tests - failed_tests))"
log_info "Incompatible endpoints: $failed_tests"
echo ""

if [ $failed_tests -eq 0 ]; then
    log_info "✓ All tested endpoints are compatible!"
    echo "[SUCCESS] All tested endpoints are compatible!" >> "$REPORT_FILE"
    exit 0
else
    log_error "✗ Found $failed_tests incompatible endpoint(s)"
    log_info "See detailed report: $REPORT_FILE"
    echo "[FAILURE] Found incompatibilities that need to be addressed" >> "$REPORT_FILE"
    exit 1
fi
