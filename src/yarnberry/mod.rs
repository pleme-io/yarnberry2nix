use std::path::Path;
use anyhow::{Result, Context};

mod package_json;
//mod yarnrc_yml;
//mod pnp;
//mod cache;

use package_json::PackageJson;
//use yarnrc_yml::YarnRcYml;

#[derive(Debug)]
pub struct YarnBerryEnvironment {
    pub package_json: Option<PackageJson>,
    // pub yarnrc_yml: Option<YarnRcYml>,
    // Add other components as needed
}

impl YarnBerryEnvironment {
    /// Creates a `YarnBerryEnvironment` from the given project root
    pub fn from_project_root(project_root: &Path) -> Result<Self> {
        let package_json = PackageJson::from_file(&project_root.join("package.json"))
            .context("Failed to parse package.json")?;

        //let yarnrc_yml = YarnRcYml::from_file(&project_root.join(".yarnrc.yml"))
        //    .context("Failed to parse .yarnrc.yml")?;
        
        // Add other components as they are implemented

        Ok(Self {
            package_json,
            //yarnrc_yml,
        })
    }
}

