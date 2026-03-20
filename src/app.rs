use anyhow::{Context, Result, bail};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

use crate::config::Config;
use crate::core::{
    archive::ArchivePack, cache::CacheManager, download::DownloadManager, lock_file::LockFile,
    verify,
};
use crate::types::platform::Platform;

pub struct App {
    platform: Platform,
    lock_file: LockFile,
    download_manager: Arc<DownloadManager>,
    cache_manager: Arc<CacheManager>,
}

impl App {
    pub fn new(
        platform: Platform,
        lock_file: LockFile,
        download_manager: Arc<DownloadManager>,
        cache_manager: Arc<CacheManager>,
    ) -> Self {
        Self {
            platform,
            lock_file,
            download_manager,
            cache_manager,
        }
    }

    pub fn process_config(&self, config: Config) -> Result<()> {
        let Some(platform_config) = config.inner.platforms.get(&self.platform) else {
            eprintln!(
                "Warning: platform {} not configured for {}, skipping",
                self.platform, config.name
            );
            return Ok(());
        };

        if self
            .cache_manager
            .lock_file
            .is_locked(&config.name, self.platform, &config.inner)
        {
            return Ok(());
        }

        let downloaded_path = self
            .download_manager
            .download(&platform_config.url, &config.name)
            .with_context(|| format!("Failed to download {}, skipping", config.name))?;

        if let Some(ref hash_config) = platform_config.hash {
            let verify_result = verify::verify_file(&downloaded_path, hash_config)
                .with_context(|| format!("Failed to verify {}, skipping", config.name))?;
            if !verify_result {
                bail!(
                    "Hash verification failed for {}: {} mismatch, skipping",
                    config.name,
                    hash_config.algorithm
                );
            }
        }

        let output_files = if let Some(archive_type) = platform_config.archive {
            let extracted = ArchivePack::new(
                archive_type,
                downloaded_path.clone(),
                platform_config.root.clone(),
            )
            .extract(&config.inner.target)
            .with_context(|| format!("failed to extract {}", downloaded_path.display()))?;
            extracted
        } else {
            let target_path = &config.inner.target;
            fs::create_dir_all(target_path)?;

            let target_file_name = downloaded_path
                .file_name()
                .and_then(|name| name.to_str())
                .ok_or_else(|| anyhow::anyhow!("invalid download file name"))?;
            let target_file_path = target_path.join(target_file_name);
            fs::copy(&downloaded_path, &target_file_path)?;
            vec![PathBuf::from(target_file_name)]
        };

        self.cache_manager.lock_file.lock(
            &config.name,
            output_files,
            self.platform,
            platform_config,
        );

        return Ok(());
    }
}
