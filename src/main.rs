mod cli;
mod config;
mod core;
mod platform;
mod traits;

use anyhow::{Context, Result, bail};
use clap::Parser;
use std::path::Path;

use cli::Cli;
use core::archive::ArchivePack;

fn process_config(
    download_dir: &Path,
    platform: platform::Platform,
    config: &config::Config,
) -> Result<()> {
    let Some(platform_config) = config.inner.platforms.get(&platform) else {
        eprintln!(
            "Warning: platform {} not configured for {}, skipping",
            platform, config.name
        );
        return Ok(());
    };

    let download_path = download_dir.join(&config.name);
    core::http::download_to(&platform_config.url, &download_path)
        .with_context(|| format!("Failed to download {}, skipping", config.name))?;

    if let Some(ref hash_config) = platform_config.hash {
        let verify_result = core::verify::verify_file(&download_path, hash_config)
            .with_context(|| format!("Failed to verify {}, skipping", config.name))?;
        if !verify_result {
            bail!(
                "Hash verification failed for {}: {} mismatch, skipping",
                config.name,
                hash_config.algorithm
            );
        }
    }

    if let Some(archive_type) = platform_config.archive {
        ArchivePack::new(
            archive_type,
            download_path.clone(),
            platform_config.root.clone(),
        )
        .extract(&config.inner.target)
        .with_context(|| format!("failed to extract {}", download_path.display()))?;
    } else {
        eprintln!(
            "Warning: archive type not specified for {}, skipping extraction",
            config.name
        );
    }
    return Ok(());
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let platform = cli.platform.unwrap_or_else(platform::Platform::current);

    let configs = config::load_configs(&cli.config)?;
    let download_dir = core::http::init_download_dir(None)?;

    for config in configs {
        process_config(&download_dir, platform, &config)?;
    }
    return Ok(());
}
