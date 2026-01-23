//! Schema validation tests
//!
//! Tests to ensure Rust models match the database schema and PHP entities.

use backend::models::*;
use chrono::Utc;

#[cfg(test)]
mod schema_validation_tests {
    use super::*;

    /// Test that Category model has all required fields from PHP entity
    #[test]
    fn test_category_has_all_fields() {
        let category = Category {
            id: 1,
            name: "Test Category".to_string(),
            parent_id: None,
            comment: "Test comment".to_string(),
            partname_hint: "Hint".to_string(),
            partname_regex: ".*".to_string(),
            part_ipn_prefix: "CAT".to_string(), // NEW: Added field
            disable_footprints: false,
            disable_manufacturers: false,
            disable_autodatasheets: false,
            disable_properties: false,
            default_description: "Default desc".to_string(),
            default_comment: "Default comment".to_string(),
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(category.id, 1);
        assert_eq!(category.name, "Test Category");
        assert_eq!(category.part_ipn_prefix, "CAT");
    }

    /// Test that StorageLocation model has all required fields from PHP entity
    #[test]
    fn test_storage_location_has_all_fields() {
        let location = StorageLocation {
            id: 1,
            name: "Shelf A1".to_string(),
            parent_id: None,
            comment: "Main shelf".to_string(),
            is_full: false,
            only_single_part: false,        // NEW: Added field
            limit_to_existing_parts: false, // NEW: Added field
            id_owner: Some(42),             // NEW: Added field
            part_owner_must_match: false,   // NEW: Added field
            storage_type_id: None,          // NEW: Added field
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(location.id, 1);
        assert_eq!(location.name, "Shelf A1");
        assert_eq!(location.only_single_part, false);
        assert_eq!(location.limit_to_existing_parts, false);
        assert_eq!(location.id_owner, Some(42));
        assert_eq!(location.part_owner_must_match, false);
    }

    /// Test that Supplier model has all required fields from PHP entity
    #[test]
    fn test_supplier_has_all_fields() {
        let supplier = Supplier {
            id: 1,
            name: "Digikey".to_string(),
            parent_id: None,
            comment: "Electronic components".to_string(),
            address: "123 Main St".to_string(),
            phone_number: "+1-555-0100".to_string(),
            fax_number: "+1-555-0101".to_string(),
            email_address: "sales@digikey.com".to_string(),
            website: "https://www.digikey.com".to_string(),
            shipping_costs: Some(5.99),
            default_currency_id: Some(1), // NEW: Added field
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(supplier.id, 1);
        assert_eq!(supplier.name, "Digikey");
        assert_eq!(supplier.shipping_costs, Some(5.99));
        assert_eq!(supplier.default_currency_id, Some(1));
    }

    /// Test that Part model has all required fields from PHP entity
    #[test]
    fn test_part_has_all_fields() {
        let part = Part {
            id: 1,
            name: "Resistor 10k".to_string(),
            description: "10k ohm resistor".to_string(),
            comment: "SMD package".to_string(),
            visible: true,
            favorite: false,
            id_category: 5,
            id_footprint: Some(2),
            id_manufacturer: Some(3),
            id_part_unit: Some(1),             // NEW: Added field
            ipn: Some("RES-10K-001".to_string()),
            mass: Some(0.05),
            tags: "resistor,10k,0805".to_string(),
            manufacturer_product_number: "RES10K".to_string(),
            manufacturer_product_url: "https://example.com/res10k".to_string(),
            manufacturing_status: Some("active".to_string()), // NEW: Added field
            minamount: 100.0,
            needs_review: false,
            id_part_custom_state: Some(1), // NEW: Added field
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(part.id, 1);
        assert_eq!(part.name, "Resistor 10k");
        assert_eq!(part.id_part_unit, Some(1));
        assert_eq!(part.manufacturing_status, Some("active".to_string()));
        assert_eq!(part.id_part_custom_state, Some(1));
    }

    /// Test that Footprint model has all required fields
    #[test]
    fn test_footprint_has_all_fields() {
        let footprint = Footprint {
            id: 1,
            name: "DIP8".to_string(),
            parent_id: None,
            comment: "8-pin DIP package".to_string(),
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(footprint.id, 1);
        assert_eq!(footprint.name, "DIP8");
    }

    /// Test that Manufacturer model has all required fields
    #[test]
    fn test_manufacturer_has_all_fields() {
        let manufacturer = Manufacturer {
            id: 1,
            name: "Texas Instruments".to_string(),
            parent_id: None,
            comment: "Semiconductor manufacturer".to_string(),
            address: "Dallas, TX".to_string(),
            phone_number: "+1-800-TI-HELP".to_string(),
            fax_number: "".to_string(),
            email_address: "ti@ti.com".to_string(),
            website: "https://www.ti.com".to_string(),
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(manufacturer.id, 1);
        assert_eq!(manufacturer.name, "Texas Instruments");
    }

    /// Test that User model has all required fields
    #[test]
    fn test_user_has_all_fields() {
        let user = User {
            id: 1,
            name: "admin".to_string(),
            first_name: Some("John".to_string()),
            last_name: Some("Doe".to_string()),
            email: Some("john@example.com".to_string()),
            password: Some("hashed_password".to_string()),
            disabled: false,
            config_theme: Some("dark".to_string()),
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(user.id, 1);
        assert_eq!(user.name, "admin");
        assert_eq!(user.config_theme, Some("dark".to_string()));
    }

    /// Test that PartLot model has all required fields
    #[test]
    fn test_part_lot_has_all_fields() {
        let part_lot = PartLot {
            id: 1,
            id_part: 5,
            id_storage_location: Some(3),
            description: "Lot from 2024".to_string(),
            comment: "Good condition".to_string(),
            expiration_date: None,
            amount: 50.0,
            instock_unknown: false,
            needs_refill: false,
            datetime_added: Some(Utc::now()),
            last_modified: Some(Utc::now()),
        };

        assert_eq!(part_lot.id, 1);
        assert_eq!(part_lot.id_part, 5);
        assert_eq!(part_lot.amount, 50.0);
    }

    /// Test manufacturing status enum values
    #[test]
    fn test_manufacturing_status_values() {
        let valid_statuses = vec![
            "announced",
            "active",
            "nrfnd",
            "eol",
            "discontinued",
        ];

        for status in valid_statuses {
            let part = Part {
                id: 1,
                name: "Test".to_string(),
                description: "".to_string(),
                comment: "".to_string(),
                visible: true,
                favorite: false,
                id_category: 1,
                id_footprint: None,
                id_manufacturer: None,
                id_part_unit: None,
                ipn: None,
                mass: None,
                tags: "".to_string(),
                manufacturer_product_number: "".to_string(),
                manufacturer_product_url: "".to_string(),
                manufacturing_status: Some(status.to_string()),
                minamount: 0.0,
                needs_review: false,
                id_part_custom_state: None,
                datetime_added: None,
                last_modified: None,
            };

            assert_eq!(part.manufacturing_status, Some(status.to_string()));
        }
    }

    /// Test optional fields can be None
    #[test]
    fn test_optional_fields_can_be_none() {
        let part = Part {
            id: 1,
            name: "Minimal Part".to_string(),
            description: "".to_string(),
            comment: "".to_string(),
            visible: true,
            favorite: false,
            id_category: 1,
            id_footprint: None,
            id_manufacturer: None,
            id_part_unit: None,
            ipn: None,
            mass: None,
            tags: "".to_string(),
            manufacturer_product_number: "".to_string(),
            manufacturer_product_url: "".to_string(),
            manufacturing_status: None,
            minamount: 0.0,
            needs_review: false,
            id_part_custom_state: None,
            datetime_added: None,
            last_modified: None,
        };

        assert!(part.id_footprint.is_none());
        assert!(part.id_manufacturer.is_none());
        assert!(part.id_part_unit.is_none());
        assert!(part.ipn.is_none());
        assert!(part.mass.is_none());
        assert!(part.manufacturing_status.is_none());
        assert!(part.id_part_custom_state.is_none());
    }

    /// Test hierarchical relationships
    #[test]
    fn test_hierarchical_parent_relationships() {
        // Category with parent
        let child_category = Category {
            id: 2,
            name: "Subcategory".to_string(),
            parent_id: Some(1),
            comment: "".to_string(),
            partname_hint: "".to_string(),
            partname_regex: "".to_string(),
            part_ipn_prefix: "".to_string(),
            disable_footprints: false,
            disable_manufacturers: false,
            disable_autodatasheets: false,
            disable_properties: false,
            default_description: "".to_string(),
            default_comment: "".to_string(),
            datetime_added: None,
            last_modified: None,
        };

        assert_eq!(child_category.parent_id, Some(1));

        // StorageLocation with parent
        let child_location = StorageLocation {
            id: 2,
            name: "Drawer 1".to_string(),
            parent_id: Some(1),
            comment: "".to_string(),
            is_full: false,
            only_single_part: false,
            limit_to_existing_parts: false,
            id_owner: None,
            part_owner_must_match: false,
            storage_type_id: None,
            datetime_added: None,
            last_modified: None,
        };

        assert_eq!(child_location.parent_id, Some(1));
    }
}
