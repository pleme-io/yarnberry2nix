pub mod structs; 
mod interfaces;
mod tests;

use anyhow::{Result, Context};
use serde_json;
use std::fs;
use std::path::Path;
pub use structs::{PackageJson, Repository};

fn from_file(path: &Path) -> Result<PackageJson> {
    let content = fs::read_to_string(path)
        .context(format!("Failed to read package.json at {:?}", path))?;
    let package_json: PackageJson = serde_json::from_str(&content)
        .context("Failed to parse package.json as JSON")?;
    Ok(package_json)
}

impl PackageJson {
    /// Public wrapper for `from_file` in `interfaces.rs`
    pub fn from_file(path: &Path) -> Result<Self> {
        from_file(path)
    }
}
