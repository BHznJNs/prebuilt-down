use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use std::fs::{self, File};
use std::path::Path;
use tar::Archive;
use xz2::read::XzDecoder;
use zip::ZipArchive;

use crate::config::ArchiveType;

pub trait ExtractArchive {
    fn extract(&self, archive_path: &Path, target_dir: &Path) -> Result<()>;
}

impl ExtractArchive for ArchiveType {
    fn extract(&self, archive_path: &Path, target_dir: &Path) -> Result<()> {
        fs::create_dir_all(target_dir).context("failed to create target directory")?;
        match self {
            ArchiveType::Zip => extract_zip(archive_path, target_dir),
            ArchiveType::TarGz => extract_tar_gz(archive_path, target_dir),
            ArchiveType::TarXz => extract_tar_xz(archive_path, target_dir),
        }
    }
}

fn extract_zip(archive_path: &Path, target_dir: &Path) -> Result<()> {
    let file = File::open(archive_path).context("failed to open zip archive")?;
    let mut archive = ZipArchive::new(file).context("failed to read zip archive")?;
    archive
        .extract(target_dir)
        .context("failed to extract zip archive")?;
    Ok(())
}

fn extract_tar_gz(archive_path: &Path, target_dir: &Path) -> Result<()> {
    let file = File::open(archive_path).context("failed to open tar.gz archive")?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive
        .unpack(target_dir)
        .context("failed to extract tar.gz archive")?;
    Ok(())
}

fn extract_tar_xz(archive_path: &Path, target_dir: &Path) -> Result<()> {
    let file = File::open(archive_path).context("failed to open tar.xz archive")?;
    let decoder = XzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive
        .unpack(target_dir)
        .context("failed to extract tar.xz archive")?;
    Ok(())
}
