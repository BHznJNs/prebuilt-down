use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

const DEFAULT_CACHE_DIR: &str = ".prebuilt-down";

// --- --- --- --- --- ---

pub struct CacheManager {
    pub dir: PathBuf,
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
        }));
    }

    pub fn path_for(&self, filename: &str) -> PathBuf {
        self.dir.join(filename)
    }
}
