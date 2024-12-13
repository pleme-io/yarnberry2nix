//#[cfg(test)]
//mod tests {
//    use super::*;
//    use std::io::Cursor;
//
//    /// Parses a `yarn.lock` content from a string.
//    fn parse_yarn_lock_from_str(content: &str) -> Result<YarnLock> {
//        // Use a `Cursor` to simulate a file-like object.
//        let reader = Cursor::new(content);
//        let parsed = parse_yarn_lock_reader(reader)?;
//        Ok(parsed)
//    }
//
//    /// Helper function to wrap `parse_yarn_lock` logic for testing with in-memory content.
//    fn parse_yarn_lock_reader<R: std::io::Read>(mut reader: R) -> Result<YarnLock> {
//        use std::io::Read;
//        let mut content = String::new();
//        reader.read_to_string(&mut content)?;
//        let parsed = parse_str(&content)
//            .map_err(|err| anyhow::anyhow!("Failed to parse yarn.lock: {}", err))?;
//
//        let entries = parsed
//            .into_iter()
//            .map(|entry| {
//                let Entry {
//                    name,
//                    version,
//                    integrity,
//                    dependencies,
//                    ..
//                } = entry;
//                let yarn_lock_entry = YarnLockEntry {
//                    version: version.to_string(),
//                    resolved: None,
//                    integrity: integrity.map(|s| s.to_string()),
//                    dependencies: Some(
//                        dependencies
//                            .into_iter()
//                            .map(|(k, v)| (k.to_string(), v.to_string()))
//                            .collect(),
//                    ),
//                };
//                (name.to_string(), yarn_lock_entry)
//            })
//            .collect();
//
//        Ok(YarnLock { entries })
//    }
//
//    #[test]
//    fn test_parse_yarn_lock_valid_file() {
//        let yarn_lock_content = r#"
//"@babel/code-frame@^7.0.0":
//  version "7.16.7"
//  resolved "https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.16.7.tgz"
//  integrity sha512-...
//  dependencies:
//    "@babel/highlight" "^7.0.0"
//"#;
//
//        let result = parse_yarn_lock_from_str(yarn_lock_content);
//        dbg!(&result);
//
//        assert!(result.is_ok(), "Failed to parse yarn.lock: {:?}", result);
//
//        let yarn_lock = result.unwrap();
//        assert_eq!(yarn_lock.entries.len(), 1);
//
//        let entry = yarn_lock.entries.get("@babel/code-frame@^7.0.0").unwrap();
//        assert_eq!(entry.version, "7.16.7");
//        assert_eq!(
//            entry.resolved.as_deref(),
//            Some("https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.16.7.tgz")
//        );
//        assert!(entry.integrity.is_some());
//        assert_eq!(
//            entry.dependencies.as_ref().unwrap().get("@babel/highlight"),
//            Some(&"^7.0.0".to_string())
//        );
//    }
//
//    #[test]
//    fn test_parse_yarn_lock_empty_file() {
//        let yarn_lock_content = "";
//
//        let result = parse_yarn_lock_from_str(yarn_lock_content);
//        dbg!(&result);
//
//        assert!(result.is_ok(), "Failed to parse empty yarn.lock: {:?}", result);
//
//        let yarn_lock = result.unwrap();
//        assert!(yarn_lock.entries.is_empty());
//    }
//
//    #[test]
//    fn test_parse_yarn_lock_invalid_format() {
//        let yarn_lock_content = r#"
//invalid_entry
//  version "1.0.0"
//"#;
//
//        let result = parse_yarn_lock_from_str(yarn_lock_content);
//        dbg!(&result);
//
//        assert!(result.is_err(), "Expected error for invalid yarn.lock format");
//
//        let error_message = result.unwrap_err().to_string();
//        assert!(error_message.contains("Failed to parse yarn.lock"));
//    }
//
//    #[test]
//    fn test_parse_yarn_lock_multiple_entries() {
//        let yarn_lock_content = r#"
//"@babel/code-frame@^7.0.0":
//  version "7.16.7"
//  resolved "https://registry.yarnpkg.com/@babel/code-frame/-/code-frame-7.16.7.tgz"
//  integrity sha512-...
//  dependencies:
//    "@babel/highlight" "^7.0.0"
//
//"@babel/highlight@^7.0.0":
//  version "7.18.6"
//  resolved "https://registry.yarnpkg.com/@babel/highlight/-/highlight-7.18.6.tgz"
//  integrity sha512-...
//"#;
//
//        let result = parse_yarn_lock_from_str(yarn_lock_content);
//        dbg!(&result);
//
//        assert!(result.is_ok(), "Failed to parse yarn.lock with multiple entries: {:?}", result);
//
//        let yarn_lock = result.unwrap();
//        assert_eq!(yarn_lock.entries.len(), 2);
//
//        let entry1 = yarn_lock.entries.get("@babel/code-frame@^7.0.0").unwrap();
//        assert_eq!(entry1.version, "7.16.7");
//
//        let entry2 = yarn_lock.entries.get("@babel/highlight@^7.0.0").unwrap();
//        assert_eq!(entry2.version, "7.18.6");
//    }
//}
//
