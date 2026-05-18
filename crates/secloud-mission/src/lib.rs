//! Mission and authority models for StealthEye Cloud S1.

pub const S1_REQUIRED_MISSION_SCHEMAS: &[&str] = &[
    "MissionExecutorDispatchV0",
    "AuthorityQueueV0",
    "OutputShelfV0",
    "OperatorStateV0",
    "StealthEyeAutonomyStatusV0",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MissionCheck {
    pub valid: bool,
    pub errors: Vec<String>,
}

pub fn validate_required_markers(content: &str, markers: &[&str]) -> MissionCheck {
    let errors = markers
        .iter()
        .filter(|marker| !content.contains(**marker))
        .map(|marker| format!("missing marker: {marker}"))
        .collect::<Vec<_>>();

    MissionCheck {
        valid: errors.is_empty(),
        errors,
    }
}

pub fn mission_schema_names() -> &'static [&'static str] {
    S1_REQUIRED_MISSION_SCHEMAS
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn marker_validation_reports_missing_values() {
        let result = validate_required_markers(
            "MissionExecutorDispatchV0",
            &["MissionExecutorDispatchV0", "AuthorityQueueV0"],
        );
        assert!(!result.valid);
        assert_eq!(result.errors.len(), 1);
    }

    #[test]
    fn schema_inventory_contains_authority_queue() {
        assert!(mission_schema_names().contains(&"AuthorityQueueV0"));
    }
}
