pub mod structs; // Publicly expose the `structs` module
mod tests;       // Include the `tests` module

use anyhow::{Result, Context};
use serde_json; // Import serde_json for JSON parsing
use std::fs;
use std::path::Path;
use structs::PackageJson; // Import `PackageJson`

/// Internal function to parse a `package.json` file into a `PackageJson` struct
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
