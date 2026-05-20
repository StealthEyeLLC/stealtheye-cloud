//! S11 One-Accept Mission Executor contracts for StealthEye Cloud.
//!
//! S11 turns one explicit mission lease into bounded GitHub-native repo/build/proof
//! execution. It does not bypass GitHub permissions and it never pre-approves
//! secrets, paid compute, production deployment, database mutation, private data
//! exposure, browser-cookie/session-token automation, destructive irreversible
//! action, scope expansion, legal signoff, or GitHub permission bypass.

pub const MISSION_EXECUTOR_PACKET_SCHEMAS: &[&str] = &[
    "GitHubCapabilityPreflightV0",
    "ActionsWritePermissionCheckV0",
    "ActionsPrCreationCheckV0",
    "WorkflowDispatchCapabilityV0",
    "RepositoryDispatchCapabilityV0",
    "WorkflowTokenLimitFindingV0",
    "GitHubAppUpgradeDecisionV0",
    "BranchRulesetCompatibilityCheckV0",
    "MergePermissionCheckV0",
    "RequiredChecksResolverV0",
    "BoundaryStopForGitHubPermissionV0",
    "MissionLeaseV0",
    "MissionAuthorityEnvelopeV0",
    "OneAcceptMissionV0",
    "ApprovedRoutineActionSetV0",
    "ForbiddenActionSetV0",
    "MissionStopConditionSetV0",
    "MissionBudgetV0",
    "MissionScopeHashV0",
    "MissionPolicySimulatorV0",
    "MissionExecutorRequestV0",
    "MissionExecutorWorkflowV0",
    "MissionExecutorStateV0",
    "MissionExecutorReportV0",
    "BatchRepoMutationV0",
    "FileSetPatchV0",
    "ExpectedShaGuardV0",
    "AtomicCommitPlanV0",
    "RollbackSnapshotV0",
    "MutationReceiptV0",
    "BranchControllerV0",
    "ExistingBranchReuseV0",
    "PrControllerV0",
    "ExistingPrReuseV0",
    "ProofControllerV0",
    "WorkflowRunInspectorV0",
    "CombinedStatusFallbackV0",
    "CurrentHeadShaProofV0",
    "ProofFreshnessCheckV0",
    "ProofRepairLoopV0",
    "FailureClusterV0",
    "RepairBatchV0",
    "RetryBudgetV0",
    "KnownFailureAutopatchV0",
    "MergeWhenGreenControllerV0",
    "MergePolicyV0",
    "RequiredChecksGreenV0",
    "PostMergeVerificationV0",
    "PostMergeProofFreshnessGateV0",
    "DirectMainMutationRiskV0",
    "PostMergeTruthCommitPolicyV0",
    "WorkflowDispatchAfterDirectMutationV0",
    "CurrentMainHeadProofV0",
    "NoUnverifiedTruthCommitPolicyV0",
    "BoundaryStopV0",
    "ResumeAfterBoundaryPlanV0",
    "MissionJournalV0",
    "ActionReceiptV0",
    "ProofReceiptV0",
    "RepairReceiptV0",
    "MergeReceiptV0",
    "IdempotencyKeyV0",
    "ActionDedupReceiptV0",
    "MissionStatePersistenceV0",
    "ToolDescriptorSnapshotV0",
    "ToolResultTaintV0",
    "McpAuthorizationGuardV0",
    "ApprovalCountReportV0",
    "NoRoutineMidpointApprovalProofV0",
    "OneAcceptDemoMissionV0",
    "MissionExecutorProofV0",
];

