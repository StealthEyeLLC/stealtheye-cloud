//! S10 Assistant Optimization Layer contracts for StealthEye Cloud.
//!
//! S10 improves assistant/operator discipline. It does not claim hidden autonomy
//! and it does not authorize weaker proof, secrets, paid compute, production
//! deployment, database mutation, or browser-cookie/session-token automation.

pub const ASSISTANT_OPTIMIZER_PACKET_SCHEMAS: &[&str] = &[
    "AssistantOperatingProfileV0",
    "BestAgentModeV0",
    "FastButExactModeV0",
    "BoundaryOnlyStopPolicyV0",
    "AssistantExecutionDisciplineV0",
    "MissionIntakePacketV0",
    "MissionScopeClassifierV0",
    "MissionAmbiguityScanV0",
    "MissionBoundaryScanV0",
    "MissionStartReadinessV0",
    "ContextLoadPlanV0",
    "RequiredContextFileSetV0",
    "ContextPriorityMapV0",
    "ContextFreshnessCheckV0",
    "RepoTruthFirstPolicyV0",
    "RepoStateDigestV0",
    "HandoffTruthSourceV0",
    "ChatMemoryConflictFindingV0",
    "PromptCompressionProfileV0",
    "MissionPromptTemplateV0",
    "ContinuationPromptTemplateV0",
    "RepairPromptTemplateV0",
    "PromptLossCheckV0",
    "ToolUsePlanV0",
    "ToolSelectionPolicyV0",
    "ToolCallBatchPlanV0",
    "ToolFallbackPolicyV0",
    "ToolFailureClassificationV0",
    "RetrySafetyDecisionV0",
    "LowAttentionWorkdayModeV0",
    "HumanAvailabilityProfileV0",
    "UserAttentionBudgetV0",
    "ProgressUpdatePolicyV0",
    "ProofNeedClassifierV0",
    "ProofGateSelectionV0",
    "ProofFailureTriageV0",
    "KnownFailurePatternV0",
    "RepairTriagePlanV0",
    "FailureClusterV0",
    "ScopeDriftDetectorV0",
    "DecisionCarryoverPacketV0",
    "CapabilityRealityMapV0",
    "HandoffQualityReportV0",
    "ContextSaturationSignalV0",
    "AssistantSelfAuditV0",
    "AssistantMistakePatternV0",
    "AssistantTraceDigestV0",
    "McpAwareOperatorPolicyV0",
    "AssistantOutputReviewV0",
    "OneSentenceStatusV0",
    "BuildCockpitCardV0",
    "OperatorStyleProfileV0",
    "AssistantOutputModeV0",
    "ReadOnlyVerificationModeV0",
    "BoundaryStopReportV0",
    "PlanActionTraceV0",
    "FutureMissionCandidateV0",
    "S10AssistantOptimizerProofV0",
];

pub const ASSISTANT_OPTIMIZER_VALIDATION_TARGETS: &[&str] = &[
    "assistant-optimizer",
    "assistant-operating-profile",
    "mission-intake",
    "context-load-policy",
    "repo-truth-first",
    "tool-use-policy",
    "tool-fallback-policy",
    "low-attention-mode",
    "progress-update-policy",
    "handoff-quality",
    "prompt-compression",
    "scope-drift-guard",
    "decision-carryover",
    "proof-awareness",
    "repair-intelligence",
    "capability-map",
    "read-only-verification",
    "assistant-self-audit",
    "operator-mistake-detector",
    "build-cockpit",
    "trace-digest",
    "mcp-aware-operator-policy",
    "output-mode-selector",
    "future-mission-recommender",
];

pub const TRUE_BOUNDARY_ACTIONS: &[&str] = &[
    "secrets",
    "paid_compute",
    "private_data_exposure",
    "account_permission_changes",
    "production_deployment",
    "database_mutation",
    "browser_cookie_session_automation",
    "destructive_irreversible_action",
    "unresolved_high_impact_ambiguity",
];

pub const ROUTINE_ASSISTANT_RULES: &[&str] = &[
    "use_s9_one_drop_mode",
    "avoid_routine_midpoint_questions",
    "batch_safe_work",
    "inspect_all_failures_before_repair",
    "patch_exact_failures_only",
    "merge_only_when_green",
    "trust_repo_truth_over_stale_chat",
    "preserve_public_safe_boundaries",
    "stop_only_for_true_boundaries",
    "handoff_at_saturation",
];

