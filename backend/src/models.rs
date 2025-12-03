//! Domain models and data structures.
//!
//! This module contains the core domain models that map to the database entities.
//! Each model uses `serde` for JSON serialization and `sqlx::FromRow` for database mapping.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a category that parts can belong to.
/// Categories are hierarchical and can have parent categories.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    /// Unique identifier for the category
    pub id: i32,
    /// Name of the category
    pub name: String,
    /// ID of the parent category (null for root categories)
    pub parent_id: Option<i32>,
    /// Comment or description for this category
    pub comment: String,
    /// Hint shown under the part name field when creating parts in this category
    pub partname_hint: String,
    /// Regular expression to validate part names in this category
    pub partname_regex: String,
    /// Whether footprints are disabled for parts in this category
    pub disable_footprints: bool,
    /// Whether manufacturers are disabled for parts in this category
    pub disable_manufacturers: bool,
    /// Whether auto-datasheets are disabled for parts in this category
    pub disable_autodatasheets: bool,
    /// Whether properties are disabled for parts in this category
    pub disable_properties: bool,
    /// Default description for parts created in this category
    pub default_description: String,
    /// Default comment for parts created in this category
    pub default_comment: String,
    /// Timestamp when the category was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the category was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents a footprint for electronic components.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Footprint {
    /// Unique identifier for the footprint
    pub id: i32,
    /// Name of the footprint (e.g., "DIP8", "SMD0805")
    pub name: String,
    /// ID of the parent footprint (for hierarchical organization)
    pub parent_id: Option<i32>,
    /// Comment or description for this footprint
    pub comment: String,
    /// Timestamp when the footprint was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the footprint was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents a manufacturer of electronic parts.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Manufacturer {
    /// Unique identifier for the manufacturer
    pub id: i32,
    /// Name of the manufacturer
    pub name: String,
    /// ID of the parent manufacturer (for hierarchical organization)
    pub parent_id: Option<i32>,
    /// Comment or description for this manufacturer
    pub comment: String,
    /// Website URL for the manufacturer
    pub address: String,
    /// Phone number for the manufacturer
    pub phone_number: String,
    /// Fax number for the manufacturer
    pub fax_number: String,
    /// Email address for the manufacturer
    pub email_address: String,
    /// Website URL for the manufacturer
    pub website: String,
    /// Timestamp when the manufacturer was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the manufacturer was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents a storage location where parts are stored.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct StorageLocation {
    /// Unique identifier for the storage location
    pub id: i32,
    /// Name of the storage location
    pub name: String,
    /// ID of the parent location (for hierarchical organization)
    pub parent_id: Option<i32>,
    /// Comment or description for this location
    pub comment: String,
    /// Whether this location can store parts directly
    pub is_full: bool,
    /// Timestamp when the location was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the location was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents a supplier that sells parts.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Supplier {
    /// Unique identifier for the supplier
    pub id: i32,
    /// Name of the supplier
    pub name: String,
    /// ID of the parent supplier (for hierarchical organization)
    pub parent_id: Option<i32>,
    /// Comment or description for this supplier
    pub comment: String,
    /// Website URL for the supplier
    pub address: String,
    /// Phone number for the supplier
    pub phone_number: String,
    /// Fax number for the supplier
    pub fax_number: String,
    /// Email address for the supplier
    pub email_address: String,
    /// Website URL for the supplier
    pub website: String,
    /// Shipping costs from this supplier
    pub shipping_costs: Option<f64>,
    /// Timestamp when the supplier was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the supplier was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents an electronic part in the inventory.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Part {
    /// Unique identifier for the part
    pub id: i32,
    /// Name of the part
    pub name: String,
    /// Description of the part
    pub description: String,
    /// Additional comments about the part
    pub comment: String,
    /// Whether this part is visible
    pub visible: bool,
    /// Whether this part is marked as a favorite
    pub favorite: bool,
    /// ID of the category this part belongs to
    pub id_category: i32,
    /// ID of the footprint for this part (optional)
    pub id_footprint: Option<i32>,
    /// ID of the manufacturer for this part (optional)
    pub id_manufacturer: Option<i32>,
    /// Internal part number (IPN)
    pub ipn: Option<String>,
    /// Mass of the part in grams
    pub mass: Option<f64>,
    /// Tags associated with the part (comma-separated)
    pub tags: String,
    /// Manufacturer's product number/part number
    pub manufacturer_product_number: String,
    /// URL to the manufacturer's product page
    pub manufacturer_product_url: String,
    /// Minimum amount of parts that should be in stock
    pub minamount: f64,
    /// Whether this part needs review
    pub needs_review: bool,
    /// Timestamp when the part was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the part was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents a lot of parts at a specific storage location.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
