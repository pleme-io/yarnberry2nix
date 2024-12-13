use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde::de::{self, Deserializer, Visitor, MapAccess};
use std::fmt;

// Implement custom deserialization for Workspaces
fn deserialize_workspaces<'de, D>(deserializer: D) -> Result<Option<Workspaces>, D::Error>
where
    D: Deserializer<'de>,
{
    struct WorkspacesVisitor;

    impl<'de> Visitor<'de> for WorkspacesVisitor {
        type Value = Workspaces;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a list of strings or an object with packages and nohoist")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
        where
            V: de::SeqAccess<'de>,
        {
            let mut vec = Vec::new();
            while let Some(value) = seq.next_element()? {
                vec.push(value);
            }
            Ok(Workspaces::Array(vec))
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut packages = None;
            let mut nohoist = None;

            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "packages" => packages = map.next_value()?,
                    "nohoist" => nohoist = map.next_value()?,
                    _ => return Err(de::Error::unknown_field(&key, &["packages", "nohoist"])),
                }
            }
            Ok(Workspaces::Object { packages, nohoist })
        }
    }

    let visitor = WorkspacesVisitor;
    deserializer.deserialize_any(visitor).map(Some)
}
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
    #[serde(deserialize_with = "deserialize_workspaces")]
    pub workspaces: Option<Workspaces>,
    #[serde(rename = "packageManager")] // Added this line
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
#[serde(untagged)]
pub enum Workspaces {
    Array(Vec<String>),
    Object {
        packages: Option<Vec<String>>,
        nohoist: Option<Vec<String>>,
    },
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