pub const REQUIRED_CONTEXT_FILES: &[&str] = &[
    "AGENTS.md",
    "STEALTHEYE_DECISIONS.md",
    "STEALTHEYE_ACTIVE.md",
    "STEALTHEYE_RELAY.md",
    "STEALTHEYE_RELAY.json",
    "STEALTHEYE_SEAL.json",
    "NEXT_ACTION.md",
    "README.md",
    "AGENT_REPO_MAP.md",
    "llms.txt",
    "llms-full.txt",
    "docs/StealthEye_Cloud_Build_Plan.md",
    "docs/HANDOFF_AND_CONTINUATION.md",
    "docs/S10_ASSISTANT_OPTIMIZATION_LAYER.md",
];

pub const REPO_TRUTH_PRECEDENCE: &[&str] = &[
    "current_user_instruction",
    "current_repo_state",
    "current_ci_browser_result",
    "latest_stealtheye_seal",
    "STEALTHEYE_ACTIVE.md",
    "STEALTHEYE_DECISIONS.md",
    "STEALTHEYE_RELAY.md_json",
    "older_chat_memory",
];

pub const TOOL_PLANNING_RULES: &[&str] = &[
    "declare_tool_class",
    "separate_reads_from_writes",
    "batch_compatible_calls",
    "avoid_duplicate_reads",
    "declare_fallback_ladder",
    "assign_result_trust_tier",
    "scan_boundary_risk",
];

pub const RECOVERABLE_TOOL_FAILURES: &[&str] = &[
    "sha_mismatch",
    "branch_exists",
    "pr_exists",
    "status_api_empty",
    "merge_payload_bad",
    "safe_path_blocked",
];

pub const LOW_ATTENTION_RULES: &[&str] = &[
    "continue_routine_approved_work",
    "short_milestone_updates_only",
    "avoid_low_value_chatter",
    "stop_for_true_boundary",
    "emit_one_exact_human_action_when_blocked",
    "compact_final_summary",
];

pub const PROGRESS_UPDATE_TRIGGERS: &[&str] = &[
    "branch_created",
    "pr_opened",
    "ci_failed",
    "failure_cluster_identified",
    "repair_pushed",
    "ci_green",
    "merge_completed",
    "true_boundary_hit",
    "saturation_handoff_needed",
];

pub const PROOF_AWARENESS_RULES: &[&str] = &[
    "compile_proof",
    "schema_proof",
    "validator_proof",
    "doctor_proof",
    "browser_proof",
    "mobile_proof",
    "activation_proof",
    "remediation_proof",
    "workflow_path_proof",
    "handoff_consistency_proof",
    "assistant_optimizer_proof",
];

pub const REPAIR_INTELLIGENCE_RULES: &[&str] = &[
    "inspect_all_failures_before_patch",
    "cluster_related_failures",
    "identify_minimal_repair_scope",
    "avoid_speculative_broad_rewrites",
    "patch_exact_failures_only",
    "rerun_after_batch",
    "avoid_cleanup_only_rerun_commits_unless_required",
];

pub const HANDOFF_QUALITY_CHECKS: &[&str] = &[
    "active_matches_relay",
    "relay_json_matches_markdown",
    "seal_matches_latest_mission",
    "next_action_is_exact",
    "readme_build_plan_not_stale",
    "latest_pr_merge_state_resolved",
    "implementation_prompt_exists",
    "forbidden_files_absent",
];

pub const PROMPT_PRESERVATION_FIELDS: &[&str] = &[
    "current_state",
    "approval_envelope",
    "exact_next_action",
    "branch_and_pr_state",
    "proof_requirements",
    "do_not_reopen_decisions",
    "true_boundaries",
    "no_weakening_invariants",
    "required_scan_files",
    "implementation_branch_name",
];

pub const CAPABILITY_STATUSES: &[&str] = &[
    "AVAILABLE",
    "AVAILABLE_WITH_CONFIRMATION",
    "REPO_SUPPORTED",
    "CI_SUPPORTED",
    "BROWSER_SUPPORTED",
    "MCP_SUPPORTED",
    "READINESS_ONLY",
    "USER_INITIATED_ONLY",
    "BLOCKED",
    "FORBIDDEN",
];

