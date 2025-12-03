//! Application route handlers.
//!
//! This module contains all HTTP route handlers for the API endpoints.
//! Routes are organized by resource type (parts, categories, etc.).

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};

use crate::db::AppState;
use crate::error::{AppError, AppResult};
use crate::models::{
    Category, CreateCategory, CreatePart, Footprint, Manufacturer, PaginatedResponse,
    PaginationParams, Part, StorageLocation, Supplier, UpdateCategory, UpdatePart, User,
};

// ============================================================================
// Health Check
// ============================================================================

/// Health check endpoint.
///
/// Returns "OK" if the service is running.
#[utoipa::path(
    get,
    path = "/health",
    tag = "health",
    responses(
        (status = 200, description = "Service is healthy", body = String)
    )
)]
pub async fn health_check() -> &'static str {
    "OK"
}

/// API info endpoint.
///
/// Returns basic information about the API.
#[utoipa::path(
    get,
    path = "/api",
    tag = "health",
    responses(
        (status = 200, description = "API information", body = serde_json::Value)
    )
)]
pub async fn api_info() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "name": "Part-DB API",
        "version": "2.0.0",
        "description": "REST API for Part-DB inventory management"
    }))
}

// ============================================================================
// Parts
// ============================================================================

/// List all parts with pagination.
#[utoipa::path(
    get,
    path = "/api/parts",
    tag = "parts",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of parts", body = PaginatedResponse<Part>)
    )
)]
pub async fn list_parts(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<Part>>> {
    let offset = (params.page - 1) * params.per_page;

    let parts = sqlx::query_as::<_, Part>(
        r#"
        SELECT id, name, description, comment, visible, favorite,
               id_category, id_footprint, id_manufacturer, ipn, mass, tags,
               manufacturer_product_number, manufacturer_product_url,
               minamount, needs_review, datetime_added, last_modified
        FROM parts
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM parts")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        parts,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single part by ID.
#[utoipa::path(
    get,
    path = "/api/parts/{id}",
    tag = "parts",
    params(
        ("id" = i32, Path, description = "Part ID")
    ),
    responses(
        (status = 200, description = "Part details", body = Part),
        (status = 404, description = "Part not found")
    )
)]
pub async fn get_part(State(state): State<AppState>, Path(id): Path<i32>) -> AppResult<Json<Part>> {
    let part = sqlx::query_as::<_, Part>(
        r#"
        SELECT id, name, description, comment, visible, favorite,
               id_category, id_footprint, id_manufacturer, ipn, mass, tags,
               manufacturer_product_number, manufacturer_product_url,
               minamount, needs_review, datetime_added, last_modified
        FROM parts
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Part with id {} not found", id)))?;

    Ok(Json(part))
}

/// Create a new part.
#[utoipa::path(
    post,
    path = "/api/parts",
    tag = "parts",
    request_body = CreatePart,
    responses(
        (status = 201, description = "Part created", body = Part),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn create_part(
    State(state): State<AppState>,
    Json(payload): Json<CreatePart>,
) -> AppResult<(StatusCode, Json<Part>)> {
    let description = payload.description.unwrap_or_default();
    let comment = payload.comment.unwrap_or_default();

    let part = sqlx::query_as::<_, Part>(
        r#"
        INSERT INTO parts (name, description, comment, id_category, id_footprint, id_manufacturer,
                          visible, favorite, ipn, mass, tags, manufacturer_product_number,
                          manufacturer_product_url, minamount, needs_review, datetime_added, last_modified)
        VALUES ($1, $2, $3, $4, $5, $6, true, false, NULL, NULL, '', '', '', 0, false, NOW(), NOW())
        RETURNING id, name, description, comment, visible, favorite,
                  id_category, id_footprint, id_manufacturer, ipn, mass, tags,
                  manufacturer_product_number, manufacturer_product_url,
                  minamount, needs_review, datetime_added, last_modified
        "#,
    )
    .bind(&payload.name)
    .bind(&description)
    .bind(&comment)
    .bind(payload.category_id)
    .bind(payload.footprint_id)
    .bind(payload.manufacturer_id)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(part)))
}

/// Update an existing part.
#[utoipa::path(
    patch,
    path = "/api/parts/{id}",
    tag = "parts",
    params(
        ("id" = i32, Path, description = "Part ID")
    ),
    request_body = UpdatePart,
    responses(
        (status = 200, description = "Part updated", body = Part),
        (status = 404, description = "Part not found")
    )
)]
pub async fn update_part(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdatePart>,
) -> AppResult<Json<Part>> {
    // First check if the part exists
    let existing = sqlx::query_as::<_, Part>("SELECT * FROM parts WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Part with id {} not found", id)))?;

    let name = payload.name.unwrap_or(existing.name);
    let description = payload.description.unwrap_or(existing.description);
    let comment = payload.comment.unwrap_or(existing.comment);
    let category_id = payload.category_id.unwrap_or(existing.id_category);
    let footprint_id = payload.footprint_id.or(existing.id_footprint);
    let manufacturer_id = payload.manufacturer_id.or(existing.id_manufacturer);
    let favorite = payload.favorite.unwrap_or(existing.favorite);

    let part = sqlx::query_as::<_, Part>(
        r#"
        UPDATE parts
        SET name = $1, description = $2, comment = $3, id_category = $4,
            id_footprint = $5, id_manufacturer = $6, favorite = $7, last_modified = NOW()
        WHERE id = $8
        RETURNING id, name, description, comment, visible, favorite,
                  id_category, id_footprint, id_manufacturer, ipn, mass, tags,
                  manufacturer_product_number, manufacturer_product_url,
                  minamount, needs_review, datetime_added, last_modified
        "#,
    )
    .bind(&name)
    .bind(&description)
    .bind(&comment)
    .bind(category_id)
    .bind(footprint_id)
    .bind(manufacturer_id)
    .bind(favorite)
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(part))
}