pub const MISSION_EXECUTOR_VALIDATION_TARGETS: &[&str] = &[
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

pub const ROUTINE_ACTIONS: &[&str] = &[
    "read_repo",
    "create_or_reuse_branch",
    "batch_create_update_delete_files",
    "commit_and_push",
    "open_or_reuse_pr",
    "inspect_ci_and_logs",
    "classify_proof_failures",
    "patch_exact_failures",
    "rerun_proof",
    "update_state_handoff_final_report",
    "merge_when_green_if_allowed",
];

pub const FORBIDDEN_ACTIONS: &[&str] = &[
    "secrets",
    "paid_compute",
    "production_deployment",
    "database_mutation",
    "account_permission_changes",
    "private_data_exposure",
    "browser_cookie_session_token_automation",
    "destructive_irreversible_action",
    "scope_expansion",
    "unapproved_external_posting",
    "legal_compliance_signoff",
    "github_permission_bypass",
];

pub const STOP_CONDITIONS: &[&str] = &[
    "github_permission_boundary",
    "branch_ruleset_boundary",
    "required_review_boundary",
    "forbidden_action_requested",
    "retry_budget_exhausted",
    "scope_hash_mismatch",
    "fresh_main_proof_required",
];

pub const REQUIRED_ARTIFACTS: &[&str] = &[
    ".stealtheye/mission-executor/github-capability-preflight.json",
    ".stealtheye/mission-executor/mission-lease.json",
    ".stealtheye/mission-executor/mission-executor-request.json",
    ".stealtheye/mission-executor/mission-plan.json",
    ".stealtheye/mission-executor/batch-repo-mutation.json",
    ".stealtheye/mission-executor/pr-controller-report.json",
    ".stealtheye/mission-executor/proof-controller-report.json",
    ".stealtheye/mission-executor/proof-repair-loop-report.json",
    ".stealtheye/mission-executor/merge-when-green-report.json",
    ".stealtheye/mission-executor/post-merge-proof-freshness-report.json",
    ".stealtheye/mission-executor/boundary-stop-report.json",
    ".stealtheye/mission-executor/mission-journal.jsonl",
    ".stealtheye/mission-executor/approval-count-report.json",
    ".stealtheye/mission-executor/mission-executor-proof.json",
];

pub const CAPABILITY_CHECKS: &[&str] = &[
    "actions_can_write_contents",
    "actions_can_create_prs",
    "workflow_dispatch_available",
    "repository_dispatch_available",
    "workflow_runs_inspectable",
    "failed_jobs_rerunnable",
    "required_checks_resolved",
    "merge_permission_checked",
    "branch_rulesets_checked",
];

fn has(values: &[&str], value: &str) -> bool {
    values.contains(&value)
}

pub fn is_mission_executor_schema(name: &str) -> bool {
    has(MISSION_EXECUTOR_PACKET_SCHEMAS, name)
}

pub fn is_mission_executor_validation_target(target: &str) -> bool {
    has(MISSION_EXECUTOR_VALIDATION_TARGETS, target)
}

pub fn is_routine_action(action: &str) -> bool {
    has(ROUTINE_ACTIONS, action)
}

pub fn is_forbidden_action(action: &str) -> bool {
    has(FORBIDDEN_ACTIONS, action)
}

pub fn is_stop_condition(condition: &str) -> bool {
    has(STOP_CONDITIONS, condition)
}

pub fn is_required_artifact(path: &str) -> bool {
    has(REQUIRED_ARTIFACTS, path)
}

pub fn has_capability_check(check: &str) -> bool {
    has(CAPABILITY_CHECKS, check)
}

pub fn requires_fresh_main_proof_for_direct_truth_commit(direct_main_mutation: bool) -> bool {
    direct_main_mutation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionExecutorVerdict {
    OneAcceptReady,
    BoundaryStop,
    NeedsFreshMainProof,
    Invalid,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MissionLeaseSummary {
    pub initial_approval_count: u16,
    pub routine_midpoint_approvals: u16,
    pub forbidden_actions_requested: u16,
    pub true_boundary_hit: bool,
    pub github_permission_blocked: bool,
    pub direct_post_merge_truth_commit: bool,
    pub fresh_main_head_proof_recorded: bool,
}

pub fn evaluate_mission_lease(summary: MissionLeaseSummary) -> MissionExecutorVerdict {
    if summary.forbidden_actions_requested > 0 || summary.true_boundary_hit || summary.github_permission_blocked {
        return MissionExecutorVerdict::BoundaryStop;
    }
    if summary.direct_post_merge_truth_commit && !summary.fresh_main_head_proof_recorded {
        return MissionExecutorVerdict::NeedsFreshMainProof;
    }
    if summary.initial_approval_count != 1 || summary.routine_midpoint_approvals != 0 {
        return MissionExecutorVerdict::Invalid;
    }
    MissionExecutorVerdict::OneAcceptReady
}

#[cfg(test)]
mod tests {
    use super::*;

    const LEASE_SCHEMA: &str = include_str!("../../../schemas/MissionLeaseV0.schema.json");
    const PREFLIGHT_SCHEMA: &str = include_str!("../../../schemas/GitHubCapabilityPreflightV0.schema.json");
    const REQUEST_SCHEMA: &str = include_str!("../../../schemas/MissionExecutorRequestV0.schema.json");
    const PROOF_SCHEMA: &str = include_str!("../../../schemas/MissionExecutorProofV0.schema.json");

    #[test]
    fn inventory_contains_required_s11_contracts() {
        assert!(is_mission_executor_schema("MissionLeaseV0"));
        assert!(is_mission_executor_schema("MissionExecutorRequestV0"));
        assert!(is_mission_executor_schema("ProofRepairLoopV0"));
        assert!(is_mission_executor_schema("PostMergeProofFreshnessGateV0"));
        assert!(is_mission_executor_schema("NoUnverifiedTruthCommitPolicyV0"));
        assert!(is_mission_executor_schema("MissionExecutorProofV0"));
    }

    #[test]
    fn core_schema_files_are_materialized() {
        assert!(LEASE_SCHEMA.contains("MissionLeaseV0"));
        assert!(PREFLIGHT_SCHEMA.contains("GitHubCapabilityPreflightV0"));
        assert!(REQUEST_SCHEMA.contains("MissionExecutorRequestV0"));
        assert!(PROOF_SCHEMA.contains("MissionExecutorProofV0"));
    }

    #[test]
    fn lease_allows_routine_actions_only() {
        assert!(is_routine_action("read_repo"));
        assert!(is_routine_action("merge_when_green_if_allowed"));
        assert!(is_forbidden_action("secrets"));
        assert!(is_forbidden_action("github_permission_bypass"));
        assert!(!is_routine_action("secrets"));
        assert!(!is_routine_action("scope_expansion"));
    }

    #[test]
    fn one_accept_ready_requires_zero_midpoint_approvals() {
        let summary = MissionLeaseSummary {
            initial_approval_count: 1,
            routine_midpoint_approvals: 0,
            forbidden_actions_requested: 0,
            true_boundary_hit: false,
            github_permission_blocked: false,
            direct_post_merge_truth_commit: false,
            fresh_main_head_proof_recorded: false,
        };
        assert_eq!(evaluate_mission_lease(summary), MissionExecutorVerdict::OneAcceptReady);
    }

    #[test]
    fn direct_post_merge_truth_requires_fresh_main_head_proof() {
        let summary = MissionLeaseSummary {
            initial_approval_count: 1,
            routine_midpoint_approvals: 0,
            forbidden_actions_requested: 0,
            true_boundary_hit: false,
            github_permission_blocked: false,
            direct_post_merge_truth_commit: true,
            fresh_main_head_proof_recorded: false,
        };
        assert_eq!(evaluate_mission_lease(summary), MissionExecutorVerdict::NeedsFreshMainProof);
        assert!(requires_fresh_main_proof_for_direct_truth_commit(true));
    }

    #[test]
    fn required_artifacts_cover_demo_path() {
        assert!(is_required_artifact(".stealtheye/mission-executor/mission-journal.jsonl"));
        assert!(is_required_artifact(".stealtheye/mission-executor/approval-count-report.json"));
        assert!(has_capability_check("workflow_dispatch_available"));
        assert!(is_stop_condition("github_permission_boundary"));
    }
}
