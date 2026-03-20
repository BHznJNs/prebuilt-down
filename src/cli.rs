use clap::{ArgAction, Parser};
use std::path::PathBuf;

use crate::types::platform::Platform;

#[derive(Parser, Debug)]
#[command(name = "prebuilt-down")]
#[command(about = "Download and install prebuilt binaries defined in a config file")]
pub struct Cli {
    #[arg(short, long, default_value = "prebuilt-down.toml")]
    pub config: PathBuf,
    #[arg(short, long)]
    pub platform: Option<Platform>,
    #[arg(short, long, action = ArgAction::Count)]
    pub verbose: u8,
}
