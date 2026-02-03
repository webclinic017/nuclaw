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
