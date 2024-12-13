#[cfg(test)]
mod tests {
    use crate::yarnberry::package_json::structs::PackageJson;
    use serde_json::json;
    use crate::yarnberry::package_json::structs::Workspaces;

    //#[test]
    //fn test_package_json_parsing() {
    //    let package_json_str = json!({
    //        "name": "example-package",
    //        "version": "1.0.0"
    //    })
    //    .to_string();
    //
    //    let parsed: PackageJson = serde_json::from_str(&package_json_str)
    //        .expect("Failed to parse package.json");
    //
    //    assert_eq!(parsed.name, Some("example-package".to_string()));
    //    assert_eq!(parsed.version, Some("1.0.0".to_string()));
    //}

    #[test]
    fn test_package_json_parsing_specific() {
        let package_json_str = json!({
            "name": "lilith",
            "packageManager": "yarn@4.4.0",
            "private": true,
            "workspaces": ["packages/*"]
        })
        .to_string();
    
        let parsed: PackageJson = serde_json::from_str(&package_json_str)
            .expect("Failed to parse package.json");

        // Debug the parsed workspaces
        dbg!(&parsed.workspaces);

        // Validate `name`
        assert_eq!(parsed.name, Some("lilith".to_string()));
        
        // Validate `packageManager`
        assert_eq!(parsed.package_manager, Some("yarn@4.4.0".to_string()));
    
        // Validate other fields as necessary
        assert_eq!(parsed.private, Some(true));
    
        if let Some(Workspaces::Array(workspaces)) = parsed.workspaces {
            assert_eq!(workspaces, vec!["packages/*"]);
        } else {
            panic!("Workspaces should be an array of strings");
        }
    }

}
