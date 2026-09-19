use anyhow::Result;
use std::fs::File;
use std::path::{Path, PathBuf};

use xz2::read::XzDecoder;

use super::ArchiveExtractor;
use super::tar::TarExtractor;

pub struct TarXzExtractor {
    tar: TarExtractor<XzDecoder<File>>,
}

impl TarXzExtractor {
    pub fn new(path: PathBuf, root: PathBuf) -> Result<Self> {
        let file = File::open(&path)?;
        let decoder = XzDecoder::new(file);
        Ok(Self {
            tar: TarExtractor::new(decoder, path, root),
        })
    }
}

impl ArchiveExtractor for TarXzExtractor {
    fn extract(&mut self, target_dir: &Path) -> Result<Vec<PathBuf>> {
        self.tar.extract(target_dir)
    }
}
