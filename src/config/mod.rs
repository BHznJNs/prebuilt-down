mod loader;
mod types;

pub use loader::load_config;
pub use types::{ArchiveType, HashAlgorithm, HashConfig, PlatformEntry, PrebuiltConfig};
