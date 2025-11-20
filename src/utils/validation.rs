
pub fn is_project_name_valid(name: &str) -> bool {
    // Must not contain path separtiors
    if name.contains('/') || name.contains('\\') {
        return false;
    }

    // Must not contain traversal sequence
    if name.contains("..") {
        return false;
    }

    // Must not be empty
    if name.trim().is_empty() {
        return false;
    } 

    // allowed characters only
    if !name.chars().all(|ch| ch.is_alphanumeric() || ch == '_' || ch == '-') {
        return false;
    }

    true
}