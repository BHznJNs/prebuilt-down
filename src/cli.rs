use super::platform::Platform;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "prebuilt-down")]
#[command(about = "Download and install prebuilt binaries defined in a config file")]
pub struct Cli {
    #[arg(short, long, default_value = "prebuilt-down.toml")]
    pub config: PathBuf,
    #[arg(short, long)]
    pub platform: Option<Platform>,
}
