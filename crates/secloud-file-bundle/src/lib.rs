//! Validated file-set bundle rules for StealthEye Cloud S1.

pub const FORBIDDEN_PATH_FRAGMENTS: &[&str] = &[
    "..",
    ".git",
    ".env",
    "secrets/",
    "CLAUDE.md",
    ".cursorrules",
    "soul.md",
    "MEMORY.md",
    "rules.md",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn validate_repo_relative_path(path: &str) -> PathValidation {
    let mut errors = Vec::new();

    if path.trim().is_empty() {
        errors.push("path is empty".to_string());
    }
    if path.starts_with('/') || path.starts_with('\\') {
        errors.push("path must be repo-relative".to_string());
    }
    if path.contains(':') {
        errors.push("path must not contain a drive or URI separator".to_string());
    }
    for fragment in FORBIDDEN_PATH_FRAGMENTS {
        if path.contains(fragment) {
            errors.push(format!("path contains forbidden fragment: {fragment}"));
        }
    }

    PathValidation {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_repo_relative_paths() {
        assert!(validate_repo_relative_path("docs/MISSION_EXECUTOR.md").valid);
    }

    #[test]
    fn rejects_absolute_windows_path() {
        let result = validate_repo_relative_path("X:\\repo\\file.txt");
        assert!(!result.valid);
    }

    #[test]
    fn rejects_traversal() {
        let result = validate_repo_relative_path("../secret.txt");
        assert!(!result.valid);
    }
}
