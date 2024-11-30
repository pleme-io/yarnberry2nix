use crate::yarnberry::package_json::PackageJson;
use crate::yarnberry::yarn_lock::structs::YarnLock;
use crate::yarnberry::yarn_lock::parse_yarn_lock;
use anyhow::Result;
use std::path::Path;

#[derive(Debug)]
pub struct YarnBerryEnvironment {
    pub package_json: Option<PackageJson>,
    pub yarn_lock: Option<YarnLock>,
}

impl YarnBerryEnvironment {
    pub fn from_project_root(project_root: &Path) -> Result<Self> {
        let package_json = match PackageJson::from_file(&project_root.join("package.json")) {
            Ok(pkg) => Some(pkg),
            Err(err) => {
                eprintln!("Warning: Failed to load package.json: {:?}", err);
                None
            }
        };

        let yarn_lock = match parse_yarn_lock(&project_root.join("yarn.lock")) {
            Ok(lock) => Some(lock),
            Err(err) => {
                eprintln!("Warning: Failed to load yarn.lock: {:?}", err);
                None
            }
        };

        Ok(Self {
            package_json,
            yarn_lock,
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
