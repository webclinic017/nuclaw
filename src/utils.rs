//! Utilities for NuClaw

use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

pub mod json {
    use super::*;

    pub fn load_json<T>(path: &Path, default: T) -> T
    where
        T: for<'de> Deserialize<'de> + Default,
    {
        if !path.exists() {
            return default;
        }

        match File::open(path) {
            Ok(mut file) => {
                let mut contents = String::new();
                if file.read_to_string(&mut contents).is_ok() {
                    serde_json::from_str(&contents).unwrap_or(default)
                } else {
                    default
                }
            }
            Err(_) => default,
        }
    }

    pub fn save_json<T>(path: &Path, data: &T) -> std::io::Result<()>
    where
        T: Serialize,
    {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let json = serde_json::to_string_pretty(data)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::json::{load_json, save_json};
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[derive(Debug, Default, serde::Deserialize, serde::Serialize, Clone)]
    struct TestData {
        name: String,
        value: i32,
    }

    fn test_dir() -> PathBuf {
        tempdir().unwrap().into_path()
    }

    fn cleanup(path: &PathBuf) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn test_save_json() {
        let dir = test_dir();
        let path = dir.join("test.json");

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        let result = save_json(&path, &data);
        assert!(result.is_ok());
        assert!(path.exists());

        cleanup(&path);
    }

    #[test]
    fn test_load_json_existing() {
        let dir = test_dir();
        let path = dir.join("test.json");

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };
        save_json(&path, &data).unwrap();

        let loaded: TestData = load_json(&path, TestData::default());
        assert_eq!(loaded.name, "test");
        assert_eq!(loaded.value, 42);

        cleanup(&path);
    }

    #[test]
    fn test_load_json_nonexistent() {
        let dir = test_dir();
        let path = dir.join("nonexistent.json");

        let default = TestData {
            name: "default".to_string(),
            value: 0,
        };

        let loaded: TestData = load_json(&path, default.clone());
        assert_eq!(loaded.name, "default");
        assert_eq!(loaded.value, 0);
    }

    #[test]
    fn test_load_json_invalid() {
        let dir = test_dir();
        let path = dir.join("invalid.json");

        fs::write(&path, "not valid json").unwrap();

        let default = TestData {
            name: "default".to_string(),
            value: 0,
        };

        let loaded: TestData = load_json(&path, default.clone());
        assert_eq!(loaded.name, "default");
        assert_eq!(loaded.value, 0);

        cleanup(&path);
    }

    #[test]
    fn test_save_json_creates_parent() {
        let dir = test_dir();
        let path = dir.join("subdir").join("nested.json");

        let data = TestData {
            name: "nested".to_string(),
            value: 100,
        };

        let result = save_json(&path, &data);
        assert!(result.is_ok());
        assert!(path.exists());

        let _ = fs::remove_dir_all(dir);
    }
}
