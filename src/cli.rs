use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    /// Path to an alternate configuration file.
    #[clap(long)]
    pub config: Option<PathBuf>,
    /// Path to the socket to use for ipc. Takes precedence over the same option in the config file.
    #[clap(long)]
    pub socket: Option<PathBuf>,
}
