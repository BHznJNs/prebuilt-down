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

fn init_logger(verbose: u8) {
    let level = match cli.verbose {
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

    for config in configs.into_iter() {
        let name = config.name.clone();
        match app.process_config(config) {
            Ok(_) => {}
            Err(e) => eprintln!("Failed to process '{}':\n{e:#}", name),
        }
    }
    app.save()?;
    return Ok(());
}
