//! S12 One-Accept Mission Gauntlet contracts for StealthEye Cloud.
//!
//! S12 proves and hardens S11. The gauntlet records bounded mission evidence,
//! approval-count proof, connector/GitHub capability closure, command-dispatch
//! fallback evidence, proof freshness policy, replay, repair, resume, and boundary
//! quality. It does not bypass GitHub permissions and it does not authorize
//! secrets, paid compute, production deployment, database mutation, private data
//! exposure, browser-cookie/session-token automation, destructive irreversible
//! action, scope expansion, or GitHub permission bypass.

pub const MISSION_GAUNTLET_PACKET_SCHEMAS: &[&str] = &[
    "MissionGauntletSuiteV0",
    "GauntletMissionV0",
    "GauntletRunPlanV0",
    "GauntletResultV0",
    "GitHubCapabilityClosureV0",
    "ConnectorCapabilityManifestV0",
    "CurrentMainHeadProofV0",
    "RequiredCheckResolverV0",
    "PathFilterPendingGuardV0",
    "MissionControlPlaneV0",
    "MissionResourceMirrorV0",
    "AgenticWorkflowInjectionGuardV0",
    "PromptToScriptFirewallV0",
    "MissionReplayHarnessV0",
    "SyntheticFailureMissionV0",
    "RepairLoopExerciseV0",
    "MissionResumeTortureTestV0",
    "BoundaryQualityGateV0",
    "S12MissionGauntletProofV0",
];

pub const MISSION_GAUNTLET_VALIDATION_TARGETS: &[&str] = &[
    "mission-gauntlet",
    "approval-count-report",
    "github-capability-closure",
    "connector-capability-manifest",
    "command-dispatch",
    "command-outbox",
    "proof-surface",
    "mission-surface",
    "repo-surface",
    "current-main-head-proof",
    "required-check-resolver",
    "path-filter-pending-guard",
    "mission-control-plane",
    "mission-resource-mirror",
    "descriptor-pinning",
    "tool-output-taint",
    "workflow-injection-guard",
    "prompt-to-script-firewall",
    "mission-replay",
    "synthetic-failure-injection",
    "mission-resume",
    "boundary-quality",
];

pub const REQUIRED_MISSION_CLASSES: &[&str] = &[
    "docs_state_update",
    "schema_validator_registration",
    "proof_script_repair",
    "workflow_path_filter_update",
    "failed_ci_repair",
    "post_merge_proof_freshness",
    "branch_pr_reuse",
    "rerun_resume",
];

pub const REQUIRED_ARTIFACTS: &[&str] = &[
    ".stealtheye/mission-gauntlet/gauntlet-run-plan.json",
    ".stealtheye/mission-gauntlet/gauntlet-result.json",
    ".stealtheye/mission-gauntlet/approval-count-report.json",
    ".stealtheye/mission-gauntlet/github-capability-closure.json",
    ".stealtheye/mission-gauntlet/connector-capability-manifest.json",
    ".stealtheye/mission-gauntlet/current-main-head-proof.json",
    ".stealtheye/mission-gauntlet/required-check-resolver-report.json",
    ".stealtheye/mission-gauntlet/path-filter-pending-guard.json",
    ".stealtheye/mission-gauntlet/mission-control-plane.json",
    ".stealtheye/mission-gauntlet/mission-resource-mirror.json",
    ".stealtheye/mission-gauntlet/workflow-injection-guard-report.json",
    ".stealtheye/mission-gauntlet/prompt-to-script-firewall-report.json",
    ".stealtheye/mission-gauntlet/mission-replay-report.json",
    ".stealtheye/mission-gauntlet/synthetic-failure-report.json",
    ".stealtheye/mission-gauntlet/resume-torture-report.json",
    ".stealtheye/mission-gauntlet/boundary-quality-report.json",
    ".stealtheye/mission-gauntlet/s12-proof.json",
    ".stealtheye/command-outbox/latest.json",
    ".stealtheye/command-outbox/history.jsonl",
];

pub const REQUIRED_GAUNTLET_CHECKS: &[&str] = &[
    "at_least_five_bounded_missions",
    "approval_count_report_per_mission",
    "zero_routine_midpoint_approvals",
    "synthetic_failure_repaired",
    "branch_pr_reuse_proven",
    "post_merge_current_main_gate_defined",
    "required_checks_resolved_from_workflow_data",
    "path_filter_pending_risk_tested",
    "connector_manifest_records_can_and_cannot",
    "command_dispatch_bridge_available",
    "github_app_need_evidence_based",
    "receipt_first_resource_mirror",
    "workflow_injection_guard_blocks_untrusted_text",
    "prompt_to_script_firewall_blocks_model_shell",
    "resume_no_duplicate_branch_pr_commit_merge",
    "boundary_stop_single_action",
    "s0_s11_gates_not_weakened",
];

pub const TRUE_BOUNDARY_CLASSES: &[&str] = &[
    "secrets",
    "paid_compute",
    "production_deployment",
    "database_mutation",
    "private_data_exposure",
    "account_permission_changes",
    "browser_cookie_session_token_automation",
    "destructive_irreversible_action",
    "github_permission_bypass",
];

fn has(values: &[&str], value: &str) -> bool {
    values.contains(&value)
}

pub fn is_mission_gauntlet_schema(name: &str) -> bool {
    has(MISSION_GAUNTLET_PACKET_SCHEMAS, name)
}

