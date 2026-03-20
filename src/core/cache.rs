use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::types::platform::Platform;

const DEFAULT_CACHE_DIR: &str = ".prebuilt-down";
const DEFAULT_LOCKFILE_NAME: &str = "prebuilt-down.lock";

#[derive(Debug, Serialize, Deserialize)]
pub struct LockFile {
    #[serde(flatten)]
    pub entries: HashMap<String, HashMap<String, LockEntry>>,
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

    pub fn is_locked(&self, platform: Platform) {
        todo!()
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

    pub fn check_exists(name: &str, platform: Platform) -> Result<bool> {
        let file_name = format!("{name}-{platform}.json");
        return Ok(true);
    }
}