#[allow(dead_code)]
pub struct PartLot {
    /// Unique identifier for the lot
    pub id: i32,
    /// ID of the part this lot belongs to
    pub id_part: i32,
    /// ID of the storage location
    pub id_storage_location: Option<i32>,
    /// Description of this lot
    pub description: String,
    /// Additional comment about this lot
    pub comment: String,
    /// Expiration date of this lot
    pub expiration_date: Option<DateTime<Utc>>,
    /// Amount of parts in this lot
    pub amount: f64,
    /// Whether the exact amount is unknown
    pub instock_unknown: bool,
    /// Whether this lot needs to be refilled
    pub needs_refill: bool,
    /// Timestamp when the lot was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the lot was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// Represents a user in the system.
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Unique identifier for the user
    pub id: i32,
    /// Username (unique)
    pub name: String,
    /// First name of the user
    pub first_name: Option<String>,
    /// Last name of the user
    pub last_name: Option<String>,
    /// Email address
    pub email: Option<String>,
    /// Hashed password (never serialized)
    #[serde(skip_serializing)]
    #[allow(dead_code)]
    pub password: Option<String>,
    /// Whether the user is disabled
    pub disabled: bool,
    /// Theme preference
    pub config_theme: Option<String>,
    /// Timestamp when the user was created
    pub datetime_added: Option<DateTime<Utc>>,
    /// Timestamp when the user was last modified
    pub last_modified: Option<DateTime<Utc>>,
}

/// DTO for creating a new part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePart {
    /// Name of the part
    pub name: String,
    /// Description of the part
    pub description: Option<String>,
    /// Additional comments about the part
    pub comment: Option<String>,
    /// ID of the category this part belongs to
    pub category_id: i32,
    /// ID of the footprint for this part (optional)
    pub footprint_id: Option<i32>,
    /// ID of the manufacturer for this part (optional)
    pub manufacturer_id: Option<i32>,
}

/// DTO for updating a part.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePart {
    /// Name of the part (optional update)
    pub name: Option<String>,
    /// Description of the part (optional update)
    pub description: Option<String>,
    /// Additional comments about the part (optional update)
    pub comment: Option<String>,
    /// ID of the category this part belongs to (optional update)
    pub category_id: Option<i32>,
    /// ID of the footprint for this part (optional update)
    pub footprint_id: Option<i32>,
    /// ID of the manufacturer for this part (optional update)
    pub manufacturer_id: Option<i32>,
    /// Whether this part is a favorite (optional update)
    pub favorite: Option<bool>,
}

/// DTO for creating a new category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCategory {
    /// Name of the category
    pub name: String,
    /// ID of the parent category (optional)
    pub parent_id: Option<i32>,
    /// Comment or description for this category
    pub comment: Option<String>,
}

/// DTO for updating a category.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCategory {
    /// Name of the category (optional update)
    pub name: Option<String>,
    /// ID of the parent category (optional update)
    pub parent_id: Option<i32>,
    /// Comment or description for this category (optional update)
    pub comment: Option<String>,
}

/// Pagination parameters for list queries.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationParams {
    /// Page number (1-indexed)
    #[serde(default = "default_page")]
    pub page: i32,
    /// Number of items per page
    #[serde(default = "default_per_page")]
    pub per_page: i32,
}

fn default_page() -> i32 {
    1
}

fn default_per_page() -> i32 {
    30
}

/// Paginated response wrapper.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginatedResponse<T> {
    /// The items on the current page
    pub items: Vec<T>,
    /// Total number of items
    pub total: i64,
    /// Current page number
    pub page: i32,
    /// Number of items per page
    pub per_page: i32,
    /// Total number of pages
    pub total_pages: i32,
}

impl<T> PaginatedResponse<T> {
    /// Creates a new paginated response.
    pub fn new(items: Vec<T>, total: i64, page: i32, per_page: i32) -> Self {
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
        }
    }
}
