use anyhow::Result;
use std::fs::File;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;

use super::ArchiveExtractor;
use super::tar::TarExtractor;

pub struct TarGzExtractor {
    tar: TarExtractor<GzDecoder<File>>,
}

impl TarGzExtractor {
    pub fn new(path: PathBuf, root: PathBuf) -> Result<Self> {
        let file = File::open(&path)?;
        let decoder = GzDecoder::new(file);
        Ok(Self {
            tar: TarExtractor::new(decoder, path, root),
        })
    }
}

impl ArchiveExtractor for TarGzExtractor {
    fn extract(&mut self, target_dir: &Path) -> Result<Vec<PathBuf>> {
        self.tar.extract(target_dir)
    }
}
