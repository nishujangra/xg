
use clap::{Parser, Subcommand};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(version, about = "xg(ex-gee) - Safe Git with superpowers")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Show git status of the current repository
    Status,
}


fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Status => {
            let output = Command::new("git")
                .arg("status")
                .output()
                .expect("failed, it git installed?");

            if !output.status.success() {
                eprintln!("git status failed");
                std::process::exit(1);
            }

            // Print stdout (the actual git status)
            println!("{}", String::from_utf8_lossy(&output.stdout));
        }
    }
}