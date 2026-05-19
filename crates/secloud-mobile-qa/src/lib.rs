//! S6 mobile and game QA readiness contracts for StealthEye Cloud.
//!
//! This crate models preview/playtest readiness contracts. It does not launch
//! devices, emulate phones, or claim live mobile activation in S6.

pub const MOBILE_QA_PACKET_SCHEMAS: &[&str] = &[
    "MobileQaReadinessV0",
    "GameQaReadinessV0",
    "MobilePreviewPolicyV0",
];

pub const REQUIRED_MOBILE_QA_CHECKS: &[&str] = &[
    "viewport_matrix_declared",
    "touch_input_matrix_declared",
    "artifact_capture_declared",
    "no_device_session_rehydration",
    "activation_deferred_to_s7",
];

pub const REQUIRED_GAME_QA_CHECKS: &[&str] = &[
    "startup_smoke_declared",
    "input_smoke_declared",
    "screenshot_artifact_declared",
    "console_failure_capture_declared",
];

pub fn is_mobile_qa_schema(name: &str) -> bool {
    MOBILE_QA_PACKET_SCHEMAS.contains(&name)
}

pub fn has_mobile_qa_check(check: &str) -> bool {
    REQUIRED_MOBILE_QA_CHECKS.contains(&check)
}

pub fn has_game_qa_check(check: &str) -> bool {
    REQUIRED_GAME_QA_CHECKS.contains(&check)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mobile_qa_activation_is_deferred() {
        assert!(has_mobile_qa_check("activation_deferred_to_s7"));
    }

    #[test]
    fn game_qa_requires_screenshot_artifact() {
        assert!(has_game_qa_check("screenshot_artifact_declared"));
    }
}
