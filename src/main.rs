use clap::{Parser, Subcommand};
use std::process;

mod git;
mod rules;
use git::{is_git_repository, get_staged_files, get_remote_url, is_https_remote};
use rules::{get_blocked_files, get_blocked_files_with_patterns};

#[derive(Parser)]
#[command(
    name = "git-guard",
    about = "A lightweight Rust CLI tool to block accidental git push of unwanted files",
    version = "0.1.0",
    long_about = "Stop embarrassing commits before they leave your machine. Blocks git push if disallowed files like .idea/, node_modules/, .env, and more are staged."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Push to a remote repository with safety checks
    Push {
        /// The remote repository name (e.g., origin)
        remote: String,
        
        /// The branch name to push (e.g., main)
        branch: String,
        
        /// Additional git push arguments
        #[arg(trailing_var_arg = true)]
        git_args: Vec<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Push { remote, branch, git_args } => {
            if let Err(e) = handle_push(remote, branch, git_args) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
    }
}

fn handle_push(remote: String, branch: String, git_args: Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 git-guard: Checking staged files before push...");
    println!("📤 Target: {} -> {}", remote, branch);
    
    // Check if we're in a git repository
    if !is_git_repository()? {
        return Err(anyhow::anyhow!("Not in a git repository. Please run this command from a git repository.").into());
    }
    
    // Get staged files
    let staged_files = get_staged_files()?;
    println!("📋 Found {} staged files", staged_files.len());
    
    if !staged_files.is_empty() {
        println!("📁 Staged files:");
        for file in &staged_files {
            println!("   - {}", file);
        }
    }
    
    // Check for blocked files
    let blocked_files = get_blocked_files(&staged_files);
    if !blocked_files.is_empty() {
        println!("\n🚫 Push blocked! Found {} blocked file(s):", blocked_files.len());
        
        let blocked_with_patterns = get_blocked_files_with_patterns(&staged_files);
        for (file, pattern) in blocked_with_patterns {
            println!("   ❌ {} (blocked by pattern: {})", file, pattern);
        }
        
        println!("\n💡 To fix this:");
        println!("   1. Remove blocked files: git rm --cached <file>");
        println!("   2. Add to .gitignore to prevent future staging");
        println!("   3. Commit the changes");
        
        return Err(anyhow::anyhow!("Push blocked due to disallowed files").into());
    }
    
    // Check remote URL for HTTPS warning
    match get_remote_url(&remote) {
        Ok(url) => {
            if is_https_remote(&url) {
                println!("⚠️  You're using an HTTPS remote: {}", url);
                println!("👉 It's recommended to use SSH for pushing to Git remotes.");
                println!("   Example: git@github.com:user/repo.git");
                println!("   See: https://docs.github.com/en/authentication/connecting-to-github-with-ssh");
            }
        }
        Err(e) => {
            println!("⚠️  Warning: Could not get remote URL for '{}': {}", remote, e);
        }
    }
    
    // Execute the actual git push command
    println!("✅ All checks passed! Executing git push...");
    
    let mut push_command = std::process::Command::new("git");
    push_command.arg("push").arg(&remote).arg(&branch);
    
    // Add any additional git arguments
    for arg in git_args {
        push_command.arg(arg);
    }
    
    // Execute the push command
    let push_output = push_command.output()
        .map_err(|e| anyhow::anyhow!("Failed to execute git push: {}", e))?;
    
    // Print git's output
    if !push_output.stdout.is_empty() {
        print!("{}", String::from_utf8_lossy(&push_output.stdout));
    }
    if !push_output.stderr.is_empty() {
        eprint!("{}", String::from_utf8_lossy(&push_output.stderr));
    }
    
    // Check if push was successful
    if !push_output.status.success() {
        return Err(anyhow::anyhow!("Git push failed with exit code: {}", push_output.status).into());
    }
    
    println!("🚀 Successfully pushed to {}:{}", remote, branch);
    
    Ok(())
}
