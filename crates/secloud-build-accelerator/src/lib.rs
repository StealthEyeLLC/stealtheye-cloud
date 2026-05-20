//! S9 One-Drop Build Accelerator contracts for StealthEye Cloud.
//!
//! This crate reduces avoidable process friction while preserving proof strength.
//! S11 extends the same validator rail with one-accept mission-executor validation
//! targets so `secloud doctor` covers the approval-spam solution without weakening
//! earlier S9 checks.

pub const BUILD_ACCELERATOR_PACKET_SCHEMAS: &[&str] = &[
    "OneDropPlanV0",
    "MissionApprovalEnvelopeV0",
    "ApprovalCompressionPolicyV0",
    "NoMidpointAskPolicyV0",
    "ToolCallBundlePolicyV0",
    "RepoMutationBatchV0",
    "BatchRepairPlanV0",
    "MergeAwareNextActionV0",
    "PostMergeTruthTemplateV0",
    "StateConsistencyReportV0",
    "ObsoletePhraseScanV0",
    "DocConflictFindingV0",
    "StealthEyePhaseStateV0",
    "BranchLifecycleV0",
    "PrLifecycleV0",
    "ProofSelectorV0",
    "RequiredChecksManifestV0",
    "WorkflowPathFilterSimulationV0",
    "CiWaveCounterV0",
    "RepairCauseMemoryV0",
    "ValidatorRegistrationReportV0",
    "SchemaInventoryDiffV0",
    "WorkspaceRegistrationCheckV0",
    "CapabilityDowngradeFindingV0",
    "NoSilentDowngradePolicyV0",
    "HumanAvailabilityProfileV0",
    "ConfirmationFrictionEventV0",
    "ToolFallbackLadderV0",
    "ExistingWorkReusePolicyV0",
    "PartialDropRecoveryV0",
    "NextTabPromptArtifactV0",
    "CapabilityActivationLedgerV0",
    "FuturePhaseDefaultContractV0",
    "BuildVelocityReportV0",
    "S9BuildAcceleratorProofV0",
    "GitHubCapabilityPreflightV0",
    "MissionLeaseV0",
    "MissionExecutorRequestV0",
    "BatchRepoMutationV0",
    "BranchControllerV0",
    "PrControllerV0",
    "ProofControllerV0",
    "ProofRepairLoopV0",
    "MergeWhenGreenControllerV0",
    "PostMergeProofFreshnessGateV0",
    "BoundaryStopV0",
    "MissionJournalV0",
    "MissionExecutorStateV0",
    "IdempotencyKeyV0",
    "ApprovalCountReportV0",
    "MissionExecutorProofV0",
];

pub const BUILD_ACCELERATOR_VALIDATION_TARGETS: &[&str] = &[
    "one-drop",
    "mission-approval",
    "approval-compression",
    "no-midpoint-ask",
    "tool-call-bundling",
    "repo-mutation-batch",
    "batch-repair",
    "merge-aware-handoff",
    "no-cleanup-pr",
    "state-consistency",
    "phase-truth",
    "branch-lifecycle",
    "pr-lifecycle",
    "proof-selector",
    "required-checks",
    "workflow-path-filter",
    "ci-wave-counter",
    "repair-cause-memory",
    "validator-registration",
    "schema-inventory",
    "workspace-registration",
    "obsolete-text",
    "doc-conflicts",
    "merge-readiness-red-team",
    "build-velocity",
    "confirmation-friction",
    "human-availability",
    "tool-fallback",
    "existing-work-reuse",
    "partial-completion-recovery",
    "no-silent-downgrade",
    "capability-activation-ledger",
    "future-phase-contract",
    "next-tab-prompt",
    "mission-lease",
    "mission-executor-request",
    "github-capability-preflight",
    "batch-repo-mutation",
    "branch-controller",
    "pr-controller",
    "proof-controller",
    "proof-repair-loop",
    "merge-when-green",
    "post-merge-proof-freshness",
    "boundary-stop",
    "mission-journal",
    "mission-state",
    "idempotency",
    "approval-count-proof",
    "mission-executor",
];

pub const ONE_DROP_STEPS: &[&str] = &[
    "mission_approval",
    "preflight_state_snapshot",
    "coherent_repo_mutation",
    "one_pr",
    "one_proof_wave",
    "batched_repairs",
    "merge_when_green",
    "post_merge_truth_resolution",
];

pub const ROUTINE_ACTION_CLASSES: &[&str] = &[
    "safe_file_creation",
    "safe_file_update",
    "branch_creation",
    "commit_creation",
    "pr_creation",
    "ci_inspection",
    "exact_failure_repair",
    "docs_state_update",
    "merge_when_green",
    "read_repo",
    "create_or_reuse_branch",
    "batch_create_update_delete_files",
    "open_or_reuse_pr",
    "inspect_ci_and_logs",
    "classify_proof_failures",
    "rerun_proof",
];

