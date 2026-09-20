mod app;
mod cli;
mod config;
mod core;
mod traits;
mod types;

use anyhow::{Result, anyhow};
use clap::Parser;

use app::App;
use cli::Cli;
use types::platform::Platform;

use crate::core::{
    cache::CacheManager,
    download::DownloadManager,
    lock_file::{DEFAULT_LOCKFILE_NAME, LockFile},
};

fn init_logger(verbose: u8) {
    let level = match verbose {
        0 => tracing::Level::WARN,
        1 => tracing::Level::INFO,
        2 => tracing::Level::DEBUG,
        _ => tracing::Level::TRACE,
    };

    tracing_subscriber::fmt().with_max_level(level).init();
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    init_logger(cli.verbose);

    let platform = cli.platform.unwrap_or_else(Platform::current);
    let configs = config::load_configs(&cli.config)?;

    let cache_manager = CacheManager::init(None)?;
    let download_manager = DownloadManager::init(cache_manager.clone())?;
    let lock_file = LockFile::load(&cache_manager.path_for(DEFAULT_LOCKFILE_NAME))?;
    let mut app = App::new(platform, lock_file, download_manager, cache_manager);

    let mut failed = false;
    for config in configs {
        let name = config.name.clone();
        if let Err(e) = app.process_config(config, cli.force) {
            failed = true;
            tracing::error!("Failed to process `{}`:\n{e:#}", name);
        }
    }
    app.save()?;

    if failed {
        return Err(anyhow!("one or more configs failed to process"));
    }

    Ok(())
}
