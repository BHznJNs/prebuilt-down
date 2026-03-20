use anyhow::{Context, Result};
use reqwest::blocking::Client as ReqwestClient;
use reqwest::redirect::Policy as ReqwestRedirectPolicy;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use crate::core::cache::CacheManager;
use crate::traits::resolve_filename::ResponseExt;

const REDIRECT_LIMIT: usize = 10;

fn build_client() -> Result<ReqwestClient> {
    let client = ReqwestClient::builder()
        .redirect(ReqwestRedirectPolicy::limited(REDIRECT_LIMIT))
        .build()
        .context("failed to build http client")?;
    return Ok(client);
}

pub struct DownloadManager {
    cache: Arc<CacheManager>,
    client: ReqwestClient,
}

impl DownloadManager {
    pub fn init(cache: Arc<CacheManager>) -> Result<Arc<Self>> {
        let client = build_client()?;
        return Ok(Arc::new(Self { cache, client }));
    }

    pub fn download(&self, url: &str, fallback_filename: &str) -> Result<PathBuf> {
        let mut response = self.client.get(url).send()?.error_for_status()?;

        let filename = response.resolve_filename(fallback_filename);
        let download_path = self.cache.path_for(&filename);
        let mut file = File::create(&download_path)?;
        io::copy(&mut response, &mut file)?;
        return Ok(download_path);
    }
}
