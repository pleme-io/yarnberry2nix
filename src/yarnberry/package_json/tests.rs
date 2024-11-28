#[cfg(test)]
mod tests {
    use crate::yarnberry::package_json::structs::PackageJson; // Use the correct path
    use serde_json::json;

    #[test]
    fn test_package_json_parsing() {
        // Example package.json as a JSON string
        let package_json_str = json!({
            "name": "example-package",
            "version": "1.0.0"
        })
        .to_string();

        // Parse the JSON string into the PackageJson struct
        let parsed: PackageJson = serde_json::from_str(&package_json_str)
            .expect("Failed to parse package.json");

        // Assertions to verify correctness
        assert_eq!(parsed.name, Some("example-package".to_string()));
        assert_eq!(parsed.version, Some("1.0.0".to_string()));
    }
}
