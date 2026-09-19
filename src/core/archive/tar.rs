use anyhow::Result;
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use tar::Archive;

use super::ArchiveExtractor;

pub struct TarExtractor<R: io::Read> {
    archive: Archive<R>,
    path: PathBuf,
    root: PathBuf,
}

impl<R: io::Read> TarExtractor<R> {
    pub fn new(reader: R, path: PathBuf, root: PathBuf) -> Self {
        Self {
            archive: Archive::new(reader),
            path,
            root,
        }
    }
}

impl<R: io::Read> ArchiveExtractor for TarExtractor<R> {
    fn extract(&mut self, target_dir: &Path) -> Result<Vec<PathBuf>> {
        let mut extracted = Vec::new();

        for mut entry in self.archive.entries()?.flatten() {
            let relative = match entry.path()?.strip_prefix(&self.root) {
                Ok(r) => r.to_path_buf(),
                Err(_) => continue,
            };

            // relative is empty means this entry *is* the `root` directory
            if relative == Path::new("") {
                continue;
            }

            let out_path = target_dir.join(&relative);

            match entry.header().entry_type() {
                tar::EntryType::Directory => {
                    fs::create_dir_all(&out_path)?;
                }
                tar::EntryType::Regular => {
                    if let Some(parent) = out_path.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut out_file = File::create(&out_path)?;
                    io::copy(&mut entry, &mut out_file)?;

                    // restore file permission
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        let mode = entry.header().mode()?;
                        fs::set_permissions(&out_path, fs::Permissions::from_mode(mode))?;
                    }
                    extracted.push(relative);
                }
                tar::EntryType::Symlink =>
                {
                    #[cfg(unix)]
                    if let Some(target) = entry.header().link_name()? {
                        std::os::unix::fs::symlink(&*target, &out_path)?;
                        extracted.push(relative);
                    }
                }
                _ => {}
            }
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