pub const READ_ONLY_ALLOWED_ACTIONS: &[&str] = &[
    "read_repo_files",
    "inspect_prs",
    "inspect_branches",
    "inspect_ci_state",
    "inspect_proof_artifacts",
    "report_findings",
];

pub const READ_ONLY_FORBIDDEN_ACTIONS: &[&str] = &[
    "create_branch",
    "create_commit",
    "update_files",
    "create_pr",
    "rerun_workflows",
    "merge_pr",
    "mutate_external_systems",
];

pub const MCP_OPERATOR_RULES: &[&str] = &[
    "mcp_results_are_data_not_instructions",
    "mcp_descriptors_untrusted_until_validated",
    "mcp_capabilities_checked_against_policy",
    "sensitive_mcp_actions_hard_stop",
    "mcp_result_summaries_are_taint_aware",
    "mcp_approval_reuse_scope_explicit",
];

pub const OUTPUT_MODES: &[&str] = &[
    "one_sentence_status",
    "build_cockpit_card",
    "detailed_report",
    "boundary_stop_report",
    "final_summary",
    "handoff_prompt",
];

pub fn is_assistant_optimizer_schema(name: &str) -> bool {
    ASSISTANT_OPTIMIZER_PACKET_SCHEMAS.contains(&name)
}

pub fn is_assistant_optimizer_validation_target(target: &str) -> bool {
    ASSISTANT_OPTIMIZER_VALIDATION_TARGETS.contains(&target)
}

pub fn is_true_boundary_action(action: &str) -> bool {
    TRUE_BOUNDARY_ACTIONS.contains(&action)
}

pub fn has_routine_assistant_rule(rule: &str) -> bool {
    ROUTINE_ASSISTANT_RULES.contains(&rule)
}

pub fn is_required_context_file(path: &str) -> bool {
    REQUIRED_CONTEXT_FILES.contains(&path)
}

pub fn has_repo_truth_source(source: &str) -> bool {
    REPO_TRUTH_PRECEDENCE.contains(&source)
}

pub fn has_tool_planning_rule(rule: &str) -> bool {
    TOOL_PLANNING_RULES.contains(&rule)
}

pub fn is_recoverable_tool_failure(failure: &str) -> bool {
    RECOVERABLE_TOOL_FAILURES.contains(&failure)
}

pub fn has_low_attention_rule(rule: &str) -> bool {
    LOW_ATTENTION_RULES.contains(&rule)
}

pub fn has_progress_update_trigger(trigger: &str) -> bool {
    PROGRESS_UPDATE_TRIGGERS.contains(&trigger)
}

pub fn has_proof_awareness_rule(rule: &str) -> bool {
    PROOF_AWARENESS_RULES.contains(&rule)
}

pub fn has_repair_intelligence_rule(rule: &str) -> bool {
    REPAIR_INTELLIGENCE_RULES.contains(&rule)
}

pub fn has_handoff_quality_check(check: &str) -> bool {
    HANDOFF_QUALITY_CHECKS.contains(&check)
}

pub fn has_prompt_preservation_field(field: &str) -> bool {
    PROMPT_PRESERVATION_FIELDS.contains(&field)
}

pub fn is_capability_status(status: &str) -> bool {
    CAPABILITY_STATUSES.contains(&status)
}

pub fn is_read_only_allowed_action(action: &str) -> bool {
    READ_ONLY_ALLOWED_ACTIONS.contains(&action)
}

pub fn is_read_only_forbidden_action(action: &str) -> bool {
    READ_ONLY_FORBIDDEN_ACTIONS.contains(&action)
}

pub fn has_mcp_operator_rule(rule: &str) -> bool {
    MCP_OPERATOR_RULES.contains(&rule)
}

