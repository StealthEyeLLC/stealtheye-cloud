//! S5 end-to-end mission validation for StealthEye Cloud.

pub const E2E_PACKET_SCHEMAS: &[&str] = &[
    "EndToEndMissionV0",
    "EndToEndStepV0",
    "EndToEndProofSummaryV0",
    "MissionCompletionGateV0",
];

pub const REQUIRED_E2E_STEPS: &[&str] = &[
    "read_active_state",
    "validate_relay",
    "validate_seal",
    "validate_schemas",
    "validate_skills",
    "validate_browser_proof",
    "validate_release_readiness",
];

pub fn is_e2e_schema(name: &str) -> bool {
    E2E_PACKET_SCHEMAS.contains(&name)
}

pub fn has_required_step(step: &str) -> bool {
    REQUIRED_E2E_STEPS.contains(&step)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_end_to_end_mission_schema() {
        assert!(is_e2e_schema("EndToEndMissionV0"));
    }

    #[test]
    fn includes_browser_proof_step() {
        assert!(has_required_step("validate_browser_proof"));
    }
}
