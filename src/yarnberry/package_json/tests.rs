#[cfg(test)]
mod tests {
    use super::structs::{PackageJson, Repository}; // Explicitly import structs
    use serde_json::{from_str, json}; // Import `from_str` and `json` for JSON handling
    use serde_json;


    #[test]
    fn test_parse_package_json() {
        // Example package.json as a JSON string
        let package_json_str = json!({
            "name": "example-package",
            "version": "1.0.0",
            "description": "An example package for testing",
            "main": "index.js",
            "scripts": {
                "start": "node index.js",
                "test": "jest"
            },
            "dependencies": {
                "express": "^4.17.1"
            },
            "devDependencies": {
                "jest": "^26.6.0"
            },
            "keywords": ["example", "test", "package"],
            "repository": {
                "type": "git",
                "url": "https://github.com/example/repo.git"
            },
            "license": "MIT",
            "private": true
        })
        .to_string();

        // Parse the JSON string into the PackageJson struct
        let parsed: PackageJson = serde_json::from_str(&package_json_str)
            .expect("Failed to parse package.json");

        // Assertions to verify correctness
        assert_eq!(parsed.name, Some("example-package".to_string()));
        assert_eq!(parsed.version, Some("1.0.0".to_string()));
        assert_eq!(
            parsed.dependencies,
            Some(
                [("express".to_string(), "^4.17.1".to_string())]
                    .iter()
                    .cloned()
                    .collect()
            )
        );
        assert_eq!(
            parsed.dev_dependencies,
            Some(
                [("jest".to_string(), "^26.6.0".to_string())]
                    .iter()
                    .cloned()
                    .collect()
            )
        );
        assert!(parsed.private.unwrap());
        assert_eq!(
            parsed.repository,
            Some(Repository {
                repo_type: Some("git".to_string()),
                url: Some("https://github.com/example/repo.git".to_string()),
                directory: None,
            })
        );
    }
}

