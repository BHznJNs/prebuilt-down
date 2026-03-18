use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;

use crate::platform::Platform;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
    Blake3,
}

#[derive(Debug, Deserialize)]
pub struct HashConfig {
    pub algorithm: HashAlgorithm,
    pub digest: String,
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "kebab-case")]
pub enum ArchiveType {
    TarGz,
    TarXz,
    Zip,
}

#[derive(Debug, Deserialize)]
pub struct PlatformEntry {
    pub url: String,
    pub root: String,
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
