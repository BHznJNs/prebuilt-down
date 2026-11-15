use anyhow::{Context, Result};
use reqwest::blocking::Client as ReqwestClient;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::redirect::Policy as ReqwestRedirectPolicy;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::sync::Arc;

use crate::core::cache::CacheManager;
use crate::traits::response_ext::ResponseExt;

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

    pub fn resolve_authorization_headers(
        authorization_headers: &HashMap<String, String>,
        resolve_value: impl Fn(&str) -> Result<String>,
    ) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        for (name, template) in authorization_headers {
            let header_name = HeaderName::from_bytes(name.as_bytes())
                .with_context(|| format!("invalid authorization header name `{name}`"))?;
            let value = resolve_value(template)
                .with_context(|| format!("failed to resolve authorization header `{name}`"))?;
            let mut header_value = HeaderValue::from_str(&value)
                .with_context(|| format!("invalid value for authorization header `{name}`"))?;
            header_value.set_sensitive(true);
            headers.insert(header_name, header_value);
        }
        return Ok(headers);
    }

    pub fn download(
        &self,
        url: &str,
        fallback_filename: &str,
        headers: HeaderMap,
    ) -> Result<PathBuf> {
        let mut response = self
            .client
            .get(url)
            .headers(headers)
            .send()?
            .error_for_status()?;

        let filename = response.resolve_filename(fallback_filename);
        let download_path = self.cache.path_for(&filename);
        let mut file = File::create(&download_path)?;
        io::copy(&mut response, &mut file)?;
        return Ok(download_path);
    }
}
