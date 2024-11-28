use crate::yarnberry::package_json::PackageJson;
use anyhow::{Context, Result};
use std::path::Path;

#[derive(Debug)]
pub struct YarnBerryEnvironment {
    pub package_json: Option<PackageJson>,
    // pub yarnrc_yml: Option<YarnRcYml>,
    // Add other components as needed
}

impl YarnBerryEnvironment {
    /// Creates a `YarnBerryEnvironment` from the given project root
    pub fn from_project_root(project_root: &Path) -> Result<Self> {
        // Attempt to load `package.json`
        let package_json = match PackageJson::from_file(&project_root.join("package.json")) {
            Ok(pkg) => Some(pkg),
            Err(err) => {
                eprintln!("Warning: Failed to load package.json: {:?}", err);
                None
            }
        };

        // Placeholder for future `yarnrc.yml` functionality
        // let yarnrc_yml = match YarnRcYml::from_file(&project_root.join(".yarnrc.yml")) {
        //     Ok(yml) => Some(yml),
        //     Err(err) => {
        //         eprintln!("Warning: Failed to load .yarnrc.yml: {:?}", err);
        //         None
        //     }
        // };

        // Add other components as they are implemented

        Ok(Self {
            package_json,
            // yarnrc_yml,
        })
    }
}

//fn main() -> Result<()> {
//    let project_root = Path::new("/path/to/project");
//
//    match YarnBerryEnvironment::from_project_root(project_root) {
//        Ok(env) => println!("Loaded environment: {:?}", env),
//        Err(err) => eprintln!("Failed to load environment: {:?}", err),
//    }
//
//    Ok(())
//}
