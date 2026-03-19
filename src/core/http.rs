use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};

use crate::traits::resolve_filename::ResponseExt;

const REDIRECT_LIMIT: usize = 10;
const DEFAULT_DOWNLOAD_DIR: &str = ".prebuilt-down";

fn build_client() -> Result<Client> {
    let client = Client::builder()
        .redirect(Policy::limited(REDIRECT_LIMIT))
        .build()
        .context("failed to build http client")?;
    return Ok(client);
}

pub struct DownloadManager {
    dir: PathBuf,
    client: Client,
}

impl DownloadManager {
    pub fn initialize(dir: PathBuf) -> Result<Self> {
        let client = build_client()?;
        return Ok(Self { dir, client });
    }

    /// init download directory and create a .gitignore file within it,
    pub fn init_download_dir(path: Option<&Path>) -> Result<PathBuf> {
        let path = path.unwrap_or_else(|| Path::new(DEFAULT_DOWNLOAD_DIR));
        fs::create_dir_all(path)?;
        let gitignore_path = path.join(".gitignore");
        fs::write(&gitignore_path, "*\n")?;
        return Ok(path.to_path_buf());
    }

    pub fn download(&self, url: &str, fallback_filename: &str) -> Result<PathBuf> {
        let mut response = self.client.get(url).send()?.error_for_status()?;

        let filename = response.resolve_filename(fallback_filename);
        let download_path = self.dir.join(filename);
        let mut file = File::create(&download_path)?;
        copy(&mut response, &mut file)?;
        return Ok(download_path);
    }
}
