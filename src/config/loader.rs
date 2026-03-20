use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

use super::{Config, ConfigMap};

pub fn load_configs(path: &Path) -> Result<Vec<Config>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {}", path.display()))?;
    let config_map: ConfigMap = toml::from_str(&content)
        .with_context(|| format!("failed to parse config toml: {}", path.display()))?;
    let configs: Vec<Config> = config_map
        .into_iter()
        .map(|(name, p)| Config { name, inner: p })
        .collect();
    return Ok(configs);
}