pub const BOUNDARY_ACTION_CLASSES: &[&str] = &[
    "secrets",
    "paid_compute",
    "private_data_exposure",
    "account_permission_changes",
    "production_deployment",
    "database_mutation",
    "browser_cookie_session_automation",
    "destructive_irreversible_action",
    "unresolved_high_impact_ambiguity",
    "scope_expansion",
    "unapproved_external_posting",
    "legal_compliance_signoff",
    "github_permission_bypass",
];

pub const BATCH_REPAIR_RULES: &[&str] = &[
    "inspect_all_failures_before_patch",
    "group_related_repairs",
    "patch_exact_failures_only",
    "rerun_after_batch",
    "do_not_add_truth_only_rerun_commit_unless_required",
    "retry_budget_required",
    "known_failure_autopatch_only",
];

pub const MERGE_HANDOFF_RULES: &[&str] = &[
    "pre_merge_truth_template",
    "post_merge_truth_template",
    "merge_sha_resolution_field",
    "no_stale_pending_language",
    "next_action_survives_merge",
    "direct_truth_commit_requires_fresh_main_proof",
];

pub const STATE_CONSISTENCY_RULES: &[&str] = &[
    "readme_build_plan_match",
    "active_relay_seal_match",
    "next_action_matches_phase",
    "no_obsolete_phase_reference",
    "no_duplicate_conflicting_doc_truth",
    "s11_prep_truth_normalized",
];

pub const LIFECYCLE_STATES: &[&str] = &[
    "planned",
    "branch_active",
    "pr_open",
    "ci_running",
    "repairing",
    "green",
    "merged",
    "blocked",
    "boundary_stop",
    "fresh_main_proof_required",
];

pub const PROOF_SELECTION_RULES: &[&str] = &[
    "touched_surface_declared",
    "required_checks_manifested",
    "path_filters_simulated",
    "unexpected_skip_detected",
    "ci_wave_counted",
    "current_head_sha_recorded",
    "freshness_checked",
];

pub const REGISTRATION_GUARDS: &[&str] = &[
    "validator_registered",
    "schema_inventory_registered",
    "workspace_member_registered",
    "cli_dependency_registered",
    "workflow_registered",
    "mission_executor_workflow_registered",
];

pub const HUMAN_ATTENTION_RULES: &[&str] = &[
    "mission_approval_reuse",
    "no_midpoint_ask_for_routine_action",
    "friction_event_recorded",
    "low_attention_workday_supported",
    "approval_count_report_required",
    "routine_midpoint_approvals_zero",
];

pub const TOOL_FALLBACK_RULES: &[&str] = &[
    "prefer_git_tree_batch",
    "fall_back_to_contents_api",
    "fall_back_to_pr_patch",
    "stop_at_true_boundary",
    "github_permission_boundary_stop",
];

pub const RECOVERY_RULES: &[&str] = &[
    "existing_branch_reuse_checked",
    "existing_pr_reuse_checked",
    "partial_drop_recovery_plan",
    "saturation_handoff_prompt",
    "idempotency_key_checked",
    "action_dedup_receipt_recorded",
];

pub const NO_SILENT_DOWNGRADE_RULES: &[&str] = &[
    "required_validator_not_removed",
    "required_schema_not_removed",
    "required_workflow_not_removed",
    "proof_gate_not_weakened",
    "safety_boundary_not_relaxed",
    "no_unverified_truth_commit_policy",
];

pub const FUTURE_PHASE_CONTRACT_RULES: &[&str] = &[
    "one_drop_default",
    "mission_approval_default",
    "proof_plan_required",
    "state_update_required",
    "final_report_required",
    "next_tab_prompt_required",
    "one_accept_executor_available",
];

pub const REQUIRED_S9_DOCS: &[&str] = &[
    "docs/ONE_DROP_BUILD_MODE.md",
    "docs/MISSION_APPROVAL_ENVELOPE.md",
    "docs/BATCH_REPAIR_POLICY.md",
    "docs/MERGE_AWARE_HANDOFF.md",
    "docs/PHASE_TEMPLATE_SYSTEM.md",
    "docs/S9_FINAL_REPORT.md",
];

pub const REQUIRED_S11_DOCS: &[&str] = &[
    "docs/S11_ONE_ACCEPT_MISSION_EXECUTOR.md",
    "docs/S11_FINAL_REPORT.md",
];

pub const REQUIRED_S9_PROMPT_ARTIFACTS: &[&str] = &[
    "docs/PROMPTS/NEXT_TAB_PROMPT.md",
    "docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md",
];

