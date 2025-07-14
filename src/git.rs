use std::process::Command;
use anyhow::{Result, Context};


/// Check if the current directory is a git repository
pub fn is_git_repository() -> Result<bool> {
    let output = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .context("Failed to execute git command")?;
    
    Ok(output.status.success())
}

/// Get the current git status in porcelain format
pub fn get_git_status() -> Result<String> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("Failed to execute git status command")?;
    
    if !output.status.success() {
        anyhow::bail!("Git status command failed");
    }
    
    let status = String::from_utf8(output.stdout)
        .context("Failed to parse git status output")?;
    
    Ok(status)
}

/// Parse git status porcelain output to get staged files
pub fn get_staged_files() -> Result<Vec<String>> {
    let status_output = get_git_status()?;
    let mut staged_files = Vec::new();
    
    for line in status_output.lines() {
        if line.is_empty() {
            continue;
        }
        
        // Parse porcelain format: XY PATH
        // X = status of index, Y = status of working tree
        if line.len() < 3 {
            continue;
        }
        
        let status = &line[0..2];
        let file_path = &line[3..];
        
        // Check if file is staged (X is not space)
        if status.chars().next().unwrap() != ' ' {
            staged_files.push(file_path.to_string());
        }
    }
    
    Ok(staged_files)
}


/// Get remote URL for a given remote name
pub fn get_remote_url(remote_name: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["remote", "get-url", remote_name])
        .output()
        .context(format!("Failed to get remote URL for {}", remote_name))?;
    
    if !output.status.success() {
        anyhow::bail!("Failed to get remote URL for {}", remote_name);
    }
    
    let url = String::from_utf8(output.stdout)
        .context("Failed to parse remote URL")?
        .trim()
        .to_string();
    
    Ok(url)
}

/// Check if a remote URL uses HTTPS
pub fn is_https_remote(url: &str) -> bool {
    url.starts_with("https://")
}