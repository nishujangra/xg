pub mod create_go_app;

use crate::{cli::create_go_app::GoArgs};
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
    /// Create a new Go application
    #[command(name = "create-go-app")]
    CreateGoApp(GoArgs),
}