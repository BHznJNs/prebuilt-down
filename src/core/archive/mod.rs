use anyhow::Result;
use std::path::{Path, PathBuf};

use crate::types::archive::ArchiveType;

use self::tar_gz::TarGzExtractor;
use self::tar_xz::TarXzExtractor;
use self::zip::ZipExtractor;

mod tar;
mod tar_gz;
mod tar_xz;
mod zip;

pub trait ArchiveExtractor {
    fn extract(&mut self, target_dir: &Path) -> Result<Vec<PathBuf>>;
}

pub struct ArchivePackBuilder;

impl ArchivePackBuilder {
    pub fn build(
        kind: ArchiveType,
        path: impl Into<PathBuf>,
        root: impl Into<PathBuf>,
    ) -> Result<Box<dyn ArchiveExtractor>> {
        let path = path.into();
        let mut root = root.into();
        if root == Path::new(".") {
            root = PathBuf::new();
        }

        let extractor: Box<dyn ArchiveExtractor> = match kind {
            ArchiveType::Zip => Box::new(ZipExtractor::new(path, root)),
            ArchiveType::TarGz => Box::new(TarGzExtractor::new(path, root)?),
            ArchiveType::TarXz => Box::new(TarXzExtractor::new(path, root)?),
        };
        Ok(extractor)
    }
}