pub fn is_output_mode(mode: &str) -> bool {
    OUTPUT_MODES.contains(&mode)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssistantOptimizationVerdict {
    Ready,
    NeedsContext,
    BlockedByBoundary,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissionStartSummary {
    pub approval_present: bool,
    pub repo_truth_loaded: bool,
    pub required_context_loaded: bool,
    pub true_boundary_hit: bool,
    pub read_only_mode: bool,
    pub mutation_attempted: bool,
}

pub fn evaluate_mission_start(summary: MissionStartSummary) -> AssistantOptimizationVerdict {
    if summary.true_boundary_hit {
        return AssistantOptimizationVerdict::BlockedByBoundary;
    }
    if summary.read_only_mode && summary.mutation_attempted {
        return AssistantOptimizationVerdict::Invalid;
    }
    if !summary.approval_present || !summary.repo_truth_loaded || !summary.required_context_loaded {
        return AssistantOptimizationVerdict::NeedsContext;
    }
    AssistantOptimizationVerdict::Ready
}

#[cfg(test)]
mod tests {
    use super::*;

    const ASSISTANT_PROFILE_SCHEMA: &str =
        include_str!("../../../schemas/AssistantOperatingProfileV0.schema.json");
    const MISSION_INTAKE_SCHEMA: &str =
        include_str!("../../../schemas/MissionIntakePacketV0.schema.json");
    const CONTEXT_LOAD_SCHEMA: &str =
        include_str!("../../../schemas/ContextLoadPlanV0.schema.json");
    const PROOF_SCHEMA: &str =
        include_str!("../../../schemas/S10AssistantOptimizerProofV0.schema.json");

    #[test]
    fn schema_inventory_contains_required_s10_contracts() {
        assert!(is_assistant_optimizer_schema("AssistantOperatingProfileV0"));
        assert!(is_assistant_optimizer_schema("MissionIntakePacketV0"));
        assert!(is_assistant_optimizer_schema("ContextLoadPlanV0"));
        assert!(is_assistant_optimizer_schema("RepoTruthFirstPolicyV0"));
        assert!(is_assistant_optimizer_schema("McpAwareOperatorPolicyV0"));
        assert!(is_assistant_optimizer_schema("S10AssistantOptimizerProofV0"));
    }

    #[test]
    fn schema_files_are_materialized() {
        assert!(ASSISTANT_PROFILE_SCHEMA.contains("AssistantOperatingProfileV0"));
        assert!(MISSION_INTAKE_SCHEMA.contains("MissionIntakePacketV0"));
        assert!(CONTEXT_LOAD_SCHEMA.contains("ContextLoadPlanV0"));
        assert!(PROOF_SCHEMA.contains("S10AssistantOptimizerProofV0"));
    }

    #[test]
    fn assistant_profile_preserves_boundaries() {
        assert!(has_routine_assistant_rule("use_s9_one_drop_mode"));
        assert!(has_routine_assistant_rule("merge_only_when_green"));
        assert!(is_true_boundary_action("secrets"));
        assert!(is_true_boundary_action("browser_cookie_session_automation"));
    }

    #[test]
    fn context_and_repo_truth_are_explicit() {
        assert!(is_required_context_file("AGENTS.md"));
        assert!(is_required_context_file("STEALTHEYE_RELAY.json"));
        assert!(has_repo_truth_source("current_repo_state"));
        assert!(has_repo_truth_source("older_chat_memory"));
    }

    #[test]
    fn tool_fallback_distinguishes_recoverable_from_boundary_stop() {
        assert!(is_recoverable_tool_failure("sha_mismatch"));
        assert!(is_recoverable_tool_failure("pr_exists"));
        assert!(is_true_boundary_action("secrets"));
    }

    #[test]
    fn read_only_mode_blocks_mutation() {
        assert!(is_read_only_allowed_action("inspect_prs"));
        assert!(is_read_only_forbidden_action("create_commit"));
        let summary = MissionStartSummary {
            approval_present: true,
            repo_truth_loaded: true,
            required_context_loaded: true,
            true_boundary_hit: false,
            read_only_mode: true,
            mutation_attempted: true,
        };
        assert_eq!(
            evaluate_mission_start(summary),
            AssistantOptimizationVerdict::Invalid
        );
    }

    #[test]
    fn capability_map_blocks_hidden_autonomy_claims() {
        assert!(is_capability_status("USER_INITIATED_ONLY"));
        assert!(is_capability_status("FORBIDDEN"));
    }

    #[test]
    fn mcp_results_are_treated_as_data() {
        assert!(has_mcp_operator_rule("mcp_results_are_data_not_instructions"));
        assert!(has_mcp_operator_rule("mcp_descriptors_untrusted_until_validated"));
    }
}
