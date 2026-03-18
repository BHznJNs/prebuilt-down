use anyhow::Result;
use flate2::read::GzDecoder;
use std::fs::{File, create_dir_all};
use std::io;
use std::path::{Path, PathBuf};
use tar::Archive as TarArchive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::config::ArchiveType;

pub struct ArchivePack {
    kind: ArchiveType,
    path: PathBuf,
    root: PathBuf,
}

impl ArchivePack {
    pub fn new(kind: ArchiveType, path: impl Into<PathBuf>, root: impl Into<PathBuf>) -> Self {
        Self {
            kind,
            path: path.into(),
            root: root.into(),
        }
    }

    pub fn extract(&self, target_dir: &Path) -> Result<()> {
        create_dir_all(target_dir)?;
        match self.kind {
            ArchiveType::Zip => self.extract_zip(target_dir),
            ArchiveType::TarGz => self.extract_tar_gz(target_dir),
            ArchiveType::TarXz => self.extract_tar_xz(target_dir),
        }
    }

    fn extract_zip(&self, target_dir: &Path) -> Result<()> {
        let file = File::open(&self.path)?;
        let mut archive = ZipArchive::new(file)?;

        let mut extracted_count = 0;
        for i in 0..archive.len() {
            let mut entry = archive.by_index(i)?;
            let Some(entry_path) = entry.enclosed_name() else {
                eprintln!(
                    "Encountered an unsafe entry path {}, skipping it.",
                    entry.name()
                );
                continue;
            };

            let relative = match entry_path.strip_prefix(&self.root) {
                Ok(r) => r.to_path_buf(),
                Err(_) => continue,
            };
            let out_path = target_dir.join(&relative);
            extracted_count += 1;

            if entry.is_dir() {
                create_dir_all(&out_path)?;
                continue;
            }
            if let Some(parent) = out_path.parent() {
                create_dir_all(parent)?;
            }
            let mut out_file = File::create(&out_path)?;
            io::copy(&mut entry, &mut out_file)?;

            #[cfg(unix)]
            {
                // restore file permissions for UNIX
                use std::os::unix::fs::PermissionsExt;
                if let Some(mode) = entry.unix_mode() {
                    fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
                }
            }
        }

        if extracted_count == 0 {
            eprintln!(
                "Root path {} for archive {} not found.",
                self.root.display(),
                self.path.display()
            );
        }

        return Ok(());
    }

    fn extract_tar_gz(&self, target_dir: &Path) -> Result<()> {
        let tar_gz_file = File::open(&self.path)?;
        let tar = GzDecoder::new(tar_gz_file);
        let mut archive = TarArchive::new(tar);
        self.extract_tar(&mut archive, target_dir)?;
        return Ok(());
    }

    fn extract_tar_xz(&self, target_dir: &Path) -> Result<()> {
        let file = File::open(&self.path)?;
        let decoder = XzDecoder::new(file);
        let mut archive = TarArchive::new(decoder);
        self.extract_tar(&mut archive, target_dir)?;
        return Ok(());
    }

    fn extract_tar<R: io::Read>(
        &self,
        archive: &mut TarArchive<R>,
        target_dir: &Path,
    ) -> Result<()> {
        let mut extracted_count = 0;

        for mut entry in archive.entries()?.flatten() {
            let relative = match entry.path()?.strip_prefix(&self.root) {
                Ok(r) => r.to_path_buf(),
                Err(_) => continue,
            };

            // relative is empty means this entry *is* the `root` directory
            if relative == Path::new("") {
                continue;
            }

            let out_path = target_dir.join(&relative);
            extracted_count += 1;

            match entry.header().entry_type() {
                tar::EntryType::Directory => {
                    create_dir_all(&out_path)?;
                }
                tar::EntryType::Regular => {
                    if let Some(parent) = out_path.parent() {
                        create_dir_all(parent)?;
                    }
                    let mut out_file = File::create(&out_path)?;
                    io::copy(&mut entry, &mut out_file)?;

                    // 恢复文件权限
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mode = entry.header().mode()?;
                        fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
                    }
                }
                // symbol link
                tar::EntryType::Symlink =>
                {
                    #[cfg(unix)]
                    if let Some(target) = entry.header().link_name()? {
                        std::os::unix::fs::symlink(&*target, &out_path)?;
                    }
                }
                _ => {}
            }
        }

        if extracted_count == 0 {
            eprintln!(
                "Root path {} for archive {} not found.",
                self.root.display(),
                self.path.display()
            );
        }

        return Ok(());
    }
}
