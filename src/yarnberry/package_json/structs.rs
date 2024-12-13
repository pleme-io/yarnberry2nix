use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct PackageJson {
    pub name: Option<String>,
    pub version: Option<String>,
    pub description: Option<String>,
    pub main: Option<String>,
    pub scripts: Option<HashMap<String, String>>,
    pub dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    pub dev_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "peerDependencies")]
    pub peer_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "optionalDependencies")]
    pub optional_dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "bundledDependencies")]
    pub bundled_dependencies: Option<Vec<String>>,
    pub directories: Option<Directories>,
    pub config: Option<HashMap<String, String>>,
    pub workspaces: Option<Workspaces>,
    pub package_manager: Option<String>,
    pub license: Option<String>,
    pub repository: Option<Repository>,
    pub bugs: Option<Bugs>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub author: Option<Author>,
    pub contributors: Option<Vec<Author>>,
    pub engines: Option<HashMap<String, String>>,
    pub os: Option<Vec<String>>,
    pub cpu: Option<Vec<String>>,
    pub private: Option<bool>,
    pub publish_config: Option<HashMap<String, String>>,
    pub resolutions: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Directories {
    pub lib: Option<String>,
    pub bin: Option<String>,
    pub man: Option<String>,
    pub doc: Option<String>,
    pub example: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspaces {
    pub packages: Option<Vec<String>>,
    pub nohoist: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    #[serde(rename = "type")]
    pub repo_type: Option<String>,
    pub url: Option<String>,
    pub directory: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Bugs {
    pub url: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: Option<String>,
    pub email: Option<String>,
    pub url: Option<String>,
}

