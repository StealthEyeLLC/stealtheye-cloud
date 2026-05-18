//! CI evidence cards for StealthEye Cloud S1.

pub const CI_EVIDENCE_CARD_SCHEMA: &str = "CiEvidenceCardV0";
pub const PR_EVIDENCE_CARD_SCHEMA: &str = "PrEvidenceCardV0";
pub const FAILURE_CARD_SCHEMA: &str = "FailureCardV0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceCardValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn validate_evidence_card_text(content: &str, schema_name: &str) -> EvidenceCardValidation {
    let mut errors = Vec::new();
    if !content.contains(schema_name) {
        errors.push(format!("missing schema marker: {schema_name}"));
    }
    if !content.contains("mission_id") {
        errors.push("missing mission_id marker".to_string());
    }
    if !content.contains("summary") {
        errors.push("missing summary marker".to_string());
    }
    EvidenceCardValidation {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evidence_card_requires_schema_marker() {
        let result = validate_evidence_card_text("mission_id summary", CI_EVIDENCE_CARD_SCHEMA);
        assert!(!result.valid);
    }

    #[test]
    fn evidence_card_accepts_markers() {
        let result = validate_evidence_card_text(
            "CiEvidenceCardV0 mission_id summary",
            CI_EVIDENCE_CARD_SCHEMA,
        );
        assert!(result.valid, "{:?}", result.errors);
    }
}
