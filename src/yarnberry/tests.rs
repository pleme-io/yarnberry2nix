#[cfg(test)]
mod tests {
    //use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use crate::yarnberry::structs::YarnBerryEnvironment;

    /// Helper to create a temporary project directory
    fn setup_temp_project() -> TempDir {
        tempfile::tempdir().expect("Failed to create temp directory")
    }

    /// Helper to write a `package.json` to the temporary directory
    fn write_package_json(project_root: &Path, content: &str) {
        let package_json_path = project_root.join("package.json");
        fs::write(&package_json_path, content).expect("Failed to write package.json");
    }

    #[test]
    fn test_from_project_root_with_valid_package_json() {
        let temp_dir = setup_temp_project();
        let project_root = temp_dir.path();

        let valid_package_json = r#"
        {
            "name": "example-project",
            "version": "1.0.0"
        }
        "#;
        write_package_json(project_root, valid_package_json);

        let result = YarnBerryEnvironment::from_project_root(project_root);
        assert!(result.is_ok());

        let environment = result.unwrap();
        assert!(environment.package_json.is_some());

        let package_json = environment.package_json.unwrap();
        assert_eq!(package_json.name, Some("example-project".to_string()));
        assert_eq!(package_json.version, Some("1.0.0".to_string()));
    }

    //#[test]
    //fn test_from_project_root_with_missing_package_json() {
    //    let temp_dir = setup_temp_project();
    //    let project_root = temp_dir.path();
    //
    //    // Ensure no package.json file exists in the directory
    //    assert!(!project_root.join("package.json").exists());
    //
    //    let result = YarnBerryEnvironment::from_project_root(project_root);
    //    assert!(result.is_err());
    //
    //    let error_message = result.unwrap_err().to_string();
    //    assert!(error_message.contains("Failed to parse package.json"));
    //}

    //#[test]
    //fn test_from_project_root_with_invalid_package_json() {
    //    let temp_dir = setup_temp_project();
    //    let project_root = temp_dir.path();
    //
    //    let invalid_package_json = r#"
    //    {
    //        "name": "example-project",
    //        "version": 
    //    }
    //    "#; // Invalid JSON (missing value for "version")
    //    write_package_json(project_root, invalid_package_json);
    //
    //    let result = YarnBerryEnvironment::from_project_root(project_root);
    //    assert!(result.is_err());
    //
    //    let error_message = result.unwrap_err().to_string();
    //    assert!(error_message.contains("Failed to parse package.json"));
    //}
}

