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

        for entry in self.archive.entries()? {
            let mut entry = entry?;
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
                tar::EntryType::Symlink => {
                    #[cfg(unix)]
                    if let Some(target) = entry.link_name()? {
                        if let Some(parent) = out_path.parent() {
                            fs::create_dir_all(parent)?;
                        }
                        std::os::unix::fs::symlink(&*target, &out_path)?;
                        extracted.push(relative);
                    }

                    #[cfg(windows)]
                    anyhow::bail!(
                        "symbolic links in tar archives are not supported on Windows: {}",
                        entry.path()?.display()
                    );
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use tar::{Builder, EntryType, Header};
    use tempfile::tempdir;

    use super::{ArchiveExtractor, TarExtractor};

    fn archive_with_symlink() -> Vec<u8> {
        let mut builder = Builder::new(Vec::new());
        let mut header = Header::new_gnu();
        header.set_entry_type(EntryType::Symlink);
        header.set_size(0);
        builder
            .append_link(&mut header, "root/bin/tool", "tool-real")
            .unwrap();
        builder.into_inner().unwrap()
    }

    #[cfg(unix)]
    #[test]
    fn creates_parent_directory_for_symlink() {
        let archive = archive_with_symlink();
        let output = tempdir().unwrap();
        let mut extractor = TarExtractor::new(
            Cursor::new(archive),
            "test.tar".into(),
            "root".into(),
        );

        let extracted = extractor.extract(output.path()).unwrap();

        assert_eq!(extracted, vec![std::path::PathBuf::from("bin/tool")]);
        assert_eq!(
            std::fs::read_link(output.path().join("bin/tool")).unwrap(),
            std::path::PathBuf::from("tool-real")
        );
    }

    #[cfg(windows)]
    #[test]
    fn rejects_symlink_on_windows() {
        let archive = archive_with_symlink();
        let output = tempdir().unwrap();
        let mut extractor = TarExtractor::new(
            Cursor::new(archive),
            "test.tar".into(),
            "root".into(),
        );

        let error = extractor.extract(output.path()).unwrap_err();

        assert!(
            error
                .to_string()
                .contains("symbolic links in tar archives are not supported on Windows")
        );
    }
}
