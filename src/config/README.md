# Yarn2Berry Configuration Module

This module provides a flexible, hierarchical configuration system for applications, allowing configurations to be loaded and merged from multiple levels (system, user, and repository). It is designed to handle YAML-based configuration files and ensures that configurations are merged in the correct order of precedence.

---

## Features

- **Hierarchical Configuration**:

  - Merges configurations from:
    1. **System-level**: `/etc/yarn2berry/yarn2berry.yml`
    2. **User-level**: `~/.config/yarn2berry/yarn2berry.yml`
    3. **Repository-level**: `.yarn2berry.yml` (current project directory)
  - Later configurations override earlier ones, ensuring repository-level configurations have the highest priority.

- **Singleton Access**:

  - The configuration is loaded once and is globally accessible through the `config()` function.

- **Robust Error Handling**:

  - Provides detailed error messages for file reading and parsing issues.

- **Extensible**:
  - Easily adaptable to additional configuration sources or formats.

---

## Philosophy

This module is built on the principle of **layered configuration**, where settings can be overridden at more specific levels. The philosophy is to provide sane defaults at the system level, allow user-level customization, and enable project-specific overrides at the repository level.

By merging configurations hierarchically:

- System administrators can set global defaults.
- Users can define their preferences without modifying global settings.
- Developers can tailor configurations for individual projects.

---

## Usage

### 1. **Add Configuration Files**

- **System-level**: `/etc/yarn2berry/yarn2berry.yml`
- **User-level**: `~/.config/yarn2berry/yarn2berry.yml`
- **Repository-level**: `.yarn2berry.yml` (in the root of your project directory)

### 2. **Define Configuration Schema**

Define the configuration structure in Rust as a strongly typed struct. For example:

```rust
#[derive(Debug, Serialize, Deserialize)]
struct AppConfig {
    pub database_url: Option<String>,
    pub cache_size: Option<usize>,
}
```
