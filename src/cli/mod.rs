pub mod init;

use clap::{Parser, Subcommand};
use crate::cli::init::InitArgs;

#[derive(Parser, Debug)]
#[command(name = "xg")]
#[command(version = "0.1.0")]
#[command(about = "Project generator CLI")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new project
    Init(InitArgs),
}
