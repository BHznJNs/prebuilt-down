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

#[cfg(test)]
mod tests {
    use super::PathExt;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn collect_files_returns_relative_paths() {
        let dir = tempdir().unwrap();
        let root = dir.path();

        fs::create_dir_all(root.join("a/b")).unwrap();
        fs::write(root.join("a/file1.txt"), "1").unwrap();
        fs::write(root.join("a/b/file2.txt"), "2").unwrap();

        let mut files = root.collect_files().unwrap();
        files.sort();

        let mut expected = vec![PathBuf::from("a/file1.txt"), PathBuf::from("a/b/file2.txt")];
        expected.sort();

        assert_eq!(files, expected);
    }
}
