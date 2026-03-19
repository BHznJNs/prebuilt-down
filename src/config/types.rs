use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

use crate::platform::Platform;

#[derive(Copy, Clone, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HashAlgorithm {
    Sha256,
    Sha512,
}

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Sha256 => write!(f, "SHA-256"),
            Self::Sha512 => write!(f, "SHA-512"),
        }
    }
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