pub const VELOCITY_METRICS: &[&str] = &[
    "mission_approvals",
    "repo_mutation_batches",
    "pull_requests",
    "ci_waves",
    "repair_commits",
    "cleanup_prs_avoided",
    "workflow_dispatch_runs",
];

pub const FRICTION_METRICS: &[&str] = &[
    "routine_confirmations_avoided",
    "true_boundaries_detected",
    "tool_calls_batched",
    "human_attention_events",
    "routine_midpoint_approvals",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildAccelerationVerdict {
    Accelerated,
    NeedsBatching,
    BlockedByBoundary,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BuildRunSummary {
    pub mission_approved: bool,
    pub repo_mutation_batches: u16,
    pub pull_requests: u16,
    pub ci_waves: u16,
    pub repair_commits_batched: bool,
    pub cleanup_prs_needed: u16,
    pub true_boundary_hit: bool,
}

pub fn evaluate_build_run(summary: BuildRunSummary) -> BuildAccelerationVerdict {
    if summary.true_boundary_hit {
        return BuildAccelerationVerdict::BlockedByBoundary;
    }
    if !summary.mission_approved || summary.pull_requests == 0 || summary.repo_mutation_batches == 0
    {
        return BuildAccelerationVerdict::Invalid;
    }
    if summary.repo_mutation_batches == 1
        && summary.pull_requests == 1
        && summary.ci_waves <= 2
        && summary.repair_commits_batched
        && summary.cleanup_prs_needed == 0
    {
        return BuildAccelerationVerdict::Accelerated;
    }
    BuildAccelerationVerdict::NeedsBatching
}

pub fn is_build_accelerator_schema(name: &str) -> bool {
    BUILD_ACCELERATOR_PACKET_SCHEMAS.contains(&name)
}

pub fn is_build_accelerator_validation_target(target: &str) -> bool {
    BUILD_ACCELERATOR_VALIDATION_TARGETS.contains(&target)
}

pub fn has_one_drop_step(step: &str) -> bool {
    ONE_DROP_STEPS.contains(&step)
}

pub fn is_routine_action_class(action: &str) -> bool {
    ROUTINE_ACTION_CLASSES.contains(&action)
}

pub fn is_boundary_action_class(action: &str) -> bool {
    BOUNDARY_ACTION_CLASSES.contains(&action)
}

pub fn has_batch_repair_rule(rule: &str) -> bool {
    BATCH_REPAIR_RULES.contains(&rule)
}

pub fn has_merge_handoff_rule(rule: &str) -> bool {
    MERGE_HANDOFF_RULES.contains(&rule)
}

pub fn has_state_consistency_rule(rule: &str) -> bool {
    STATE_CONSISTENCY_RULES.contains(&rule)
}

pub fn has_lifecycle_state(state: &str) -> bool {
    LIFECYCLE_STATES.contains(&state)
}

pub fn has_proof_selection_rule(rule: &str) -> bool {
    PROOF_SELECTION_RULES.contains(&rule)
}

pub fn has_registration_guard(rule: &str) -> bool {
    REGISTRATION_GUARDS.contains(&rule)
}

pub fn has_human_attention_rule(rule: &str) -> bool {
    HUMAN_ATTENTION_RULES.contains(&rule)
}

pub fn has_tool_fallback_rule(rule: &str) -> bool {
    TOOL_FALLBACK_RULES.contains(&rule)
}

pub fn has_recovery_rule(rule: &str) -> bool {
    RECOVERY_RULES.contains(&rule)
}

pub fn has_no_silent_downgrade_rule(rule: &str) -> bool {
    NO_SILENT_DOWNGRADE_RULES.contains(&rule)
}

pub fn has_future_phase_contract_rule(rule: &str) -> bool {
    FUTURE_PHASE_CONTRACT_RULES.contains(&rule)
}

pub fn has_required_doc(path: &str) -> bool {
    REQUIRED_S9_DOCS.contains(&path) || REQUIRED_S11_DOCS.contains(&path)
}

pub fn has_required_prompt_artifact(path: &str) -> bool {
    REQUIRED_S9_PROMPT_ARTIFACTS.contains(&path)
}

pub fn has_velocity_metric(metric: &str) -> bool {
    VELOCITY_METRICS.contains(&metric)
}

pub fn has_friction_metric(metric: &str) -> bool {
    FRICTION_METRICS.contains(&metric)
}

pub fn is_s11_validation_target(target: &str) -> bool {
    matches!(
        target,
        "mission-lease"
            | "mission-executor-request"
            | "github-capability-preflight"
            | "batch-repo-mutation"
            | "branch-controller"
            | "pr-controller"
            | "proof-controller"
            | "proof-repair-loop"
            | "merge-when-green"
            | "post-merge-proof-freshness"
            | "boundary-stop"
            | "mission-journal"
            | "mission-state"
            | "idempotency"
            | "approval-count-proof"
            | "mission-executor"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const ONE_DROP_SCHEMA: &str = include_str!("../../../schemas/OneDropPlanV0.schema.json");
    const MISSION_APPROVAL_SCHEMA: &str =
        include_str!("../../../schemas/MissionApprovalEnvelopeV0.schema.json");
    const STATE_REPORT_SCHEMA: &str =
        include_str!("../../../schemas/StateConsistencyReportV0.schema.json");
    const VELOCITY_SCHEMA: &str =
        include_str!("../../../schemas/BuildVelocityReportV0.schema.json");
    const S11_LEASE_SCHEMA: &str = include_str!("../../../schemas/MissionLeaseV0.schema.json");

    #[test]
    fn schema_inventory_contains_required_s9_contracts() {
        assert!(is_build_accelerator_schema("OneDropPlanV0"));
        assert!(is_build_accelerator_schema("MissionApprovalEnvelopeV0"));
        assert!(is_build_accelerator_schema("BatchRepairPlanV0"));
        assert!(is_build_accelerator_schema("StateConsistencyReportV0"));
        assert!(is_build_accelerator_schema("FuturePhaseDefaultContractV0"));
        assert!(is_build_accelerator_schema("S9BuildAcceleratorProofV0"));
    }

    #[test]
    fn schema_inventory_contains_required_s11_contracts() {
        assert!(is_build_accelerator_schema("MissionLeaseV0"));
        assert!(is_build_accelerator_schema("MissionExecutorRequestV0"));
        assert!(is_build_accelerator_schema("ProofRepairLoopV0"));
        assert!(is_build_accelerator_schema("PostMergeProofFreshnessGateV0"));
        assert!(is_build_accelerator_schema("ApprovalCountReportV0"));
        assert!(is_build_accelerator_schema("MissionExecutorProofV0"));
        assert!(is_s11_validation_target("mission-lease"));
        assert!(is_s11_validation_target("mission-executor"));
    }

    #[test]
    fn schema_files_are_materialized() {
        assert!(ONE_DROP_SCHEMA.contains("OneDropPlanV0"));
        assert!(MISSION_APPROVAL_SCHEMA.contains("MissionApprovalEnvelopeV0"));
        assert!(STATE_REPORT_SCHEMA.contains("StateConsistencyReportV0"));
        assert!(VELOCITY_SCHEMA.contains("BuildVelocityReportV0"));
        assert!(S11_LEASE_SCHEMA.contains("MissionLeaseV0"));
    }

    #[test]
    fn one_drop_steps_match_s9_mission_shape() {
        for step in ONE_DROP_STEPS {
            assert!(has_one_drop_step(step));
        }
        assert!(has_one_drop_step("mission_approval"));
        assert!(has_one_drop_step("merge_when_green"));
    }

    #[test]
    fn mission_approval_does_not_cover_true_boundaries() {
        assert!(is_routine_action_class("safe_file_update"));
        assert!(is_routine_action_class("ci_inspection"));
        assert!(is_routine_action_class("read_repo"));
        assert!(is_boundary_action_class("secrets"));
        assert!(is_boundary_action_class("database_mutation"));
        assert!(is_boundary_action_class("github_permission_bypass"));
        assert!(!is_routine_action_class("secrets"));
        assert!(!is_routine_action_class(
            "browser_cookie_session_automation"
        ));
    }

    #[test]
    fn no_silent_downgrade_rules_preserve_validator_strength() {
        assert!(has_no_silent_downgrade_rule(
            "required_validator_not_removed"
        ));
        assert!(has_no_silent_downgrade_rule("proof_gate_not_weakened"));
        assert!(has_no_silent_downgrade_rule("safety_boundary_not_relaxed"));
        assert!(has_no_silent_downgrade_rule(
            "no_unverified_truth_commit_policy"
        ));
    }

    #[test]
    fn accelerated_build_run_requires_one_pr_and_no_cleanup_pr() {
        let summary = BuildRunSummary {
            mission_approved: true,
            repo_mutation_batches: 1,
            pull_requests: 1,
            ci_waves: 1,
            repair_commits_batched: true,
            cleanup_prs_needed: 0,
            true_boundary_hit: false,
        };
        assert_eq!(
            evaluate_build_run(summary),
            BuildAccelerationVerdict::Accelerated
        );
    }

    #[test]
    fn cleanup_pr_need_is_not_accelerated() {
        let summary = BuildRunSummary {
            mission_approved: true,
            repo_mutation_batches: 1,
            pull_requests: 1,
            ci_waves: 1,
            repair_commits_batched: true,
            cleanup_prs_needed: 1,
            true_boundary_hit: false,
        };
        assert_eq!(
            evaluate_build_run(summary),
            BuildAccelerationVerdict::NeedsBatching
        );
    }
}
