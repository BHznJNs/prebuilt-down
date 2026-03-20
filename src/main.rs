mod app;
mod cli;
mod config;
mod core;
mod traits;
mod types;

use anyhow::Result;
use clap::Parser;

use app::App;
use cli::Cli;
use types::platform::Platform;

use crate::core::{
    cache::CacheManager,
    download::DownloadManager,
    lock_file::{DEFAULT_LOCKFILE_NAME, LockFile},
};

fn main() -> Result<()> {
    let cli = Cli::parse();
    let platform = cli.platform.unwrap_or_else(Platform::current);
    let configs = config::load_configs(&cli.config)?;

    let cache_manager = CacheManager::init(None)?;
    let download_manager = DownloadManager::init(cache_manager.clone())?;
    let lock_file = LockFile::load(&cache_manager.path_for(DEFAULT_LOCKFILE_NAME))?;
    let app = App::new(platform, lock_file, download_manager, cache_manager);

    for config in configs.into_iter() {
        app.process_config(config)?;
    }
    return Ok(());
}
