#[cfg(test)]
mod tests {
    use crate::yarnberry::package_json::structs::PackageJson;
    use serde_json::json;

    #[test]
    fn test_package_json_parsing() {
        let package_json_str = json!({
            "name": "example-package",
            "version": "1.0.0"
        })
        .to_string();

        let parsed: PackageJson = serde_json::from_str(&package_json_str)
            .expect("Failed to parse package.json");

        assert_eq!(parsed.name, Some("example-package".to_string()));
        assert_eq!(parsed.version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_package_json_parsing_specific() {
        let package_json_str = json!({
            "name": "lilith",
            "packageManager": "yarn@4.4.0",
            "private": true,
            "workspaces": ["packages/*"],
            "scripts": {
                "media:assets": "scripts/media",
                "clean": "scripts/clean",
                "installs": "bin/install",
                "genconfig": "bin/genconfig",
                "build": "bin/build",
                "migrate": "bin/migrate",
                "restart": "bin/restart",
                "compose:up": "scripts/compose/up"
            },
            "devDependencies": {
                "@types/jest": "^29.5.12",
                "@types/node": "^20.14.10",
                "jest": "^29.7.0",
                "storybook-addon-designs": "^6.3.1",
                "ts-node": "^10.9.2",
                "typescript": "~5.3.3",
                "webpack": "^5.93.0",
                "webpack-cli": "^5.1.4"
            },
            "dependencies": {
                "dotenv": "^16.4.5"
            },
            "resolutions": {
                "react": "18.3.1",
                "react-dom": "18.3.1"
            }
        }).to_string();

        let parsed: PackageJson = serde_json::from_str(&package_json_str)
            .expect("Failed to parse package.json");

        assert_eq!(parsed.name, Some("lilith".to_string()));
        assert_eq!(parsed.package_manager, Some("yarn@4.4.0".to_string()));
        assert_eq!(parsed.private, Some(true));

        // Check scripts
        let scripts = parsed.scripts.unwrap();
        assert_eq!(scripts.get("media:assets"), Some(&"scripts/media".to_string()));
        assert_eq!(scripts.get("clean"), Some(&"scripts/clean".to_string()));

        // Check devDependencies
        let dev_dependencies = parsed.dev_dependencies.unwrap();
        assert_eq!(dev_dependencies.get("@types/jest"), Some(&"^29.5.12".to_string()));
        assert_eq!(dev_dependencies.get("jest"), Some(&"^29.7.0".to_string()));

        // Check dependencies
        let dependencies = parsed.dependencies.unwrap();
        assert_eq!(dependencies.get("dotenv"), Some(&"^16.4.5".to_string()));

        // Check resolutions
        let resolutions = parsed.resolutions.unwrap();
        assert_eq!(resolutions.get("react"), Some(&"18.3.1".to_string()));
        assert_eq!(resolutions.get("react-dom"), Some(&"18.3.1".to_string()));
    }
}
