use super::structs::PackageJson;
use anyhow::{Result, Context};
use std::fs;
use std::path::Path;

/// Parse a `package.json` file into a `PackageJson` struct
pub fn from_file(path: &Path) -> Result<PackageJson> {
    let content = fs::read_to_string(path)
        .context(format!("Failed to read package.json at {:?}", path))?;
    let package_json: PackageJson = serde_json::from_str(&content)
        .context("Failed to parse package.json as JSON")?;
    Ok(package_json)
}

