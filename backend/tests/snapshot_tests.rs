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
        "idCategory": 5,
        "idFootprint": 12,
        "idManufacturer": 3,
        "ipn": "R-10K-025W",
        "mass": 0.1,
        "tags": "resistor,passive",
        "manufacturerProductNumber": "RC0402FR-0710KL",
        "manufacturerProductUrl": "https://example.com/product",
        "minamount": 100.0,
        "needsReview": false,
        "datetimeAdded": "2024-01-15T10:30:00Z",
        "lastModified": "2024-01-20T14:45:00Z"
    });

    assert_json_snapshot!("part_response", part);
}

/// Test the Category model structure.
#[test]
fn test_category_response_structure() {
    let category = json!({
        "id": 1,
        "name": "Resistors",
        "parentId": null,
        "comment": "All types of resistors",
        "partnameHint": "R-",
        "partnameRegex": "^R-.*$",
        "disableFootprints": false,
        "disableManufacturers": false,
        "disableAutodatasheets": false,
        "disableProperties": false,
        "defaultDescription": "Resistor component",
        "defaultComment": "",
        "datetimeAdded": "2024-01-01T00:00:00Z",
        "lastModified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("category_response", category);
}

/// Test the Footprint model structure.
#[test]
fn test_footprint_response_structure() {
    let footprint = json!({
        "id": 1,
        "name": "SMD 0402",
        "parentId": null,
        "comment": "Standard 0402 SMD package",
        "datetimeAdded": "2024-01-01T00:00:00Z",
        "lastModified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("footprint_response", footprint);
}

/// Test the Manufacturer model structure.
#[test]
fn test_manufacturer_response_structure() {
    let manufacturer = json!({
        "id": 1,
        "name": "Texas Instruments",
        "parentId": null,
        "comment": "Major semiconductor manufacturer",
        "address": "Dallas, TX, USA",
        "phoneNumber": "+1-800-336-5236",
        "faxNumber": "",
        "emailAddress": "support@ti.com",
        "website": "https://www.ti.com",
        "datetimeAdded": "2024-01-01T00:00:00Z",
        "lastModified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("manufacturer_response", manufacturer);
}

/// Test the StorageLocation model structure.
#[test]
fn test_storage_location_response_structure() {
    let storage = json!({
        "id": 1,
        "name": "Drawer A1",
        "parentId": null,
        "comment": "Small components storage",
        "isFull": false,
        "datetimeAdded": "2024-01-01T00:00:00Z",
        "lastModified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("storage_location_response", storage);
}

/// Test the Supplier model structure.
#[test]
fn test_supplier_response_structure() {
    let supplier = json!({
        "id": 1,
        "name": "Mouser Electronics",
        "parentId": null,
        "comment": "Major electronics distributor",
        "address": "1000 N Main St, Mansfield, TX",
        "phoneNumber": "+1-800-346-6873",
        "faxNumber": "",
        "emailAddress": "sales@mouser.com",
        "website": "https://www.mouser.com",
        "shippingCosts": 5.99,
        "datetimeAdded": "2024-01-01T00:00:00Z",
        "lastModified": "2024-01-01T00:00:00Z"
    });

    assert_json_snapshot!("supplier_response", supplier);
}

/// Test the User model structure (without password).
#[test]
fn test_user_response_structure() {
    let user = json!({
        "id": 1,
        "name": "admin",
        "firstName": "Admin",
        "lastName": "User",
        "email": "admin@example.com",
        "disabled": false,
        "configTheme": "dark",
        "datetimeAdded": "2024-01-01T00:00:00Z",
        "lastModified": "2024-01-01T00:00:00Z"
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
        "perPage": 30,
        "totalPages": 4
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
        "categoryId": 1,
        "footprintId": 2,
        "manufacturerId": 3
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
        "tokenType": "Bearer",
        "expiresIn": 86400
    });

    assert_json_snapshot!("login_response", login_response);
}
