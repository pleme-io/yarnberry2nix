use std::path::{Path, PathBuf};
use anyhow::{Result, Context};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::env;

// Define the configuration structure
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    pub database_url: Option<String>,
    pub cache_size: Option<usize>,
}

// Define a type alias for the Config with AppConfig
type AppConfiguration = Config<AppConfig>;

// Generic Config structure
#[derive(Debug, Serialize, Deserialize)]
struct Config<T> {
    pub general: GeneralConfig,
    pub specific: T,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeneralConfig {
    pub app_name: Option<String>,
    pub version: Option<String>,
    pub environment: Option<String>,
}

// Define the default configuration
fn default_config() -> Config<AppConfig> {
    Config {
        general: GeneralConfig {
            app_name: Some("Default App".to_string()),
            version: Some("0.1.0".to_string()),
            environment: Some("development".to_string()),
        },
        specific: AppConfig {
            database_url: Some("sqlite://default.db".to_string()),
            cache_size: Some(256),
        },
    }
}

// Define the paths to your config files
static CONFIG_PATHS: Lazy<Vec<PathBuf>> = Lazy::new(|| {
    let mut paths = Vec::new();

    // System-level configuration
    paths.push(PathBuf::from("/etc/yarn2berry/yarn2berry.yml"));

    // User-level configuration
    if let Some(home_dir) = dirs::home_dir() {
        paths.push(home_dir.join(".config/yarn2berry/yarn2berry.yml"));
    }

    // Repository-level configuration (local to the project)
    if let Ok(current_dir) = env::current_dir() {
        paths.push(current_dir.join(".yarn2berry.yml"));
    }

    paths
});

// Singleton instance of the configuration
static CONFIG: Lazy<Result<AppConfiguration>> = Lazy::new(|| {
    // Start with the default configuration
    let mut combined_config = serde_yaml::to_value(default_config())
        .context("Failed to serialize default configuration")?;

    // Load additional configurations from files
    let paths: Vec<&Path> = CONFIG_PATHS.iter().map(Path::new).collect();
    for path in paths {
        if path.exists() {
            let content = std::fs::read_to_string(path)
                .context(format!("Failed to read configuration file: {:?}", path))?;
            let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
                .context(format!("Failed to parse YAML configuration file: {:?}", path))?;
            Config::<AppConfig>::merge_values(&mut combined_config, yaml);
        }
    }

    // Deserialize the combined configuration into the strongly-typed structure
    let combined_str = serde_yaml::to_string(&combined_config)
        .context("Failed to serialize merged configuration into string")?;
    let config: Config<AppConfig> = serde_yaml::from_str(&combined_str)
        .context("Failed to deserialize merged configuration into Config struct")?;

    Ok(config)
});

impl<T> Config<T>
where
    T: for<'de> Deserialize<'de> + Serialize,
{
    /// Merge two `serde_yaml::Value` objects for merging YAML configurations
    fn merge_values(base: &mut serde_yaml::Value, override_val: serde_yaml::Value) {
        match (base, override_val) {
            (serde_yaml::Value::Mapping(base_map), serde_yaml::Value::Mapping(override_map)) => {
                for (key, value) in override_map {
                    Config::<T>::merge_values(base_map.entry(key).or_insert(serde_yaml::Value::Null), value);
                }
            }
            (base, override_val) => {
                *base = override_val; // Override with the new value
            }
        }
    }
}

/// Singleton accessor for the configuration
pub fn config() -> &'static AppConfiguration {
    match &*CONFIG {
        Ok(cfg) => cfg,
        Err(e) => panic!("Failed to load configuration: {:?}", e),
    }
}

// keeping this comment here to store usage information
//fn main() -> Result<()> {
//    // Access the configuration singleton
//    let cfg = config();
//
//    println!("App Name: {:?}", cfg.general.app_name);
//    println!("Environment: {:?}", cfg.general.environment);
//    println!("Database URL: {:?}", cfg.specific.database_url);
//
//    Ok(())
//}

