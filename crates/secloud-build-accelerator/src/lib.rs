//! S9 One-Drop Build Accelerator contracts plus S10 assistant-optimizer registry bridge.
//!
//! This crate reduces avoidable process friction while preserving proof strength.
//! It does not authorize weaker validators, skipped proofs, secrets, paid compute,
//! production deployment, database mutation, or browser-cookie/session-token automation.

pub const S10_ASSISTANT_OPTIMIZER_PACKET_SCHEMAS: &[&str] = &[
    "AssistantOperatingProfileV0", "BestAgentModeV0", "FastButExactModeV0",
    "BoundaryOnlyStopPolicyV0", "AssistantExecutionDisciplineV0", "MissionIntakePacketV0",
    "MissionScopeClassifierV0", "MissionAmbiguityScanV0", "MissionBoundaryScanV0",
    "MissionStartReadinessV0", "ContextLoadPlanV0", "RequiredContextFileSetV0",
    "ContextPriorityMapV0", "ContextFreshnessCheckV0", "RepoTruthFirstPolicyV0",
    "RepoStateDigestV0", "HandoffTruthSourceV0", "ChatMemoryConflictFindingV0",
    "PromptCompressionProfileV0", "MissionPromptTemplateV0", "ContinuationPromptTemplateV0",
    "RepairPromptTemplateV0", "PromptLossCheckV0", "ToolUsePlanV0", "ToolSelectionPolicyV0",
    "ToolCallBatchPlanV0", "ToolFallbackPolicyV0", "ToolFailureClassificationV0",
    "RetrySafetyDecisionV0", "LowAttentionWorkdayModeV0", "HumanAvailabilityProfileV0",
    "UserAttentionBudgetV0", "ProgressUpdatePolicyV0", "ProofNeedClassifierV0",
    "ProofGateSelectionV0", "ProofFailureTriageV0", "KnownFailurePatternV0",
    "RepairTriagePlanV0", "FailureClusterV0", "ScopeDriftDetectorV0",
    "DecisionCarryoverPacketV0", "CapabilityRealityMapV0", "HandoffQualityReportV0",
    "ContextSaturationSignalV0", "AssistantSelfAuditV0", "AssistantMistakePatternV0",
    "AssistantTraceDigestV0", "McpAwareOperatorPolicyV0", "AssistantOutputReviewV0",
    "OneSentenceStatusV0", "BuildCockpitCardV0", "OperatorStyleProfileV0",
    "AssistantOutputModeV0", "ReadOnlyVerificationModeV0", "BoundaryStopReportV0",
    "PlanActionTraceV0", "FutureMissionCandidateV0", "S10AssistantOptimizerProofV0",
];

pub const S10_ASSISTANT_OPTIMIZER_VALIDATION_TARGETS: &[&str] = &[
    "assistant-optimizer", "assistant-operating-profile", "mission-intake",
    "context-load-policy", "repo-truth-first", "tool-use-policy", "tool-fallback-policy",
    "low-attention-mode", "progress-update-policy", "handoff-quality", "prompt-compression",
    "scope-drift-guard", "decision-carryover", "proof-awareness", "repair-intelligence",
    "capability-map", "read-only-verification", "assistant-self-audit",
    "operator-mistake-detector", "build-cockpit", "trace-digest", "mcp-aware-operator-policy",
    "output-mode-selector", "future-mission-recommender",
];

