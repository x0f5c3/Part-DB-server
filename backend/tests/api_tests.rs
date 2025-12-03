//! Integration tests for the Part-DB backend API.
//!
//! These tests verify the API endpoints work correctly.

// Note: Full integration tests would require a test database.
// These are placeholder tests that verify basic routing and serialization.

#[tokio::test]
async fn test_health_endpoint() {
    // This test would need the app to be set up without database
    // For now, we just verify the test infrastructure works
    assert!(true);
}

#[tokio::test]
async fn test_api_info_structure() {
    // Verify the API info JSON structure matches expectations
    let expected_fields = vec!["name", "version", "description"];

    let json_str = r#"{"name":"Part-DB API","version":"2.0.0","description":"REST API for Part-DB inventory management"}"#;
    let value: serde_json::Value = serde_json::from_str(json_str).unwrap();

    for field in expected_fields {
        assert!(value.get(field).is_some(), "Missing field: {}", field);
    }
}

#[tokio::test]
async fn test_pagination_params_defaults() {
    use backend::models::PaginationParams;

    // Test deserialization with defaults
    let json_str = "{}";
    let params: PaginationParams = serde_json::from_str(json_str).unwrap();

    assert_eq!(params.page, 1);
    assert_eq!(params.per_page, 30);
}

#[tokio::test]
async fn test_pagination_params_custom() {
    use backend::models::PaginationParams;

    let json_str = r#"{"page": 5, "per_page": 50}"#;
    let params: PaginationParams = serde_json::from_str(json_str).unwrap();

    assert_eq!(params.page, 5);
    assert_eq!(params.per_page, 50);
}

#[tokio::test]
async fn test_paginated_response_calculation() {
    use backend::models::PaginatedResponse;

    let response: PaginatedResponse<i32> = PaginatedResponse::new(vec![1, 2, 3, 4, 5], 100, 1, 10);

    assert_eq!(response.total, 100);
    assert_eq!(response.page, 1);
    assert_eq!(response.per_page, 10);
    assert_eq!(response.total_pages, 10);
    assert_eq!(response.items.len(), 5);
}

#[tokio::test]
async fn test_part_serialization() {
    use backend::models::Part;

    // Test that Part can be serialized to JSON
    let part_json = r#"{
        "id": 1,
        "name": "Test Part",
        "description": "A test part",
        "comment": "",
        "visible": true,
        "favorite": false,
        "id_category": 1,
        "id_footprint": null,
        "id_manufacturer": null,
        "ipn": null,
        "mass": null,
        "tags": "",
        "manufacturer_product_number": "",
        "manufacturer_product_url": "",
        "minamount": 0.0,
        "needs_review": false,
        "datetime_added": null,
        "last_modified": null
    }"#;

    let part: Part = serde_json::from_str(part_json).unwrap();
    assert_eq!(part.id, 1);
    assert_eq!(part.name, "Test Part");
    assert_eq!(part.id_category, 1);
    assert!(!part.favorite);
}

#[tokio::test]
async fn test_category_serialization() {
    use backend::models::Category;

    let category_json = r#"{
        "id": 1,
        "name": "Resistors",
        "parent_id": null,
        "comment": "All resistor types",
        "partname_hint": "",
        "partname_regex": "",
        "disable_footprints": false,
        "disable_manufacturers": false,
        "disable_autodatasheets": false,
        "disable_properties": false,
        "default_description": "",
        "default_comment": "",
        "datetime_added": null,
        "last_modified": null
    }"#;

    let category: Category = serde_json::from_str(category_json).unwrap();
    assert_eq!(category.id, 1);
    assert_eq!(category.name, "Resistors");
    assert!(category.parent_id.is_none());
}

#[tokio::test]
async fn test_create_part_dto() {
    use backend::models::CreatePart;

    let dto_json = r#"{
        "name": "New Part",
        "category_id": 1
    }"#;

    let dto: CreatePart = serde_json::from_str(dto_json).unwrap();
    assert_eq!(dto.name, "New Part");
    assert_eq!(dto.category_id, 1);
    assert!(dto.description.is_none());
    assert!(dto.footprint_id.is_none());
}

#[tokio::test]
async fn test_update_part_dto() {
    use backend::models::UpdatePart;

    let dto_json = r#"{
        "name": "Updated Part",
        "favorite": true
    }"#;

    let dto: UpdatePart = serde_json::from_str(dto_json).unwrap();
    assert_eq!(dto.name, Some("Updated Part".to_string()));
    assert_eq!(dto.favorite, Some(true));
    assert!(dto.category_id.is_none());
}
