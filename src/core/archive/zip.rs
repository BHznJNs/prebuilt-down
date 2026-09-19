use anyhow::Result;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use zip::ZipArchive;

use super::ArchiveExtractor;

pub struct ZipExtractor {
    path: PathBuf,
    root: PathBuf,
}

impl ZipExtractor {
    pub fn new(path: PathBuf, root: PathBuf) -> Self {
        Self { path, root }
    }
}

impl ArchiveExtractor for ZipExtractor {
    fn extract(&mut self, target_dir: &Path) -> Result<Vec<PathBuf>> {
        let file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(file)?;
        let mut extracted = Vec::new();

        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let Some(entry_path) = entry.enclosed_name() else {
                tracing::warn!(
                    "Encountered an unsafe entry path `{}`, skipping it.",
                    entry.name()
                );
                continue;
            };

            let relative = match entry_path.strip_prefix(&self.root) {
                Ok(r) => r.to_path_buf(),
                Err(_) => continue,
            };
            if relative == Path::new("") {
                continue;
            }
            let out_path = target_dir.join(&relative);

            if entry.is_dir() {
                fs::create_dir_all(&out_path)?;
                continue;
            }
            if let Some(parent) = out_path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut out_file = File::create(&out_path)?;
            io::copy(&mut entry, &mut out_file)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = entry.unix_mode() {
                    fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
                }
            }
            extracted.push(relative);
        }

        if extracted.is_empty() {
            tracing::warn!(
                "Root path `{}` for archive `{}` not found.",
                self.root.display(),
                self.path.display()
            );
        }

        Ok(extracted)
    }
}
