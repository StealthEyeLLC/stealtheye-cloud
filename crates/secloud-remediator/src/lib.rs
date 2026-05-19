//! S8 active Remediator Mode contracts for StealthEye Cloud.
//!
//! Remediator Mode is proof-driven: diagnosis is allowed without a reproduced
//! failure, but completed remediation requires reproduced failure, bounded patch,
//! and passing proof gates.

pub const REMEDIATOR_PACKET_SCHEMAS: &[&str] = &[
    "RemediatorReadinessV0",
    "RemediationIntakeV0",
    "RemediationPermissionsV0",
    "RemediationRealityMapV0",
    "RemediationCommandDiscoveryV0",
    "RemediationEnvironmentV0",
    "RemediationReproductionV0",
    "RemediationFailureTaxonomyV0",
    "RemediationLocalizationV0",
    "RemediationRepairStrategyV0",
    "RemediationPatchTournamentV0",
    "RemediationProofPlanV0",
    "RemediationReportV0",
    "RemediationCommercialV0",
    "RemediatorExecutionReceiptV0",
];

pub const REQUIRED_REMEDIATOR_MODULES: &[&str] = &[
    "intake",
    "permissions",
    "reality_map",
    "command_discovery",
    "environment",
    "reproduction",
    "failure_taxonomy",
    "localization",
    "repair_strategy",
    "patch_tournament",
    "proof_plan",
    "report",
    "quote_risk",
];

pub const REQUIRED_REMEDIATOR_BOUNDARIES: &[&str] = &[
    "no_claim_without_reproduction",
    "no_patch_without_reproduced_failure",
    "bounded_patch_only",
    "proof_gates_required",
    "diagnosis_only_when_unreproduced",
    "no_browser_cookie_session_automation",
    "no_secrets",
    "no_paid_compute",
    "no_production_deployment",
    "no_database_mutation",
    "commercial_quote_does_not_activate_billing",
];

pub const REQUIRED_PROOF_REQUIREMENTS: &[&str] = &[
    "failing_behavior_reproduced",
    "bounded_patch_applied",
    "proof_gates_passed",
    "remediation_report_emitted",
];

pub const COMMAND_DISCOVERY_RULES: &[&str] = &[
    "inspect_declared_scripts",
    "prefer_existing_verify_entrypoint",
    "bounded_command_set",
    "no_secret_commands",
];

pub const ENVIRONMENT_RULES: &[&str] = &[
    "detect_language_runtime",
    "synthesize_minimal_environment",
    "avoid_paid_compute",
    "record_platform_assumptions",
];

pub const FAILURE_TAXONOMY_CLASSES: &[&str] = &[
    "test_failure",
    "compile_failure",
    "lint_failure",
    "browser_failure",
    "runtime_failure",
    "dependency_failure",
    "unknown_unreproduced",
];

pub const LOCALIZATION_SIGNALS: &[&str] = &[
    "failing_command",
    "stderr_excerpt",
    "changed_file_hint",
    "minimal_reproduction_path",
];

pub const REPAIR_STRATEGIES: &[&str] = &[
    "minimal_patch",
    "config_correction",
    "test_aligned_fix",
    "dependency_pin_or_update",
    "diagnosis_only",
];

pub const PATCH_TOURNAMENT_RULES: &[&str] = &[
    "candidate_patch_bounded",
    "candidate_patch_reversible",
    "selects_first_green_candidate",
    "rejects_unproven_candidate",
];

