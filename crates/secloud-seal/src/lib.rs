//! StealthEye Seal validation for S0.

pub const ALLOWED_SEAL_TYPES: &[&str] = &[
    "MISSION",
    "APPROVAL",
    "PROOF",
    "BLOCKED",
    "HANDOFF",
    "FINAL",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SealValidation {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn validate_seal_json_text(content: &str) -> SealValidation {
    let mut errors = Vec::new();

    for required in [
        "\"schema_version\"",
        "\"seal_id\"",
        "\"seal_type\"",
        "\"product\"",
        "\"mission_id\"",
        "\"next_action\"",
        "\"content_hash\"",
    ] {
        if !content.contains(required) {
            errors.push(format!("missing required field marker: {required}"));
        }
    }

    if !content.contains("\"schema_version\": \"StealthEyeSealV0\"") {
        errors.push("schema_version must be StealthEyeSealV0".to_string());
    }

    let has_allowed_type = ALLOWED_SEAL_TYPES
        .iter()
        .any(|kind| content.contains(&format!("\"seal_type\": \"{kind}\"")));
    if !has_allowed_type {
        errors.push("seal_type must be one of MISSION, APPROVAL, PROOF, BLOCKED, HANDOFF, FINAL".to_string());
    }

    if !content.contains("StealthEye Cloud") {
        errors.push("product must reference StealthEye Cloud".to_string());
    }

    SealValidation {
        valid: errors.is_empty(),
        errors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_seal_accepts_allowed_type() {
        let content = r#"{
          "schema_version": "StealthEyeSealV0",
          "seal_id": "seal-test",
          "seal_type": "MISSION",
          "product": "StealthEye Cloud",
          "mission_id": "s0",
          "next_action": "continue",
          "content_hash": "hash"
        }"#;
        let result = validate_seal_json_text(content);
        assert!(result.valid, "{:?}", result.errors);
    }

    #[test]
    fn invalid_seal_rejects_unknown_type() {
        let content = r#"{
          "schema_version": "StealthEyeSealV0",
          "seal_id": "seal-test",
          "seal_type": "CI_RECEIPT",
          "product": "StealthEye Cloud",
          "mission_id": "s0",
          "next_action": "continue",
          "content_hash": "hash"
        }"#;
        let result = validate_seal_json_text(content);
        assert!(!result.valid);
    }
}
