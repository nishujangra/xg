pub mod init;

use crate::cli::init::InitArgs;
use clap::{Parser, Subcommand};

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