pub const COMMERCIAL_ARTIFACTS: &[&str] = &[
    "quote_range",
    "risk_band",
    "scope_assumptions",
    "billing_not_activated",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RemediationStatus {
    DiagnosisOnly,
    Remediated,
    Blocked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RemediationAttempt {
    pub failure_reproduced: bool,
    pub bounded_patch_applied: bool,
    pub proof_gates_passed: bool,
    pub report_emitted: bool,
}

pub fn classify_remediation_attempt(attempt: RemediationAttempt) -> RemediationStatus {
    if !attempt.failure_reproduced {
        return RemediationStatus::DiagnosisOnly;
    }
    if attempt.bounded_patch_applied && attempt.proof_gates_passed && attempt.report_emitted {
        return RemediationStatus::Remediated;
    }
    RemediationStatus::Blocked
}

pub fn can_claim_remediated(attempt: RemediationAttempt) -> bool {
    classify_remediation_attempt(attempt) == RemediationStatus::Remediated
}

pub fn is_remediator_schema(name: &str) -> bool {
    REMEDIATOR_PACKET_SCHEMAS.contains(&name)
}

pub fn has_remediator_module(module: &str) -> bool {
    REQUIRED_REMEDIATOR_MODULES.contains(&module)
}

pub fn has_remediation_boundary(boundary: &str) -> bool {
    REQUIRED_REMEDIATOR_BOUNDARIES.contains(&boundary)
}

pub fn has_proof_requirement(requirement: &str) -> bool {
    REQUIRED_PROOF_REQUIREMENTS.contains(&requirement)
}

pub fn has_command_discovery_rule(rule: &str) -> bool {
    COMMAND_DISCOVERY_RULES.contains(&rule)
}

pub fn has_environment_rule(rule: &str) -> bool {
    ENVIRONMENT_RULES.contains(&rule)
}

pub fn has_failure_taxonomy_class(kind: &str) -> bool {
    FAILURE_TAXONOMY_CLASSES.contains(&kind)
}

pub fn has_localization_signal(signal: &str) -> bool {
    LOCALIZATION_SIGNALS.contains(&signal)
}

pub fn has_repair_strategy(strategy: &str) -> bool {
    REPAIR_STRATEGIES.contains(&strategy)
}

pub fn has_patch_tournament_rule(rule: &str) -> bool {
    PATCH_TOURNAMENT_RULES.contains(&rule)
}

pub fn is_commercial_artifact(artifact: &str) -> bool {
    COMMERCIAL_ARTIFACTS.contains(&artifact)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INTAKE_SCHEMA: &str = include_str!("../../../schemas/RemediationIntakeV0.schema.json");
    const REALITY_MAP_SCHEMA: &str =
        include_str!("../../../schemas/RemediationRealityMapV0.schema.json");
    const TOURNAMENT_SCHEMA: &str =
        include_str!("../../../schemas/RemediationPatchTournamentV0.schema.json");
    const REPORT_SCHEMA: &str = include_str!("../../../schemas/RemediationReportV0.schema.json");

    #[test]
    fn schema_inventory_contains_s8_active_packets() {
        assert!(is_remediator_schema("RemediationRealityMapV0"));
        assert!(is_remediator_schema("RemediationCommandDiscoveryV0"));
        assert!(is_remediator_schema("RemediationEnvironmentV0"));
        assert!(is_remediator_schema("RemediationLocalizationV0"));
        assert!(is_remediator_schema("RemediationRepairStrategyV0"));
        assert!(is_remediator_schema("RemediationPatchTournamentV0"));
        assert!(is_remediator_schema("RemediationCommercialV0"));
        assert!(is_remediator_schema("RemediatorExecutionReceiptV0"));
    }

    #[test]
    fn schema_files_are_materialized() {
        assert!(INTAKE_SCHEMA.contains("RemediationIntakeV0"));
        assert!(REALITY_MAP_SCHEMA.contains("RemediationRealityMapV0"));
        assert!(TOURNAMENT_SCHEMA.contains("RemediationPatchTournamentV0"));
        assert!(REPORT_SCHEMA.contains("RemediationReportV0"));
    }

    #[test]
    fn all_required_modules_are_present() {
        for module in REQUIRED_REMEDIATOR_MODULES {
            assert!(has_remediator_module(module));
        }
    }

    #[test]
    fn remediation_cannot_be_claimed_without_reproduction() {
        let attempt = RemediationAttempt {
            failure_reproduced: false,
            bounded_patch_applied: true,
            proof_gates_passed: true,
            report_emitted: true,
        };
        assert_eq!(
            classify_remediation_attempt(attempt),
            RemediationStatus::DiagnosisOnly
        );
        assert!(!can_claim_remediated(attempt));
    }

    #[test]
    fn remediation_requires_patch_proof_and_report() {
        let attempt = RemediationAttempt {
            failure_reproduced: true,
            bounded_patch_applied: true,
            proof_gates_passed: true,
            report_emitted: true,
        };
        assert_eq!(
            classify_remediation_attempt(attempt),
            RemediationStatus::Remediated
        );
        assert!(can_claim_remediated(attempt));
    }

    #[test]
    fn reproduced_but_unproven_attempt_is_blocked() {
        let attempt = RemediationAttempt {
            failure_reproduced: true,
            bounded_patch_applied: true,
            proof_gates_passed: false,
            report_emitted: true,
        };
        assert_eq!(
            classify_remediation_attempt(attempt),
            RemediationStatus::Blocked
        );
        assert!(!can_claim_remediated(attempt));
    }

    #[test]
    fn boundaries_preserve_public_safe_operation() {
        assert!(has_remediation_boundary(
            "no_browser_cookie_session_automation"
        ));
        assert!(has_remediation_boundary("no_secrets"));
        assert!(has_remediation_boundary("no_paid_compute"));
        assert!(has_remediation_boundary("no_database_mutation"));
        assert!(has_remediation_boundary(
            "commercial_quote_does_not_activate_billing"
        ));
    }
}
