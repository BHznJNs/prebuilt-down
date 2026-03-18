mod cli;
mod config;
mod platform;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let platform = platform::Platform::current();
    println!("Current platform: {}", platform);

    let configs = config::load_configs(&cli.config)?;

    return Ok(());
}
