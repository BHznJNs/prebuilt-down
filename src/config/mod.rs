mod loader;

pub use loader::load_configs;

use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::types::{archive::ArchiveType, hash::HashAlgorithm, platform::Platform};

#[derive(Debug, Deserialize)]
pub struct HashConfig {
    pub algorithm: HashAlgorithm,
    pub digest: String,
}

#[derive(Debug, Deserialize)]
pub struct PlatformEntry {
    pub url: String,
    pub root: PathBuf,
    #[serde(default)]
    pub hash: Option<HashConfig>,
    #[serde(default)]
    pub archive: Option<ArchiveType>,
}

#[derive(Debug, Deserialize)]
pub struct PrebuiltConfig {
    pub target: PathBuf,
    #[serde(flatten)]
    pub platforms: HashMap<Platform, PlatformEntry>,
}

pub type ConfigMap = HashMap<String, PrebuiltConfig>;

#[derive(Debug)]
pub struct Config {
    pub name: String,
    pub inner: PrebuiltConfig,
}
