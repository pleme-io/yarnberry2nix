pub mod structs;

use anyhow::{Result, Context};
use yarn_lock_parser::{parse_str, Entry};
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::yarnberry::yarn_lock::structs::YarnLockEntry;
use crate::yarnberry::yarn_lock::structs::YarnLock;

/// Parses a yarn.lock file into a `YarnLock` struct
pub fn parse_yarn_lock(path: &Path) -> Result<YarnLock> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read yarn.lock at {:?}", path))?;

    let parsed = parse_str(&content)
        .map_err(|err| anyhow::anyhow!("Failed to parse yarn.lock: {}", err))?;

    let entries = parsed
        .into_iter()
        .map(|entry| {
            let Entry {
                name,
                version,
                integrity,
                dependencies,
                ..
            } = entry;

            let yarn_lock_entry = YarnLockEntry {
                version: version.to_string(),
                resolved: None, // Adjust if `resolved` is part of `Entry`
                integrity: Some(integrity.to_string()), // Adjusted to handle non-Option
                dependencies: Some(
                    dependencies
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                ),
            };

            (name.to_string(), yarn_lock_entry)
        })
        .collect::<HashMap<String, YarnLockEntry>>();

    Ok(YarnLock { entries })
}
