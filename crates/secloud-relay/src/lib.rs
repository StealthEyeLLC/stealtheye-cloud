//! StealthEye Relay validation for S0.

pub const REQUIRED_RELAY_SECTIONS: &[&str] = &[
    "## 1. Resume Command",
    "## 2. Current Mission",
    "## 3. Current State",
    "## 4. Latest Verified Result",
    "## 5. Active Approval Envelope",
    "## 6. Next Exact Action",
    "## 7. Decisions That Must Not Drift",
    "## 8. Do Not Reopen",
    "## 9. Open Questions / Boundaries",
    "## 10. Required Files / Repos / Branches",
    "## 11. Latest Seal",
    "## 12. Failure / Blocker State",
    "## 13. Codex / Worker State",
    "## 14. Browser State",
    "## 15. Public / Private Boundary",
];

pub const REQUIRED_RESUME_PHRASE: &str = "Resume this StealthEye Cloud mission from this Relay";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn validate_relay_markdown(content: &str) -> RelayValidation {
    let mut errors = Vec::new();

    if !content.contains(REQUIRED_RESUME_PHRASE) {
        errors.push("missing resume command phrase".to_string());
    }

    for section in REQUIRED_RELAY_SECTIONS {
        if !content.contains(section) {
            errors.push(format!("missing required section: {section}"));
        }
    }

    if !content.contains("STEALTHEYE_SEAL.json") {
        errors.push("missing latest Seal reference".to_string());
    }

    if !content.contains("Next Exact Action") {
        errors.push("missing next exact action".to_string());
    }

    RelayValidation {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_relay_requires_all_sections() {
        let mut content = String::from("# StealthEye Relay\n\n");
        for section in REQUIRED_RELAY_SECTIONS {
            content.push_str(section);
            content.push_str("\nplaceholder STEALTHEYE_SEAL.json Next Exact Action\n\n");
        }
        content.push_str(REQUIRED_RESUME_PHRASE);
        let result = validate_relay_markdown(&content);
        assert!(result.valid, "{:?}", result.errors);
    }

    #[test]
    fn invalid_relay_reports_missing_resume() {
        let result = validate_relay_markdown("# StealthEye Relay");
        assert!(!result.valid);
        assert!(result.errors.iter().any(|e| e.contains("resume")));
    }
}
