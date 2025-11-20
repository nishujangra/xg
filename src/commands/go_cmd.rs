use std::process::Command;
use std::io;

fn get_git_username() -> Option<String> {
    // Try local username
    let local = Command::new("git")
        .args(["config", "user.name"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !s.is_empty() { 
                Some(s) 
            } else { 
                None 
            }
        });

    if local.is_some() {
        return local;
    }

    // Try global username
    Command::new("git")
        .args(["config", "--global", "user.name"])
        .output()
        .ok()
        .and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if !s.is_empty() { 
                Some(s) 
            } else { 
                None 
            }
        })
}

pub fn run_go_mod_init(project_name: &str) -> io::Result<()> {
    // Try getting Git username
    let module_name = if let Some(username) = get_git_username() {
        format!("github.com/{username}/{project_name}")
    } else {
        project_name.to_string()
    };

    println!("👉 Using module name: {}", module_name);

    // Run: go mod init <module_name>
    let status = Command::new("go")
        .args(["mod", "init", &module_name])
        .current_dir(project_name)
        .status()?;

    if !status.success() {
        eprintln!("Failed to run `go mod init`");
    }

    Ok(())
}

pub fn run_go_mod_tidy(project_name: &str) -> io::Result<()> {
    let status = Command::new("go")
        .args(["mod", "tidy"])
        .current_dir(project_name)
        .status()?;

    if !status.success() {
        eprintln!("Failed to run `go mod tidy`");
    }

    Ok(())
}