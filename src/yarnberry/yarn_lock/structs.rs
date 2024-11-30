use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct YarnLockEntry {
    pub version: String,
    pub resolved: Option<String>,
    pub integrity: Option<String>,
    pub dependencies: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct YarnLock {
    pub entries: HashMap<String, YarnLockEntry>,
}
