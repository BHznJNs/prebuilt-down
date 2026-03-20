use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::config::PrebuiltConfig;
use crate::traits::path_ext::PathExt;
use crate::types::platform::Platform;

const DEFAULT_CACHE_DIR: &str = ".prebuilt-down";
const DEFAULT_LOCKFILE_NAME: &str = "prebuilt-down.lock";

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

    pub fn lock(&self, ...) {
        //
    }

    pub fn is_locked(&self, name: &str, platform: Platform, config: &PrebuiltConfig) -> bool {
        let Some(entry) = self
            .entries
            .get(name)
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

// --- --- --- --- --- ---

pub struct CacheManager {
    pub dir: PathBuf,
    pub lock_file: LockFile,
}

impl CacheManager {
    /// init cache directory and create a .gitignore file within it,
    pub fn init(path: Option<&Path>) -> Result<Arc<Self>> {
        let path = path.unwrap_or_else(|| Path::new(DEFAULT_CACHE_DIR));
        fs::create_dir_all(path)?;
        let gitignore_path = path.join(".gitignore");
        if !gitignore_path.exists() {
            fs::write(&gitignore_path, "*\n")?;
        }
        return Ok(Arc::new(Self {
            dir: path.to_path_buf(),
            lock_file: LockFile::load(&path.join(DEFAULT_LOCKFILE_NAME))?,
        }));
    }

    pub fn path_for(&self, filename: &str) -> PathBuf {
        self.dir.join(filename)
    }
}
