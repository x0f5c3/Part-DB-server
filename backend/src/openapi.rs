//! OpenAPI documentation generation.
//!
//! This module provides OpenAPI/Swagger documentation for the API using utoipa.

use utoipa::OpenApi;

use crate::models::{
    Category, CreateCategory, CreatePart, Footprint, Manufacturer, PaginatedResponse,
    PaginationParams, Part, StorageLocation, Supplier, UpdateCategory, UpdatePart, User,
};

/// OpenAPI documentation for the Part-DB API.
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Part-DB API",
        version = "2.0.0",
        description = "REST API for Part-DB electronic parts inventory management system",
        license(name = "AGPL-3.0-or-later", url = "https://www.gnu.org/licenses/agpl-3.0.html"),
        contact(name = "Part-DB Team", url = "https://github.com/Part-DB/Part-DB-server")
    ),
    servers(
        (url = "/api", description = "API base path")
    ),
    paths(
        crate::routes::health_check,
        crate::routes::api_info,
        crate::routes::list_parts,
        crate::routes::get_part,
        crate::routes::create_part,
        crate::routes::update_part,
        crate::routes::delete_part,
        crate::routes::list_categories,
        crate::routes::get_category,
        crate::routes::create_category,
        crate::routes::update_category,
        crate::routes::delete_category,
        crate::routes::list_footprints,
        crate::routes::get_footprint,
        crate::routes::list_manufacturers,
        crate::routes::get_manufacturer,
        crate::routes::list_storage_locations,
        crate::routes::get_storage_location,
        crate::routes::list_suppliers,
        crate::routes::get_supplier,
        crate::routes::list_users,
        crate::routes::get_user,
    ),
    components(
        schemas(
            Part,
            Category,
            Footprint,
            Manufacturer,
            StorageLocation,
            Supplier,
            User,
            CreatePart,
            UpdatePart,
            CreateCategory,
            UpdateCategory,
            PaginationParams,
            PaginatedResponse<Part>,
            PaginatedResponse<Category>,
            PaginatedResponse<Footprint>,
            PaginatedResponse<Manufacturer>,
            PaginatedResponse<StorageLocation>,
            PaginatedResponse<Supplier>,
            PaginatedResponse<User>,
        )
    ),
    tags(
        (name = "health", description = "Health check endpoints"),
        (name = "parts", description = "Electronic parts management"),
        (name = "categories", description = "Part categories management"),
        (name = "footprints", description = "Component footprints"),
        (name = "manufacturers", description = "Part manufacturers"),
        (name = "storage", description = "Storage locations"),
        (name = "suppliers", description = "Part suppliers"),
        (name = "users", description = "User management"),
    )
)]
pub struct ApiDoc;
