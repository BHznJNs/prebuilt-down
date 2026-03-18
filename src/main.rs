mod cli;
mod config;
mod core;
mod platform;

use anyhow::{Context, Result};
use clap::Parser;

use crate::cli::Cli;
use crate::core::archive::ArchivePack;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let platform = cli.platform.unwrap_or_else(platform::Platform::current);

    let configs = config::load_configs(&cli.config)?;
    let download_dir = core::http::init_download_dir(None)?;
    for config in configs {
        let platform_config = &config.inner.platforms[&platform];
        let download_path = download_dir.join(&config.name);
        core::http::download_to(&platform_config.url, &download_path)?;
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
    }

    return Ok(());
}