pub const BUILD_ACCELERATOR_PACKET_SCHEMAS: &[&str] = &[
    "OneDropPlanV0", "MissionApprovalEnvelopeV0", "ApprovalCompressionPolicyV0",
    "NoMidpointAskPolicyV0", "ToolCallBundlePolicyV0", "RepoMutationBatchV0",
    "BatchRepairPlanV0", "MergeAwareNextActionV0", "PostMergeTruthTemplateV0",
    "StateConsistencyReportV0", "ObsoletePhraseScanV0", "DocConflictFindingV0",
    "StealthEyePhaseStateV0", "BranchLifecycleV0", "PrLifecycleV0", "ProofSelectorV0",
    "RequiredChecksManifestV0", "WorkflowPathFilterSimulationV0", "CiWaveCounterV0",
    "RepairCauseMemoryV0", "ValidatorRegistrationReportV0", "SchemaInventoryDiffV0",
    "WorkspaceRegistrationCheckV0", "CapabilityDowngradeFindingV0", "NoSilentDowngradePolicyV0",
    "HumanAvailabilityProfileV0", "ConfirmationFrictionEventV0", "ToolFallbackLadderV0",
    "ExistingWorkReusePolicyV0", "PartialDropRecoveryV0", "NextTabPromptArtifactV0",
    "CapabilityActivationLedgerV0", "FuturePhaseDefaultContractV0", "BuildVelocityReportV0",
    "S9BuildAcceleratorProofV0",
    "AssistantOperatingProfileV0", "BestAgentModeV0", "FastButExactModeV0",
    "BoundaryOnlyStopPolicyV0", "AssistantExecutionDisciplineV0", "MissionIntakePacketV0",
    "MissionScopeClassifierV0", "MissionAmbiguityScanV0", "MissionBoundaryScanV0",
    "MissionStartReadinessV0", "ContextLoadPlanV0", "RequiredContextFileSetV0",
    "ContextPriorityMapV0", "ContextFreshnessCheckV0", "RepoTruthFirstPolicyV0",
    "RepoStateDigestV0", "HandoffTruthSourceV0", "ChatMemoryConflictFindingV0",
    "PromptCompressionProfileV0", "MissionPromptTemplateV0", "ContinuationPromptTemplateV0",
    "RepairPromptTemplateV0", "PromptLossCheckV0", "ToolUsePlanV0", "ToolSelectionPolicyV0",
    "ToolCallBatchPlanV0", "ToolFallbackPolicyV0", "ToolFailureClassificationV0",
    "RetrySafetyDecisionV0", "LowAttentionWorkdayModeV0", "UserAttentionBudgetV0",
    "ProgressUpdatePolicyV0", "ProofNeedClassifierV0", "ProofGateSelectionV0",
    "ProofFailureTriageV0", "KnownFailurePatternV0", "RepairTriagePlanV0", "FailureClusterV0",
    "ScopeDriftDetectorV0", "DecisionCarryoverPacketV0", "CapabilityRealityMapV0",
    "HandoffQualityReportV0", "ContextSaturationSignalV0", "AssistantSelfAuditV0",
    "AssistantMistakePatternV0", "AssistantTraceDigestV0", "McpAwareOperatorPolicyV0",
    "AssistantOutputReviewV0", "OneSentenceStatusV0", "BuildCockpitCardV0",
    "OperatorStyleProfileV0", "AssistantOutputModeV0", "ReadOnlyVerificationModeV0",
    "BoundaryStopReportV0", "PlanActionTraceV0", "FutureMissionCandidateV0",
    "S10AssistantOptimizerProofV0",
];

pub const BUILD_ACCELERATOR_VALIDATION_TARGETS: &[&str] = &[
    "one-drop", "mission-approval", "approval-compression", "no-midpoint-ask",
    "tool-call-bundling", "repo-mutation-batch", "batch-repair", "merge-aware-handoff",
    "no-cleanup-pr", "state-consistency", "phase-truth", "branch-lifecycle", "pr-lifecycle",
    "proof-selector", "required-checks", "workflow-path-filter", "ci-wave-counter",
    "repair-cause-memory", "validator-registration", "schema-inventory", "workspace-registration",
    "obsolete-text", "doc-conflicts", "merge-readiness-red-team", "build-velocity",
    "confirmation-friction", "human-availability", "tool-fallback", "existing-work-reuse",
    "partial-completion-recovery", "no-silent-downgrade", "capability-activation-ledger",
    "future-phase-contract", "next-tab-prompt",
    "assistant-optimizer", "assistant-operating-profile", "mission-intake", "context-load-policy",
    "repo-truth-first", "tool-use-policy", "tool-fallback-policy", "low-attention-mode",
    "progress-update-policy", "handoff-quality", "prompt-compression", "scope-drift-guard",
    "decision-carryover", "proof-awareness", "repair-intelligence", "capability-map",
    "read-only-verification", "assistant-self-audit", "operator-mistake-detector",
    "build-cockpit", "trace-digest", "mcp-aware-operator-policy", "output-mode-selector",
    "future-mission-recommender",
];

pub const ONE_DROP_STEPS: &[&str] = &[
    "mission_approval", "preflight_state_snapshot", "coherent_repo_mutation", "one_pr",
    "one_proof_wave", "batched_repairs", "merge_when_green", "post_merge_truth_resolution",
];

pub const ROUTINE_ACTION_CLASSES: &[&str] = &[
    "safe_file_creation", "safe_file_update", "branch_creation", "commit_creation",
    "pr_creation", "ci_inspection", "exact_failure_repair", "docs_state_update", "merge_when_green",
];

pub const BOUNDARY_ACTION_CLASSES: &[&str] = &[
    "secrets", "paid_compute", "private_data_exposure", "account_permission_changes",
    "production_deployment", "database_mutation", "browser_cookie_session_automation",
    "destructive_irreversible_action", "unresolved_high_impact_ambiguity",
];