/// Delete a part.
#[utoipa::path(
    delete,
    path = "/api/parts/{id}",
    tag = "parts",
    params(
        ("id" = i32, Path, description = "Part ID")
    ),
    responses(
        (status = 204, description = "Part deleted"),
        (status = 404, description = "Part not found")
    )
)]
pub async fn delete_part(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM parts WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("Part with id {} not found", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Categories
// ============================================================================

/// List all categories with pagination.
#[utoipa::path(
    get,
    path = "/api/categories",
    tag = "categories",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of categories", body = PaginatedResponse<Category>)
    )
)]
pub async fn list_categories(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<Category>>> {
    let offset = (params.page - 1) * params.per_page;

    let categories = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, parent_id, comment, partname_hint, partname_regex,
               disable_footprints, disable_manufacturers, disable_autodatasheets,
               disable_properties, default_description, default_comment,
               datetime_added, last_modified
        FROM categories
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM categories")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        categories,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single category by ID.
#[utoipa::path(
    get,
    path = "/api/categories/{id}",
    tag = "categories",
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
    responses(
        (status = 200, description = "Category details", body = Category),
        (status = 404, description = "Category not found")
    )
)]
pub async fn get_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Category>> {
    let category = sqlx::query_as::<_, Category>(
        r#"
        SELECT id, name, parent_id, comment, partname_hint, partname_regex,
               disable_footprints, disable_manufacturers, disable_autodatasheets,
               disable_properties, default_description, default_comment,
               datetime_added, last_modified
        FROM categories
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Category with id {} not found", id)))?;

    Ok(Json(category))
}

/// Create a new category.
#[utoipa::path(
    post,
    path = "/api/categories",
    tag = "categories",
    request_body = CreateCategory,
    responses(
        (status = 201, description = "Category created", body = Category),
        (status = 400, description = "Invalid input")
    )
)]
pub async fn create_category(
    State(state): State<AppState>,
    Json(payload): Json<CreateCategory>,
) -> AppResult<(StatusCode, Json<Category>)> {
    let comment = payload.comment.unwrap_or_default();

    let category = sqlx::query_as::<_, Category>(
        r#"
        INSERT INTO categories (name, parent_id, comment, partname_hint, partname_regex,
                               disable_footprints, disable_manufacturers, disable_autodatasheets,
                               disable_properties, default_description, default_comment,
                               datetime_added, last_modified)
        VALUES ($1, $2, $3, '', '', false, false, false, false, '', '', NOW(), NOW())
        RETURNING id, name, parent_id, comment, partname_hint, partname_regex,
                  disable_footprints, disable_manufacturers, disable_autodatasheets,
                  disable_properties, default_description, default_comment,
                  datetime_added, last_modified
        "#,
    )
    .bind(&payload.name)
    .bind(payload.parent_id)
    .bind(&comment)
    .fetch_one(&state.pool)
    .await?;

    Ok((StatusCode::CREATED, Json(category)))
}

