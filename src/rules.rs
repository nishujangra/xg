use std::path::Path;

/// Default blocked file patterns
pub const DEFAULT_BLOCKED_PATTERNS: &[&str] = &[
    ".idea/",
    "node_modules/",
    "target/",
    ".env",
    ".DS_Store",
    ".vscode/",
];

/// Check if a file path matches any blocked pattern
pub fn is_file_blocked(file_path: &str) -> bool {
    for pattern in DEFAULT_BLOCKED_PATTERNS {
        if matches_pattern(file_path, pattern) {
            return true;
        }
    }
    false
}

/// Check if a file path matches a specific pattern
fn matches_pattern(file_path: &str, pattern: &str) -> bool {
    // Handle directory patterns (ending with /)
    if pattern.ends_with('/') {
        // Check if the file path starts with the pattern
        // or if any directory in the path matches the pattern
        let path_components: Vec<&str> = file_path.split('/').collect();
        let pattern_dir = &pattern[..pattern.len() - 1]; // Remove trailing /
        
        for component in path_components {
            if component == pattern_dir {
                return true;
            }
        }
        return false;
    }
    
    // Handle exact file patterns
    if pattern.starts_with('.') {
        // For hidden files, check if the file name matches exactly
        let path = Path::new(file_path);
        if let Some(file_name) = path.file_name() {
            if let Some(name_str) = file_name.to_str() {
                return name_str == pattern;
            }
        }
        return false;
    }
    
    // For other patterns, check if the file path contains the pattern
    file_path.contains(pattern)
}

/// Get all blocked files from a list of staged files
pub fn get_blocked_files(staged_files: &[String]) -> Vec<String> {
    staged_files
        .iter()
        .filter(|file| is_file_blocked(file))
        .cloned()
        .collect()
}

/// Get a formatted list of blocked files with their patterns
pub fn get_blocked_files_with_patterns(staged_files: &[String]) -> Vec<(String, String)> {
    let mut blocked_with_patterns = Vec::new();
    
    for file in staged_files {
        for pattern in DEFAULT_BLOCKED_PATTERNS {
            if matches_pattern(file, pattern) {
                blocked_with_patterns.push((file.clone(), pattern.to_string()));
                break; // Only add once per file
            }
        }
    }
    
    blocked_with_patterns
}