pub const BATCH_REPAIR_RULES: &[&str] = &[
    "inspect_all_failures_before_patch", "group_related_repairs", "patch_exact_failures_only",
    "rerun_after_batch", "do_not_add_truth_only_rerun_commit_unless_required",
];

pub const MERGE_HANDOFF_RULES: &[&str] = &[
    "pre_merge_truth_template", "post_merge_truth_template", "merge_sha_resolution_field",
    "no_stale_pending_language", "next_action_survives_merge",
];

pub const STATE_CONSISTENCY_RULES: &[&str] = &[
    "readme_build_plan_match", "active_relay_seal_match", "next_action_matches_phase",
    "no_obsolete_phase_reference", "no_duplicate_conflicting_doc_truth",
];

pub const LIFECYCLE_STATES: &[&str] = &[
    "planned", "branch_active", "pr_open", "ci_running", "repairing", "green", "merged", "blocked",
];

pub const PROOF_SELECTION_RULES: &[&str] = &[
    "touched_surface_declared", "required_checks_manifested", "path_filters_simulated",
    "unexpected_skip_detected", "ci_wave_counted",
];

pub const REGISTRATION_GUARDS: &[&str] = &[
    "validator_registered", "schema_inventory_registered", "workspace_member_registered",
    "cli_dependency_registered", "workflow_registered",
];

pub const HUMAN_ATTENTION_RULES: &[&str] = &[
    "mission_approval_reuse", "no_midpoint_ask_for_routine_action", "friction_event_recorded",
    "low_attention_workday_supported",
];

pub const TOOL_FALLBACK_RULES: &[&str] = &[
    "prefer_git_tree_batch", "fall_back_to_contents_api", "fall_back_to_pr_patch", "stop_at_true_boundary",
];

pub const RECOVERY_RULES: &[&str] = &[
    "existing_branch_reuse_checked", "existing_pr_reuse_checked", "partial_drop_recovery_plan",
    "saturation_handoff_prompt",
];

pub const NO_SILENT_DOWNGRADE_RULES: &[&str] = &[
    "required_validator_not_removed", "required_schema_not_removed", "required_workflow_not_removed",
    "proof_gate_not_weakened", "safety_boundary_not_relaxed",
];

pub const FUTURE_PHASE_CONTRACT_RULES: &[&str] = &[
    "one_drop_default", "mission_approval_default", "proof_plan_required", "state_update_required",
    "final_report_required", "next_tab_prompt_required",
];

pub const REQUIRED_S9_DOCS: &[&str] = &[
    "docs/ONE_DROP_BUILD_MODE.md", "docs/MISSION_APPROVAL_ENVELOPE.md",
    "docs/BATCH_REPAIR_POLICY.md", "docs/MERGE_AWARE_HANDOFF.md",
    "docs/PHASE_TEMPLATE_SYSTEM.md", "docs/S9_FINAL_REPORT.md",
];

pub const REQUIRED_S9_PROMPT_ARTIFACTS: &[&str] = &[
    "docs/PROMPTS/NEXT_TAB_PROMPT.md", "docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md",
];

pub const VELOCITY_METRICS: &[&str] = &[
    "mission_approvals", "repo_mutation_batches", "pull_requests", "ci_waves",
    "repair_commits", "cleanup_prs_avoided",
];

pub const FRICTION_METRICS: &[&str] = &[
    "routine_confirmations_avoided", "true_boundaries_detected", "tool_calls_batched",
    "human_attention_events",
];

fn has(values: &[&str], value: &str) -> bool { values.contains(&value) }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildAccelerationVerdict { Accelerated, NeedsBatching, BlockedByBoundary, Invalid }

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
    if summary.true_boundary_hit { return BuildAccelerationVerdict::BlockedByBoundary; }
    if !summary.mission_approved || summary.pull_requests == 0 || summary.repo_mutation_batches == 0 {
        return BuildAccelerationVerdict::Invalid;
    }
    if summary.repo_mutation_batches == 1 && summary.pull_requests == 1 && summary.ci_waves <= 2 && summary.repair_commits_batched && summary.cleanup_prs_needed == 0 {
        return BuildAccelerationVerdict::Accelerated;
    }
    BuildAccelerationVerdict::NeedsBatching
}

