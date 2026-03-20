use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub trait PathExt {
    fn collect_files(&self) -> io::Result<Vec<PathBuf>>;
}

impl PathExt for Path {
    fn collect_files(&self) -> io::Result<Vec<PathBuf>> {
        fn inner(root: &Path, dir: &Path) -> io::Result<Vec<PathBuf>> {
            let mut out = Vec::new();
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if entry.metadata()?.is_dir() {
                    out.extend(inner(root, &path)?);
                } else {
                    let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
                    out.push(relative);
                }
            }
            Ok(out)
        }

        inner(self, self)
    }
}