/// Update an existing category.
#[utoipa::path(
    patch,
    path = "/api/categories/{id}",
    tag = "categories",
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
    request_body = UpdateCategory,
    responses(
        (status = 200, description = "Category updated", body = Category),
        (status = 404, description = "Category not found")
    )
)]
pub async fn update_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateCategory>,
) -> AppResult<Json<Category>> {
    // First check if the category exists
    let existing = sqlx::query_as::<_, Category>("SELECT * FROM categories WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await?
        .ok_or_else(|| AppError::NotFound(format!("Category with id {} not found", id)))?;

    let name = payload.name.unwrap_or(existing.name);
    let parent_id = payload.parent_id.or(existing.parent_id);
    let comment = payload.comment.unwrap_or(existing.comment);

    let category = sqlx::query_as::<_, Category>(
        r#"
        UPDATE categories
        SET name = $1, parent_id = $2, comment = $3, last_modified = NOW()
        WHERE id = $4
        RETURNING id, name, parent_id, comment, partname_hint, partname_regex,
                  disable_footprints, disable_manufacturers, disable_autodatasheets,
                  disable_properties, default_description, default_comment,
                  datetime_added, last_modified
        "#,
    )
    .bind(&name)
    .bind(parent_id)
    .bind(&comment)
    .bind(id)
    .fetch_one(&state.pool)
    .await?;

    Ok(Json(category))
}

/// Delete a category.
#[utoipa::path(
    delete,
    path = "/api/categories/{id}",
    tag = "categories",
    params(
        ("id" = i32, Path, description = "Category ID")
    ),
    responses(
        (status = 204, description = "Category deleted"),
        (status = 404, description = "Category not found")
    )
)]
pub async fn delete_category(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<StatusCode> {
    let result = sqlx::query("DELETE FROM categories WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!(
            "Category with id {} not found",
            id
        )));
    }

    Ok(StatusCode::NO_CONTENT)
}

// ============================================================================
// Footprints
// ============================================================================

