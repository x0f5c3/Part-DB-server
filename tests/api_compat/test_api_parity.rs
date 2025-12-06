//! API Parity Tests
//!
//! This test suite validates that the Rust backend produces responses
//! that are compatible with the legacy PHP API.
//!
//! These tests use snapshot testing to compare response structures,
//! field names, and data types.

#[cfg(test)]
mod tests {
    use serde_json::Value;

    /// Helper to normalize JSON for comparison
    /// This function sorts object keys and normalizes formatting
    fn normalize_json(value: &Value) -> Value {
        match value {
            Value::Object(map) => {
                let mut normalized = serde_json::Map::new();
                for (k, v) in map {
                    normalized.insert(k.clone(), normalize_json(v));
                }
                Value::Object(normalized)
            }
            Value::Array(arr) => {
                Value::Array(arr.iter().map(normalize_json).collect())
            }
            _ => value.clone(),
        }
    }

    /// Compare two JSON values and report differences
    fn compare_json_structure(
        php_value: &Value,
        rust_value: &Value,
        path: &str,
    ) -> Vec<String> {
        let mut differences = Vec::new();

        match (php_value, rust_value) {
            (Value::Object(php_map), Value::Object(rust_map)) => {
                // Check for missing fields in Rust response
                for key in php_map.keys() {
                    if !rust_map.contains_key(key) {
                        differences.push(format!(
                            "{}.{}: Field missing in Rust response",
                            path, key
                        ));
                    }
                }

                // Check for extra fields in Rust response
                for key in rust_map.keys() {
                    if !php_map.contains_key(key) {
                        differences.push(format!(
                            "{}.{}: Extra field in Rust response",
                            path, key
                        ));
                    }
                }

                // Recursively compare common fields
                for (key, php_val) in php_map {
                    if let Some(rust_val) = rust_map.get(key) {
                        let new_path = if path.is_empty() {
                            key.clone()
                        } else {
                            format!("{}.{}", path, key)
                        };
                        differences.extend(compare_json_structure(
                            php_val, rust_val, &new_path,
                        ));
                    }
                }
            }
            (Value::Array(php_arr), Value::Array(rust_arr)) => {
                if !php_arr.is_empty() && !rust_arr.is_empty() {
                    // Compare structure of first element
                    differences.extend(compare_json_structure(
                        &php_arr[0],
                        &rust_arr[0],
                        &format!("{}[0]", path),
                    ));
                }
            }
            (php_val, rust_val) => {
                // Check type compatibility
                if std::mem::discriminant(php_val) != std::mem::discriminant(rust_val) {
                    differences.push(format!(
                        "{}: Type mismatch (PHP: {:?}, Rust: {:?})",
                        path,
                        php_val.as_str().map(|_| "string")
                            .or(php_val.as_i64().map(|_| "number"))
                            .or(php_val.as_bool().map(|_| "boolean"))
                            .or(php_val.as_null().map(|_| "null"))
                            .unwrap_or("unknown"),
                        rust_val.as_str().map(|_| "string")
                            .or(rust_val.as_i64().map(|_| "number"))
                            .or(rust_val.as_bool().map(|_| "boolean"))
                            .or(rust_val.as_null().map(|_| "null"))
                            .unwrap_or("unknown")
                    ));
                }
            }
        }

        differences
    }

    #[test]
    #[ignore] // Remove this when snapshot files are generated
    fn test_parts_list_structure() {
        // This test would load PHP and Rust snapshots and compare them
        // For now, it's a placeholder showing the testing approach

        // Example structure:
        // let php_snapshot = load_snapshot("snapshots/php/parts_list.json");
        // let rust_snapshot = load_snapshot("snapshots/rust/parts_list.json");
        //
        // let php_json: Value = serde_json::from_str(&php_snapshot).unwrap();
        // let rust_json: Value = serde_json::from_str(&rust_snapshot).unwrap();
        //
        // let differences = compare_json_structure(&php_json, &rust_json, "");
        //
        // assert!(differences.is_empty(), "Found differences:\n{}", differences.join("\n"));
    }

    #[test]
    #[ignore]
    fn test_parts_get_structure() {
        // Similar structure to test_parts_list_structure
    }

    #[test]
    #[ignore]
    fn test_categories_list_structure() {
        // Similar structure to test_parts_list_structure
    }
}