pub fn is_build_accelerator_schema(name: &str) -> bool { has(BUILD_ACCELERATOR_PACKET_SCHEMAS, name) }
pub fn is_build_accelerator_validation_target(target: &str) -> bool { has(BUILD_ACCELERATOR_VALIDATION_TARGETS, target) }
pub fn has_one_drop_step(step: &str) -> bool { has(ONE_DROP_STEPS, step) }
pub fn is_routine_action_class(action: &str) -> bool { has(ROUTINE_ACTION_CLASSES, action) }
pub fn is_boundary_action_class(action: &str) -> bool { has(BOUNDARY_ACTION_CLASSES, action) }
pub fn has_batch_repair_rule(rule: &str) -> bool { has(BATCH_REPAIR_RULES, rule) }
pub fn has_merge_handoff_rule(rule: &str) -> bool { has(MERGE_HANDOFF_RULES, rule) }
pub fn has_state_consistency_rule(rule: &str) -> bool { has(STATE_CONSISTENCY_RULES, rule) }
pub fn has_lifecycle_state(state: &str) -> bool { has(LIFECYCLE_STATES, state) }
pub fn has_proof_selection_rule(rule: &str) -> bool { has(PROOF_SELECTION_RULES, rule) }
pub fn has_registration_guard(rule: &str) -> bool { has(REGISTRATION_GUARDS, rule) }
pub fn has_human_attention_rule(rule: &str) -> bool { has(HUMAN_ATTENTION_RULES, rule) }
pub fn has_tool_fallback_rule(rule: &str) -> bool { has(TOOL_FALLBACK_RULES, rule) }
pub fn has_recovery_rule(rule: &str) -> bool { has(RECOVERY_RULES, rule) }
pub fn has_no_silent_downgrade_rule(rule: &str) -> bool { has(NO_SILENT_DOWNGRADE_RULES, rule) }
pub fn has_future_phase_contract_rule(rule: &str) -> bool { has(FUTURE_PHASE_CONTRACT_RULES, rule) }
pub fn has_required_doc(path: &str) -> bool { has(REQUIRED_S9_DOCS, path) }
pub fn has_required_prompt_artifact(path: &str) -> bool { has(REQUIRED_S9_PROMPT_ARTIFACTS, path) }
pub fn has_velocity_metric(metric: &str) -> bool { has(VELOCITY_METRICS, metric) }
pub fn has_friction_metric(metric: &str) -> bool { has(FRICTION_METRICS, metric) }

#[cfg(test)]
mod tests {
    use super::*;
    const ONE_DROP_SCHEMA: &str = include_str!("../../../schemas/OneDropPlanV0.schema.json");
    const MISSION_APPROVAL_SCHEMA: &str = include_str!("../../../schemas/MissionApprovalEnvelopeV0.schema.json");
    const STATE_REPORT_SCHEMA: &str = include_str!("../../../schemas/StateConsistencyReportV0.schema.json");
    const VELOCITY_SCHEMA: &str = include_str!("../../../schemas/BuildVelocityReportV0.schema.json");

    #[test]
    fn schema_inventory_contains_required_s9_contracts() {
        assert!(is_build_accelerator_schema("OneDropPlanV0"));
        assert!(is_build_accelerator_schema("MissionApprovalEnvelopeV0"));
        assert!(is_build_accelerator_schema("S9BuildAcceleratorProofV0"));
    }

    #[test]
    fn schema_inventory_contains_s10_bridge_contracts() {
        assert!(is_build_accelerator_schema("AssistantOperatingProfileV0"));
        assert!(is_build_accelerator_schema("MissionIntakePacketV0"));
        assert!(is_build_accelerator_schema("S10AssistantOptimizerProofV0"));
        assert!(is_build_accelerator_validation_target("assistant-optimizer"));
        assert!(is_build_accelerator_validation_target("mcp-aware-operator-policy"));
    }

    #[test]
    fn schema_files_are_materialized() {
        assert!(ONE_DROP_SCHEMA.contains("OneDropPlanV0"));
        assert!(MISSION_APPROVAL_SCHEMA.contains("MissionApprovalEnvelopeV0"));
        assert!(STATE_REPORT_SCHEMA.contains("StateConsistencyReportV0"));
        assert!(VELOCITY_SCHEMA.contains("BuildVelocityReportV0"));
    }

    #[test]
    fn mission_approval_does_not_cover_true_boundaries() {
        assert!(is_routine_action_class("safe_file_update"));
        assert!(is_boundary_action_class("secrets"));
        assert!(!is_routine_action_class("browser_cookie_session_automation"));
    }

    #[test]
    fn accelerated_build_run_requires_one_pr_and_no_cleanup_pr() {
        let summary = BuildRunSummary { mission_approved: true, repo_mutation_batches: 1, pull_requests: 1, ci_waves: 1, repair_commits_batched: true, cleanup_prs_needed: 0, true_boundary_hit: false };
        assert_eq!(evaluate_build_run(summary), BuildAccelerationVerdict::Accelerated);
    }
}