/// List all footprints with pagination.
#[utoipa::path(
    get,
    path = "/api/footprints",
    tag = "footprints",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of footprints", body = PaginatedResponse<Footprint>)
    )
)]
pub async fn list_footprints(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<Footprint>>> {
    let offset = (params.page - 1) * params.per_page;

    let footprints = sqlx::query_as::<_, Footprint>(
        r#"
        SELECT id, name, parent_id, comment, datetime_added, last_modified
        FROM footprints
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM footprints")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        footprints,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single footprint by ID.
#[utoipa::path(
    get,
    path = "/api/footprints/{id}",
    tag = "footprints",
    params(
        ("id" = i32, Path, description = "Footprint ID")
    ),
    responses(
        (status = 200, description = "Footprint details", body = Footprint),
        (status = 404, description = "Footprint not found")
    )
)]
pub async fn get_footprint(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Footprint>> {
    let footprint = sqlx::query_as::<_, Footprint>(
        r#"
        SELECT id, name, parent_id, comment, datetime_added, last_modified
        FROM footprints
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Footprint with id {} not found", id)))?;

    Ok(Json(footprint))
}

// ============================================================================
// Manufacturers
// ============================================================================

/// List all manufacturers with pagination.
#[utoipa::path(
    get,
    path = "/api/manufacturers",
    tag = "manufacturers",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of manufacturers", body = PaginatedResponse<Manufacturer>)
    )
)]
pub async fn list_manufacturers(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<Manufacturer>>> {
    let offset = (params.page - 1) * params.per_page;

    let manufacturers = sqlx::query_as::<_, Manufacturer>(
        r#"
        SELECT id, name, parent_id, comment, address, phone_number, fax_number,
               email_address, website, datetime_added, last_modified
        FROM manufacturers
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM manufacturers")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        manufacturers,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single manufacturer by ID.
#[utoipa::path(
    get,
    path = "/api/manufacturers/{id}",
    tag = "manufacturers",
    params(
        ("id" = i32, Path, description = "Manufacturer ID")
    ),
    responses(
        (status = 200, description = "Manufacturer details", body = Manufacturer),
        (status = 404, description = "Manufacturer not found")
    )
)]
pub async fn get_manufacturer(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Manufacturer>> {
    let manufacturer = sqlx::query_as::<_, Manufacturer>(
        r#"
        SELECT id, name, parent_id, comment, address, phone_number, fax_number,
               email_address, website, datetime_added, last_modified
        FROM manufacturers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Manufacturer with id {} not found", id)))?;

    Ok(Json(manufacturer))
}

// ============================================================================
// Storage Locations
// ============================================================================

/// List all storage locations with pagination.
#[utoipa::path(
    get,
    path = "/api/storage_locations",
    tag = "storage",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of storage locations", body = PaginatedResponse<StorageLocation>)
    )
)]
pub async fn list_storage_locations(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<StorageLocation>>> {
    let offset = (params.page - 1) * params.per_page;

    let locations = sqlx::query_as::<_, StorageLocation>(
        r#"
        SELECT id, name, parent_id, comment, is_full, datetime_added, last_modified
        FROM storelocations
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM storelocations")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        locations,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single storage location by ID.
#[utoipa::path(
    get,
    path = "/api/storage_locations/{id}",
    tag = "storage",
    params(
        ("id" = i32, Path, description = "Storage location ID")
    ),
    responses(
        (status = 200, description = "Storage location details", body = StorageLocation),
        (status = 404, description = "Storage location not found")
    )
)]
pub async fn get_storage_location(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<StorageLocation>> {
    let location = sqlx::query_as::<_, StorageLocation>(
        r#"
        SELECT id, name, parent_id, comment, is_full, datetime_added, last_modified
        FROM storelocations
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Storage location with id {} not found", id)))?;

    Ok(Json(location))
}

// ============================================================================
// Suppliers
// ============================================================================

/// List all suppliers with pagination.
#[utoipa::path(
    get,
    path = "/api/suppliers",
    tag = "suppliers",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of suppliers", body = PaginatedResponse<Supplier>)
    )
)]
pub async fn list_suppliers(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<Supplier>>> {
    let offset = (params.page - 1) * params.per_page;

    let suppliers = sqlx::query_as::<_, Supplier>(
        r#"
        SELECT id, name, parent_id, comment, address, phone_number, fax_number,
               email_address, website, shipping_costs, datetime_added, last_modified
        FROM suppliers
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM suppliers")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        suppliers,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single supplier by ID.
#[utoipa::path(
    get,
    path = "/api/suppliers/{id}",
    tag = "suppliers",
    params(
        ("id" = i32, Path, description = "Supplier ID")
    ),
    responses(
        (status = 200, description = "Supplier details", body = Supplier),
        (status = 404, description = "Supplier not found")
    )
)]
pub async fn get_supplier(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> AppResult<Json<Supplier>> {
    let supplier = sqlx::query_as::<_, Supplier>(
        r#"
        SELECT id, name, parent_id, comment, address, phone_number, fax_number,
               email_address, website, shipping_costs, datetime_added, last_modified
        FROM suppliers
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Supplier with id {} not found", id)))?;

    Ok(Json(supplier))
}

// ============================================================================
// Users (Read-only for now)
// ============================================================================

/// List all users with pagination.
#[utoipa::path(
    get,
    path = "/api/users",
    tag = "users",
    params(PaginationParams),
    responses(
        (status = 200, description = "List of users", body = PaginatedResponse<User>)
    )
)]
pub async fn list_users(
    State(state): State<AppState>,
    Query(params): Query<PaginationParams>,
) -> AppResult<Json<PaginatedResponse<User>>> {
    let offset = (params.page - 1) * params.per_page;

    // Note: We use NULL as password to avoid exposing sensitive data
    let users = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, first_name, last_name, email, NULL as password, disabled,
               config_theme, datetime_added, last_modified
        FROM users
        ORDER BY name
        LIMIT $1 OFFSET $2
        "#,
    )
    .bind(params.per_page as i64)
    .bind(offset as i64)
    .fetch_all(&state.pool)
    .await?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await?;

    Ok(Json(PaginatedResponse::new(
        users,
        total.0,
        params.page,
        params.per_page,
    )))
}

/// Get a single user by ID.
#[utoipa::path(
    get,
    path = "/api/users/{id}",
    tag = "users",
    params(
        ("id" = i32, Path, description = "User ID")
    ),
    responses(
        (status = 200, description = "User details", body = User),
        (status = 404, description = "User not found")
    )
)]
pub async fn get_user(State(state): State<AppState>, Path(id): Path<i32>) -> AppResult<Json<User>> {
    // Note: We use NULL as password to avoid exposing sensitive data
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, name, first_name, last_name, email, NULL as password, disabled,
               config_theme, datetime_added, last_modified
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(id)
    .fetch_optional(&state.pool)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("User with id {} not found", id)))?;

    Ok(Json(user))
}