pub fn is_mission_gauntlet_validation_target(target: &str) -> bool {
    has(MISSION_GAUNTLET_VALIDATION_TARGETS, target)
}

pub fn is_required_mission_class(class: &str) -> bool {
    has(REQUIRED_MISSION_CLASSES, class)
}

pub fn is_required_artifact(path: &str) -> bool {
    has(REQUIRED_ARTIFACTS, path)
}

pub fn has_required_gauntlet_check(check: &str) -> bool {
    has(REQUIRED_GAUNTLET_CHECKS, check)
}

pub fn is_true_boundary_class(class: &str) -> bool {
    has(TRUE_BOUNDARY_CLASSES, class)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GauntletVerdict {
    Passed,
    NeedsRepair,
    BoundaryStop,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GauntletRunSummary {
    pub bounded_missions: u16,
    pub initial_approval_count: u16,
    pub routine_midpoint_approvals: u16,
    pub synthetic_failure_repaired: bool,
    pub branch_pr_reuse_proven: bool,
    pub current_main_head_proof_fresh: bool,
    pub true_boundary_hit: bool,
    pub s0_s11_gate_weakening: bool,
}

pub fn evaluate_gauntlet(summary: GauntletRunSummary) -> GauntletVerdict {
    if summary.true_boundary_hit {
        return GauntletVerdict::BoundaryStop;
    }
    if summary.s0_s11_gate_weakening
        || summary.initial_approval_count != 1
        || summary.routine_midpoint_approvals != 0
        || summary.bounded_missions < 5
    {
        return GauntletVerdict::Invalid;
    }
    if !summary.synthetic_failure_repaired
        || !summary.branch_pr_reuse_proven
        || !summary.current_main_head_proof_fresh
    {
        return GauntletVerdict::NeedsRepair;
    }
    GauntletVerdict::Passed
}

#[cfg(test)]
mod tests {
    use super::*;

    const SUITE_SCHEMA: &str = include_str!("../../../schemas/MissionGauntletSuiteV0.schema.json");
    const RUN_PLAN_SCHEMA: &str = include_str!("../../../schemas/GauntletRunPlanV0.schema.json");
    const RESULT_SCHEMA: &str = include_str!("../../../schemas/GauntletResultV0.schema.json");
    const PROOF_SCHEMA: &str =
        include_str!("../../../schemas/S12MissionGauntletProofV0.schema.json");

    #[test]
    fn schema_inventory_contains_required_s12_contracts() {
        assert!(is_mission_gauntlet_schema("MissionGauntletSuiteV0"));
        assert!(is_mission_gauntlet_schema("GauntletRunPlanV0"));
        assert!(is_mission_gauntlet_schema("GitHubCapabilityClosureV0"));
        assert!(is_mission_gauntlet_schema("ConnectorCapabilityManifestV0"));
        assert!(is_mission_gauntlet_schema("CurrentMainHeadProofV0"));
        assert!(is_mission_gauntlet_schema("S12MissionGauntletProofV0"));
    }

    #[test]
    fn core_schema_files_are_materialized() {
        assert!(SUITE_SCHEMA.contains("MissionGauntletSuiteV0"));
        assert!(RUN_PLAN_SCHEMA.contains("GauntletRunPlanV0"));
        assert!(RESULT_SCHEMA.contains("GauntletResultV0"));
        assert!(PROOF_SCHEMA.contains("S12MissionGauntletProofV0"));
    }

    #[test]
    fn required_mission_classes_are_closed() {
        assert!(is_required_mission_class("docs_state_update"));
        assert!(is_required_mission_class("proof_script_repair"));
        assert!(is_required_mission_class("post_merge_proof_freshness"));
        assert!(is_required_mission_class("branch_pr_reuse"));
        assert!(!is_required_mission_class("secret_rotation"));
    }

    #[test]
    fn gauntlet_pass_requires_one_accept_and_fresh_main_proof() {
        let summary = GauntletRunSummary {
            bounded_missions: 6,
            initial_approval_count: 1,
            routine_midpoint_approvals: 0,
            synthetic_failure_repaired: true,
            branch_pr_reuse_proven: true,
            current_main_head_proof_fresh: true,
            true_boundary_hit: false,
            s0_s11_gate_weakening: false,
        };
        assert_eq!(evaluate_gauntlet(summary), GauntletVerdict::Passed);
    }

    #[test]
    fn routine_midpoint_approval_invalidates_gauntlet() {
        let summary = GauntletRunSummary {
            bounded_missions: 6,
            initial_approval_count: 1,
            routine_midpoint_approvals: 1,
            synthetic_failure_repaired: true,
            branch_pr_reuse_proven: true,
            current_main_head_proof_fresh: true,
            true_boundary_hit: false,
            s0_s11_gate_weakening: false,
        };
        assert_eq!(evaluate_gauntlet(summary), GauntletVerdict::Invalid);
    }

    #[test]
    fn required_artifacts_include_command_outbox() {
        assert!(is_required_artifact(
            ".stealtheye/mission-gauntlet/s12-proof.json"
        ));
        assert!(is_required_artifact(
            ".stealtheye/command-outbox/latest.json"
        ));
        assert!(has_required_gauntlet_check(
            "command_dispatch_bridge_available"
        ));
        assert!(is_true_boundary_class("github_permission_bypass"));
    }
}
