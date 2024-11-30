use anyhow::{Result, Context};
use yarn_lock_parser::parse_str;
use std::fs;

/// Parses a yarn.lock file into a `YarnLock` struct
pub fn parse_yarn_lock(path: &Path) -> Result<YarnLock> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read yarn.lock at {:?}", path))?;

    let parsed = parse_str(&content)
        .map_err(|err| anyhow::anyhow!("Failed to parse yarn.lock: {}", err))?;

    let entries = parsed
        .into_iter()
        .map(|(key, value)| {
            let entry = YarnLockEntry {
                version: value.version,
                resolved: value.resolved,
                integrity: value.integrity,
                dependencies: value.dependencies,
            };
            (key, entry)
        })
        .collect();

    Ok(YarnLock { entries })
}
