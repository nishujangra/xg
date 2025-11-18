use clap::Parser;

pub mod cli;
pub mod commands;


use crate::cli::Cli;


fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::Init(args) => {
            commands::init_handler::handle_init(args);
        }
    }
}