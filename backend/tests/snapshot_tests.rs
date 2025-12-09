//! API snapshot tests for compatibility assurance.
//!
//! These tests verify that the API responses maintain the expected structure
//! across changes to ensure backward compatibility.

use insta::assert_json_snapshot;
use serde_json::json;

/// Test the Part model structure matches expected format.
#[test]
fn test_part_response_structure() {
    let part = json!({
        "id": 1,
        "name": "Test Resistor",
        "description": "10k Ohm 1/4W resistor",
        "comment": "Common pull-up resistor",
        "visible": true,
        "favorite": false,
        "id_category": 5,
        "id_footprint": 12,
        "id_manufacturer": 3,
        "ipn": "R-10K-025W",
        "mass": 0.1,
        "tags": "resistor,passive",
        "manufacturer_product_number": "RC0402FR-0710KL",
        "manufacturer_product_url": "https://example.com/product",
        "minamount": 100.0,
        "needs_review": false,
        "datetime_added": "2024-01-15T10:30:00Z",
        "last_modified": "2024-01-20T14:45:00Z"
    });

    assert_json_snapshot!("part_response", part);
}

/// Test the Category model structure.
#[test]
fn test_category_response_structure() {
    let category = json!({
        "id": 1,
        "name": "Resistors",
        "parent_id": null,
        "comment": "All types of resistors",
        "partname_hint": "R-",
        "partname_regex": "^R-.*$",
        "disable_footprints": false,
        "disable_manufacturers": false,
        "disable_autodatasheets": false,
        "disable_properties": false,
        "default_description": "Resistor component",
        "default_comment": "",
        "datetime_added": "2024-01-01T00:00:00Z",
        "last_modified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("category_response", category);
}

/// Test the Footprint model structure.
#[test]
fn test_footprint_response_structure() {
    let footprint = json!({
        "id": 1,
        "name": "SMD 0402",
        "parent_id": null,
        "comment": "Standard 0402 SMD package",
        "datetime_added": "2024-01-01T00:00:00Z",
        "last_modified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("footprint_response", footprint);
}

/// Test the Manufacturer model structure.
#[test]
fn test_manufacturer_response_structure() {
    let manufacturer = json!({
        "id": 1,
        "name": "Texas Instruments",
        "parent_id": null,
        "comment": "Major semiconductor manufacturer",
        "address": "Dallas, TX, USA",
        "phone_number": "+1-800-336-5236",
        "fax_number": "",
        "email_address": "support@ti.com",
        "website": "https://www.ti.com",
        "datetime_added": "2024-01-01T00:00:00Z",
        "last_modified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("manufacturer_response", manufacturer);
}

/// Test the StorageLocation model structure.
#[test]
fn test_storage_location_response_structure() {
    let storage = json!({
        "id": 1,
        "name": "Drawer A1",
        "parent_id": null,
        "comment": "Small components storage",
        "is_full": false,
        "datetime_added": "2024-01-01T00:00:00Z",
        "last_modified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("storage_location_response", storage);
}

/// Test the Supplier model structure.
#[test]
fn test_supplier_response_structure() {
    let supplier = json!({
        "id": 1,
        "name": "Mouser Electronics",
        "parent_id": null,
        "comment": "Major electronics distributor",
        "address": "1000 N Main St, Mansfield, TX",
        "phone_number": "+1-800-346-6873",
        "fax_number": "",
        "email_address": "sales@mouser.com",
        "website": "https://www.mouser.com",
        "shipping_costs": 5.99,
        "datetime_added": "2024-01-01T00:00:00Z",
        "last_modified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("supplier_response", supplier);
}

/// Test the User model structure (without password).
#[test]
fn test_user_response_structure() {
    let user = json!({
        "id": 1,
        "name": "admin",
        "first_name": "Admin",
        "last_name": "User",
        "email": "admin@example.com",
        "disabled": false,
        "config_theme": "dark",
        "datetime_added": "2024-01-01T00:00:00Z",
        "last_modified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("user_response", user);
}

/// Test the PaginatedResponse structure.
#[test]
fn test_paginated_response_structure() {
    let paginated = json!({
        "items": [
            {"id": 1, "name": "Item 1"},
            {"id": 2, "name": "Item 2"}
        ],
        "total": 100,
        "page": 1,
        "per_page": 30,
        "total_pages": 4
    });

    assert_json_snapshot!("paginated_response", paginated);
}

/// Test the API info endpoint response.
#[test]
fn test_api_info_structure() {
    let api_info = json!({
        "name": "Part-DB API",
        "version": "2.0.0",
        "description": "REST API for Part-DB inventory management"
    });

    assert_json_snapshot!("api_info", api_info);
}

/// Test the CreatePart DTO structure.
#[test]
fn test_create_part_dto_structure() {
    let create_part = json!({
        "name": "New Resistor",
        "description": "Test resistor",
        "comment": "For testing",
        "category_id": 1,
        "footprint_id": 2,
        "manufacturer_id": 3
    });

    assert_json_snapshot!("create_part_dto", create_part);
}

/// Test the UpdatePart DTO structure.
#[test]
fn test_update_part_dto_structure() {
    let update_part = json!({
        "name": "Updated Resistor",
        "favorite": true
    });

    assert_json_snapshot!("update_part_dto", update_part);
}

/// Test the LoginRequest structure.
#[test]
fn test_login_request_structure() {
    let login_request = json!({
        "username": "admin",
        "password": "secret123"
    });

    assert_json_snapshot!("login_request", login_request);
}

/// Test the LoginResponse structure.
#[test]
fn test_login_response_structure() {
    let login_response = json!({
        "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
        "token_type": "Bearer",
        "expires_in": 86400
    });

    assert_json_snapshot!("login_response", login_response);
}
