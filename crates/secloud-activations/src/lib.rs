//! S7 real activation contracts for StealthEye Cloud.
//!
//! S7 is the first active-capability wave. These contracts distinguish real
//! public-safe activations from S6 readiness-only lanes while preserving the
//! zero-trust boundaries established by S6.

pub const S7_ACTIVATION_PACKET_SCHEMAS: &[&str] = &[
    "MobilePlaytestActivationV0",
    "NotificationActivationRunV0",
    "KnowledgeMirrorExportV0",
    "S7ActivationProofV0",
];

pub const REQUIRED_ACTIVATION_LANES: &[&str] = &[
    "mobile_browser_game_preview",
    "notification_lane",
    "knowledge_mirror_export",
];

pub const REQUIRED_MOBILE_ACTIVATION_CHECKS: &[&str] = &[
    "static_preview_exists",
    "mobile_viewport_replay",
    "tap_input_replay",
    "swipe_input_replay",
    "screenshot_artifact",
    "console_network_capture",
    "playtest_link_artifact",
];

pub const REQUIRED_NOTIFICATION_ACTIVATION_CHECKS: &[&str] = &[
    "dry_run_without_secret",
    "real_dispatch_requires_explicit_webhook_and_enable_flag",
    "content_redaction_passed",
    "secret_value_not_logged",
    "decision_boundary_supported",
];

pub const REQUIRED_MIRROR_ACTIVATION_CHECKS: &[&str] = &[
    "public_safe_bundle_generated",
    "semantic_snapshot_generated",
    "redaction_scan_passed",
    "github_artifact_declared",
    "no_live_external_sync",
];

pub fn is_s7_activation_schema(name: &str) -> bool {
    S7_ACTIVATION_PACKET_SCHEMAS.contains(&name)
}

pub fn has_activation_lane(lane: &str) -> bool {
    REQUIRED_ACTIVATION_LANES.contains(&lane)
}

pub fn has_mobile_activation_check(check: &str) -> bool {
    REQUIRED_MOBILE_ACTIVATION_CHECKS.contains(&check)
}

pub fn has_notification_activation_check(check: &str) -> bool {
    REQUIRED_NOTIFICATION_ACTIVATION_CHECKS.contains(&check)
}

pub fn has_mirror_activation_check(check: &str) -> bool {
    REQUIRED_MIRROR_ACTIVATION_CHECKS.contains(&check)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MOBILE_PLAYTEST_SCHEMA: &str =
        include_str!("../../../schemas/MobilePlaytestActivationV0.schema.json");
    const NOTIFICATION_SCHEMA: &str =
        include_str!("../../../schemas/NotificationActivationRunV0.schema.json");
    const MIRROR_SCHEMA: &str = include_str!("../../../schemas/KnowledgeMirrorExportV0.schema.json");
    const PROOF_SCHEMA: &str = include_str!("../../../schemas/S7ActivationProofV0.schema.json");

    #[test]
    fn s7_schema_inventory_contains_all_activation_packets() {
        assert!(is_s7_activation_schema("MobilePlaytestActivationV0"));
        assert!(is_s7_activation_schema("NotificationActivationRunV0"));
        assert!(is_s7_activation_schema("KnowledgeMirrorExportV0"));
        assert!(is_s7_activation_schema("S7ActivationProofV0"));
    }

    #[test]
    fn s7_schema_files_are_materialized_and_version_locked() {
        assert!(MOBILE_PLAYTEST_SCHEMA.contains("MobilePlaytestActivationV0"));
        assert!(NOTIFICATION_SCHEMA.contains("NotificationActivationRunV0"));
        assert!(MIRROR_SCHEMA.contains("KnowledgeMirrorExportV0"));
        assert!(PROOF_SCHEMA.contains("S7ActivationProofV0"));
    }

    #[test]
    fn s7_lanes_are_real_activation_lanes() {
        assert!(has_activation_lane("mobile_browser_game_preview"));
        assert!(has_activation_lane("notification_lane"));
        assert!(has_activation_lane("knowledge_mirror_export"));
    }

    #[test]
    fn mobile_activation_requires_proof_artifacts_and_input_replay() {
        assert!(has_mobile_activation_check("mobile_viewport_replay"));
        assert!(has_mobile_activation_check("tap_input_replay"));
        assert!(has_mobile_activation_check("swipe_input_replay"));
        assert!(has_mobile_activation_check("playtest_link_artifact"));
    }

    #[test]
    fn notification_activation_preserves_secret_boundary() {
        assert!(has_notification_activation_check("dry_run_without_secret"));
        assert!(has_notification_activation_check(
            "real_dispatch_requires_explicit_webhook_and_enable_flag"
        ));
        assert!(has_notification_activation_check("secret_value_not_logged"));
    }

    #[test]
    fn mirror_activation_requires_export_snapshot_and_redaction() {
        assert!(has_mirror_activation_check("public_safe_bundle_generated"));
        assert!(has_mirror_activation_check("semantic_snapshot_generated"));
        assert!(has_mirror_activation_check("redaction_scan_passed"));
        assert!(has_mirror_activation_check("no_live_external_sync"));
    }
}
