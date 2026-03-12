use serde::Deserialize;
use std::collections::HashMap;

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
    pub target: String,
    #[serde(flatten)]
    pub platforms: HashMap<Platform, PlatformEntry>,
}

pub type Config = HashMap<String, PrebuiltConfig>;
