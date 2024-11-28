/*
* YarnBerryEnvironment will be a structure representing all data possibly used
* from the yarn berry envrionment.
*
* Various other structs will be attached to represent each element
* in berry such as the package_json or yarnrc_yml
*/
pub mod package_json;
// pub mod yarnrc_yml;
// pub mod pnp;
// pub mod cache;

use std::path::Path;
use anyhow::{Result, Context};
use package_json::structs::PackageJson;
// use yarnrc_yml::YarnRcYml;

#[derive(Debug)]
pub struct YarnBerryEnvironment {
    pub package_json: Option<PackageJson>,
    // pub yarnrc_yml: Option<YarnRcYml>,
    // Add other components as needed
}

impl YarnBerryEnvironment {
    /// Creates a `YarnBerryEnvironment` from the given project root
    pub fn from_project_root(project_root: &Path) -> Result<Self> {
        let package_json = Some(
            PackageJson::from_file(&project_root.join("package.json"))
                .context("Failed to parse package.json")?,
        );

        // let yarnrc_yml = Some(
        //     YarnRcYml::from_file(&project_root.join(".yarnrc.yml"))
        //         .context("Failed to parse .yarnrc.yml")?,
        // );

        // Add other components as they are implemented

        Ok(Self {
            package_json,
            // yarnrc_yml,
        })
    }
}
