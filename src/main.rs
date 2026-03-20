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

use crate::core::cache::CacheManager;
use crate::core::download::DownloadManager;

fn main() -> Result<()> {
    let cli = Cli::parse();
    let platform = cli.platform.unwrap_or_else(Platform::current);
    let configs = config::load_configs(&cli.config)?;

    let cache_manager = CacheManager::init(None)?;
    let download_manager = DownloadManager::init(cache_manager.clone())?;
    let app = App::new(platform, download_manager, cache_manager);

    for config in configs.into_iter() {
        app.process_config(config)?;
    }
    return Ok(());
}
