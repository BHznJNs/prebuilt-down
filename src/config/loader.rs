use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::types::Config;

pub fn load_config(path: &Path) -> Result<Config> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    let config = toml::from_str(&content)
        .with_context(|| format!("failed to parse config toml: {}", path.display()))?;
    return Ok(config);
}
