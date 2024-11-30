#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::Path;

    /// Helper to write a temporary `yarn.lock` file.
    fn write_yarn_lock(temp_dir: &Path, content: &str) -> std::path::PathBuf {
        let yarn_lock_path = temp_dir.join("yarn.lock");
        fs::write(&yarn_lock_path, content).expect("Failed to write yarn.lock");
        yarn_lock_path
    }

    #[test]
    fn test_parse_yarn_lock_valid_file() {
        let temp_dir = tempdir().unwrap();
        let yarn_lock_content = r#"
"@babel/code-frame@^7.0.0":
  version "7.16.7"
  resolved "https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.16.7.tgz"
  integrity sha512-...
  dependencies:
    "@babel/highlight" "^7.0.0"
"#;
        let yarn_lock_path = write_yarn_lock(temp_dir.path(), yarn_lock_content);

        let result = parse_yarn_lock(&yarn_lock_path);
        assert!(result.is_ok());

        let yarn_lock = result.unwrap();
        assert_eq!(yarn_lock.entries.len(), 1);

        let entry = yarn_lock.entries.get("@babel/code-frame@^7.0.0").unwrap();
        assert_eq!(entry.version, "7.16.7");
        assert_eq!(
            entry.resolved.as_deref(),
            Some("https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.16.7.tgz")
        );
        assert!(entry.integrity.is_some());
        assert_eq!(
            entry.dependencies.as_ref().unwrap().get("@babel/highlight"),
            Some(&"^7.0.0".to_string())
        );
    }

    #[test]
    fn test_parse_yarn_lock_empty_file() {
        let temp_dir = tempdir().unwrap();
        let yarn_lock_path = write_yarn_lock(temp_dir.path(), "");

        let result = parse_yarn_lock(&yarn_lock_path);
        assert!(result.is_ok());

        let yarn_lock = result.unwrap();
        assert!(yarn_lock.entries.is_empty());
    }

    #[test]
    fn test_parse_yarn_lock_missing_file() {
        let temp_dir = tempdir().unwrap();
        let yarn_lock_path = temp_dir.path().join("missing.yarn.lock");

        let result = parse_yarn_lock(&yarn_lock_path);
        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Failed to read yarn.lock"));
    }

    #[test]
    fn test_parse_yarn_lock_invalid_format() {
        let temp_dir = tempdir().unwrap();
        let yarn_lock_content = r#"
invalid_entry
  version "1.0.0"
"#;
        let yarn_lock_path = write_yarn_lock(temp_dir.path(), yarn_lock_content);

        let result = parse_yarn_lock(&yarn_lock_path);
        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();
        assert!(error_message.contains("Failed to parse yarn.lock"));
    }

    #[test]
    fn test_parse_yarn_lock_multiple_entries() {
        let temp_dir = tempdir().unwrap();
        let yarn_lock_content = r#"
"@babel/code-frame@^7.0.0":
  version "7.16.7"
  resolved "https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.16.7.tgz"
  integrity sha512-...
  dependencies:
    "@babel/highlight" "^7.0.0"

"@babel/highlight@^7.0.0":
  version "7.18.6"
  resolved "https://registry.yarnpkg.com/@babel/highlight/-/highlight-7.18.6.tgz"
  integrity sha512-...
"#;
        let yarn_lock_path = write_yarn_lock(temp_dir.path(), yarn_lock_content);

        let result = parse_yarn_lock(&yarn_lock_path);
        assert!(result.is_ok());

        let yarn_lock = result.unwrap();
        assert_eq!(yarn_lock.entries.len(), 2);

        let entry1 = yarn_lock.entries.get("@babel/code-frame@^7.0.0").unwrap();
        assert_eq!(entry1.version, "7.16.7");

        let entry2 = yarn_lock.entries.get("@babel/highlight@^7.0.0").unwrap();
        assert_eq!(entry2.version, "7.18.6");
    }
}
