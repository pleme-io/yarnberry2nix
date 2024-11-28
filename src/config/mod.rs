use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// Generic Config structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Config<T> {
    pub general: GeneralConfig,
    pub specific: T,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub app_name: String,
    pub version: String,
    pub environment: String,
}

impl<T> Config<T>
where
    T: for<'de> Deserialize<'de>,
{
    /// Load a generic config file and deserialize it into the Config struct
    pub fn from_file(config_path: &Path) -> Result<Self> {
        let content = fs::read_to_string(config_path)
            .context("Failed to read configuration file")?;
        let config: Config<T> = serde_yaml::from_str(&content)
            .context("Failed to parse configuration file")?;
        Ok(config)
    }
}

