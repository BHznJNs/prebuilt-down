mod cli;
mod config;
mod platform;

use anyhow::Result;
use clap::Parser;

use crate::cli::Cli;
use crate::http::build_http_client;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let platform = platform::Platform::current();
    println!("Current platform: {}", platform);

    let config = config::load_config(&cli.config)?;
    let client = build_http_client()?;

    return Ok(());
}
