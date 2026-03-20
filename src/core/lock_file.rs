use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{Config, PlatformEntry};
use crate::traits::path_ext::PathExt;
use crate::types::platform::Platform;

pub const DEFAULT_LOCKFILE_NAME: &str = "prebuilt-down.lock";

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    #[serde(flatten)]
    pub entries: HashMap<String, HashMap<Platform, LockEntry>>,
}

impl LockFile {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self {
                entries: HashMap::new(),
            });
        }
        let content = fs::read_to_string(path)?;
        let lockfile = serde_json::from_str(&content)?;
        return Ok(lockfile);
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        fs::write(path, content)?;
        return Ok(());
    }

    pub fn lock(
        &mut self,
        name: &str,
        files: Vec<PathBuf>,
        platform: Platform,
        platform_config: &PlatformEntry,
    ) {
        let entry = LockEntry {
            url: platform_config.url.clone(),
            digest: platform_config
                .hash
                .as_ref()
                .map(|hash_config| hash_config.digest.clone()),
            files,
        };
        self.entries
            .entry(name.to_string())
            .or_insert_with(HashMap::new)
            .insert(platform, entry);
    }

    pub fn is_locked(&self, platform: Platform, config: &Config) -> bool {
        let Some(entry) = self
            .entries
            .get(&config.name)
            .and_then(|platforms| platforms.get(&platform))
        else {
            return false;
        };

        let Some(platform_config) = config.platforms.get(&platform) else {
            return false;
        };

        // compare url
        if entry.url != platform_config.url {
            return false;
        }

        // compare hash
        if let Some(ref hash_config) = platform_config.hash {
            let expected = hash_config.digest.trim().to_ascii_lowercase();
            let actual = match entry.digest.as_ref() {
                Some(digest) => digest.trim().to_ascii_lowercase(),
                None => return false,
            };
            if expected != actual {
                return false;
            }
        }

        if !config.target.exists() {
            return false;
        }
        let Ok(actual_files) = config.target.collect_files() else {
            eprintln!("Failed to read directory {}.", config.target.display());
            return false;
        };
        let mut expected_files = entry.files.clone();
        let mut actual_files = actual_files;
        expected_files.sort();
        actual_files.sort();
        expected_files == actual_files
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LockEntry {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    pub files: Vec<PathBuf>,
}
