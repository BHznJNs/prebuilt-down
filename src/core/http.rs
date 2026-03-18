use anyhow::{Context, Result};
use reqwest::blocking::Client;
use reqwest::redirect::Policy;
use std::fs::{self, File};
use std::io::copy;
use std::path::{Path, PathBuf};

const REDIRECT_LIMIT: usize = 10;
const DEFAULT_DOWNLOAD_DIR: &str = ".prebuilt-down";

fn build_client() -> Result<Client> {
    let client = Client::builder()
        .redirect(Policy::limited(REDIRECT_LIMIT))
        .build()
        .context("failed to build http client")?;
    return Ok(client);
}

pub fn init_download_dir(path: Option<&Path>) -> Result<PathBuf> {
    let path = path.unwrap_or_else(|| Path::new(DEFAULT_DOWNLOAD_DIR));
    fs::create_dir_all(path)?;
    let gitignore_path = Path::new(path).join(".gitignore");
    fs::write(&gitignore_path, "*\n")?;
    return Ok(path.to_path_buf());
}

pub fn download_to(url: &str, path: &Path) -> Result<()> {
    let client = build_client()?;
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent).context("failed to create download directory")?;
        }
    }

    let mut response = client
        .get(url)
        .send()
        .context("failed to send download request")?
        .error_for_status()
        .context("http status error during download")?;

    let mut file = File::create(path).context("failed to create download file")?;
    copy(&mut response, &mut file).context("failed to write download file")?;
    return Ok(());
}
