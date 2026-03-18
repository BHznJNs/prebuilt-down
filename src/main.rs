mod cli;
mod config;
mod core;
mod platform;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use crate::cli::Cli;
use crate::core::extract::ExtractArchive;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config_path = cli
        .config
        .unwrap_or_else(|| PathBuf::from("prebuilt-down.toml"));
    let platform = cli.platform.unwrap_or_else(platform::Platform::current);

    let configs = config::load_configs(&config_path)?;
    let download_dir = core::http::init_download_dir(None)?;
    for config in configs {
        let platform_config = &config.inner.platforms[&platform];
        let download_path = download_dir.join(&config.name);
        core::http::download_to(&platform_config.url, &download_path)?;
        if let Some(archive_type) = platform_config.archive {
            archive_type.extract(&download_path, &config.inner.target)?;
        } else {
            eprintln!(
                "Warning: archive type not specified for {}, skipping extraction",
                config.name
            );
        }
    }

    return Ok(());
}
