#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::tempdir;

    #[test]
    fn test_from_project_root_with_valid_package_json() {
        // Create a temporary directory to simulate the project root
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_root = temp_dir.path();

        // Create a valid package.json file in the temporary directory
        let package_json_path = project_root.join("package.json");
        let package_json_content = r#"
        {
            "name": "example-project",
            "version": "1.0.0"
        }
        "#;
        fs::write(&package_json_path, package_json_content).expect("Failed to write package.json");

        // Test YarnBerryEnvironment::from_project_root
        let result = YarnBerryEnvironment::from_project_root(project_root);
        assert!(result.is_ok());

        let environment = result.unwrap();
        assert!(environment.package_json.is_some());

        let package_json = environment.package_json.unwrap();
        assert_eq!(package_json.name, Some("example-project".to_string()));
        assert_eq!(package_json.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_from_project_root_with_missing_package_json() {
        // Create a temporary directory to simulate the project root
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_root = temp_dir.path();

        // Ensure no package.json file exists in the directory
        let package_json_path = project_root.join("package.json");
        assert!(!package_json_path.exists());

        // Test YarnBerryEnvironment::from_project_root
        let result = YarnBerryEnvironment::from_project_root(project_root);
        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Failed to parse package.json"));
    }

    #[test]
    fn test_from_project_root_with_invalid_package_json() {
        // Create a temporary directory to simulate the project root
        let temp_dir = tempdir().expect("Failed to create temp directory");
        let project_root = temp_dir.path();

        // Create an invalid package.json file in the temporary directory
        let package_json_path = project_root.join("package.json");
        let package_json_content = r#"
        {
            "name": "example-project",
            "version": 
        }
        "#; // Invalid JSON (missing value for "version")
        fs::write(&package_json_path, package_json_content).expect("Failed to write package.json");

        // Test YarnBerryEnvironment::from_project_root
        let result = YarnBerryEnvironment::from_project_root(project_root);
        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Failed to parse package.json"));
    }
}
