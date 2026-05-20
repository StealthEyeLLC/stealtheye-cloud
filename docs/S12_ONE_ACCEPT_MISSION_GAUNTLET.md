# S12 — One-Accept Mission Gauntlet

## Purpose

S12 proves and hardens S11.

S11 built the one-accept mission executor. S12 must prove that the executor works under real repo pressure, close GitHub/connector gaps, and make one-accept execution usable as the default rail for future phases.

## Success metric

```text
initial mission approval: 1
routine midpoint approvals: 0
fresh main proof after merge: yes
mission result: project done or true boundary
```

S12 is not another broad architecture phase. It is the gauntlet that proves the lynchpin.

## Current caveat to carry forward

S11 PR #21 merged green before merge, but the direct post-merge truth commit on main:

```text
8988e32fc61e2824dcc19eef30da2894112ea9f9
```

has no fresh workflow runs or combined statuses visible through the connector. Under S11 policy, that commit is present but not proven until fresh main-head proof exists.

S12 must solve this with a real proof/control path.

## Core thesis

```text
GitHub Actions = execution body
GitHub commits/artifacts = durable authority ledger
MCP/resource mirrors = safe interface over receipts
MissionLease = authority boundary
Connector leverage layer = high-frequency one-call surfaces for proof, mission, and repo operations
```

The novel synthesis is receipt-first autonomy: expose mission receipts/resources before exposing more raw tools.

## Required implementation shape

One planning document only:

```text
docs/S12_ONE_ACCEPT_MISSION_GAUNTLET.md
```

Likely implementation surface:

```text
crates/secloud-mission-gauntlet/
crates/secloud-connector-leverage/
.github/workflows/stealtheye-command-dispatch.yml
.github/workflows/proof-mission-gauntlet.yml
scripts/s12-mission-gauntlet-proof.mjs
scripts/check-s12-mission-gauntlet-artifacts.mjs
.stealtheye/mission-gauntlet/
.stealtheye/command-outbox/
docs/S12_FINAL_REPORT.md
```

No prompt doc. No subsystem document forest. No broad rewrite.

## S12 implementation families

S12 must implement or prove these upgrade families:

```text
MissionGauntletSuiteV0
GauntletMissionV0
GauntletRunPlanV0
GauntletResultV0
ApprovalCountReportV0
NoRoutineMidpointApprovalProofV0
OperatorInterruptionLedgerV0
ApprovalBudgetSloV0
GitHubCapabilityClosureV0
GitHubTokenCapabilityMatrixV0
ActionsPermissionResultV0
ActionsPrCreationResultV0
WorkflowDispatchResultV0
RepositoryDispatchResultV0
MergePermissionResultV0
GitHubAppFallbackDecisionV0
StealthEyeConnectorLeverageLayerV0
ConnectorCapabilityManifestV0
ConnectorProofSurfaceV0
ConnectorMissionSurfaceV0
ConnectorRepoSurfaceV0
ConnectorBoundaryClassifierV0
StealthEyeCommandInboxV0
IssueCommentCommandDispatchV0
CommandParserV0
CommandAuthorizationCheckV0
CommandResultOutboxV0
CommandResultV0
CommandHistoryV0
CommandProofReceiptV0
WorkflowDispatchBusV0
RepositoryDispatchBusV0
ProofWorkflowDispatchPlanV0
DispatchRunCorrelationV0
DispatchResultReceiptV0
RequiredCheckResolverV0
CheckRunGraphV0
WorkflowRunGraphV0
CurrentHeadShaProofV0
SkippedWorkflowFindingV0
PendingRequiredCheckBlockV0
PathFilterPendingGuardV0
WorkflowPathCoverageMapV0
ExpectedWorkflowTriggerV0
SkippedRequiredWorkflowRiskV0
CurrentMainHeadProofV0
PostMergeProofFreshnessGateV0
MainHeadWorkflowDispatchV0
DirectMainMutationRiskV0
NoUnverifiedTruthCommitPolicyV0
MissionControlPlaneV0
MissionRunLedgerV0
MissionRunResourceIndexV0
MissionActionGraphV0
MissionProofGraphV0
MissionContinuationCursorV0
GitHubAuthorityLedgerV0
McpMissionInterfaceV0
McpResourceMirrorV0
McpToolFacadeV0
MissionLeaseToMcpRootMapV0
MissionLeaseToGitHubPermissionMapV0
MissionResourceMirrorV0
MissionResourceUriV0
ProofResourceV0
FailureClusterResourceV0
PatchResourceV0
MergeReceiptResourceV0
BoundaryStopResourceV0
ToolDescriptorSnapshotV0
ToolDescriptorCommitPinV0
ToolDescriptorHashV0
McpToolManifestV0
ToolRugPullBlockV0
ToolPoisoningBlockV0
ToolShadowingBlockV0
ToolOutputTaintPolicyV0
ExternalObservationV0
UntrustedToolOutputV0
TrustedRepoInstructionV0
InstructionSourceClassifierV0
AgenticWorkflowInjectionGuardV0
UntrustedGitHubEventTaintV0
PromptToAgentTaintCheckV0
PromptToScriptTaintCheckV0
AgenticSinkClassifierV0
TaintedOutputBlockV0
PromptToScriptFirewallV0
ModelOutputShellSinkGuardV0
GeneratedCommandAllowlistV0
NoUntrustedInterpolationPolicyV0
CommandPlanSchemaOnlyV0
NoTokenPassthroughPolicyV0
ResourceBoundTokenPolicyV0
AudienceValidationReceiptV0
NoOmnibusScopePolicyV0
ScopeMinimizationCheckV0
EgressGuardV0
PrivateIpBlockV0
RedirectValidationV0
SensitiveElicitationBlockV0
NonSensitiveQuestionOnlyV0
SingleBoundaryQuestionV0
WorkflowPermissionCompilerV0
LeastPrivilegeJobPermissionV0
PermissionDiffV0
PermissionEscalationFindingV0
GitHubAppFallbackDecisionV0
InstallationTokenNeedFindingV0
AppPermissionManifestV0
RepoScopedInstallationPolicyV0
TokenExpirationHandlingV0
MissionPacketRefV0
MissionPacketStorageV0
MissionPacketDigestV0
MissionPacketSizeGuardV0
MissionInputLimitGuardV0
MissionConcurrencyGroupV0
MissionLockV0
CancelOrQueuePolicyV0
ConcurrentMutationBlockV0
MissionSupersessionReceiptV0
MergeQueueCompatibilityV0
RulesetCompatibilityV0
BranchProtectionCompatibilityV0
RequiredReviewBoundaryV0
MergeMethodCapabilityV0
MissionReplayHarnessV0
MissionReplayLogV0
ReplayDeterminismCheckV0
ReplayAgainstReceiptsV0
SyntheticFailureMissionV0
ExpectedFailureFixtureV0
FailureInjectionPolicyV0
RepairLoopExerciseV0
BoundaryStopExerciseV0
MissionResumeTortureTestV0
InterruptedRunRecoveryV0
DuplicateBranchBlockV0
DuplicatePrBlockV0
DuplicateCommitBlockV0
StepCheckpointVerifierV0
KnownFailureRecipeCompilerV0
FailurePatternToRepairPlanV0
RecipeConfidenceGateV0
RecipeScopeGuardV0
PrQualityGateV0
PrDescriptionCompletenessV0
ClaimToDiffMapV0
ProofSummaryPresenceV0
RiskBandV0
CheckRunCorrelationEngineV0
WorkflowRunToCheckSuiteMapV0
RunAttemptTrackerV0
RerunCorrelationReceiptV0
MissionSloV0
ApprovalSloV0
ProofFreshnessSloV0
RepairLoopSloV0
MergeCompletionSloV0
TimeToGreenMetricV0
BoundaryQualityGateV0
BoundaryReasonSpecificityV0
SingleHumanActionV0
ResumePlanCompletenessV0
BoundaryNoiseFindingV0
MissionCockpitCardV0
MissionStatusLineV0
ApprovalCountLineV0
ProofFreshnessLineV0
BoundaryLineV0
NextActionLineV0
```

## Connector leverage layer

S12 must convert frequent low-level connector actions into high-level StealthEye surfaces.

### Proof surface

```text
dispatch_workflow
dispatch_workflow_and_wait
prove_main_head
run_required_proof_set
list_workflows
list_workflow_runs_filtered
correlate_workflow_dispatch
rerun_and_wait
detect_stale_green
required_check_resolver
```

### Mission surface

```text
start_mission_executor
get_mission_status
get_mission_result
resume_mission
get_approval_count_report
fetch_mission_journal
fetch_mission_result
```

### Repo surface

```text
fetch_context_pack
fetch_changed_surface
apply_file_batch
create_or_reuse_branch
create_or_reuse_pr
update_handoff_bundle
get_pr_check_summary
download_failed_logs_bundle
triage_failed_workflows
```

### Safety surface

```text
get_connector_capabilities
get_repo_actions_permissions
get_branch_rules_and_merge_blockers
classify_github_boundary
permission_upgrade_request
```

## Immediate command-dispatch bridge

Because the current ChatGPT GitHub connector can create issue comments but does not expose workflow dispatch, S12 should add an in-repo command bridge:

```text
.github/workflows/stealtheye-command-dispatch.yml
```

Triggered by issue comments such as:

```text
/stealtheye prove-main sha=<sha>
/stealtheye run-proof proof-fast main
/stealtheye run-proof-set post-merge-main
/stealtheye start-mission <mission_id>
/stealtheye status
```

The command bridge must validate authorization, parse a closed command set, reject unsafe input, dispatch proof or mission workflows, and write structured results to:

```text
.stealtheye/command-outbox/latest.json
.stealtheye/command-outbox/history.jsonl
```

This is the practical no-new-connector fallback for missing workflow-dispatch tool support.

## Mission gauntlet requirements

S12 must run real bounded missions, not only synthetic schema checks.

Required mission classes:

1. docs/state update mission
2. schema/validator registration mission
3. proof-script repair mission
4. workflow/path-filter update mission
5. failed-CI repair mission
6. post-merge proof freshness mission
7. branch/PR reuse mission
8. rerun/resume mission

Each mission must emit:

```text
ApprovalCountReportV0
GitHubCapabilityClosureV0
MissionExecutorResultV0
CurrentMainHeadProofV0 when applicable
RequiredCheckResolverV0
BoundaryStopV0 if blocked
MissionCockpitCardV0
```

## GitHub/MCP synthesis

S12 should not expose random tool chaos.

Rule:

```text
GitHub stores truth.
MCP exposes controlled mission resources.
MissionLease binds both.
Tool output can inform but cannot command.
```

Resource examples:

```text
stealtheye://mission/<id>/state
stealtheye://mission/<id>/journal
stealtheye://mission/<id>/proof
stealtheye://mission/<id>/boundary
stealtheye://mission/<id>/final-report
```

## Security and injection guardrails

S12 must treat as untrusted data:

```text
issue bodies
PR bodies
comments
workflow logs
external tool output
MCP output
browser observations
```

S12 must prevent:

```text
untrusted GitHub text becoming instructions
model output flowing directly into shell
raw token passthrough
omnibus permission scopes
unbounded MCP tool authority
SSRF through metadata/redirects
secret elicitation
browser-cookie/session-token automation
GitHub permission bypass
```

## Required validators

```text
secloud validate mission-gauntlet
secloud validate approval-count-report
secloud validate github-capability-closure
secloud validate connector-capability-manifest
secloud validate command-dispatch
secloud validate command-outbox
secloud validate proof-surface
secloud validate mission-surface
secloud validate repo-surface
secloud validate current-main-head-proof
secloud validate required-check-resolver
secloud validate path-filter-pending-guard
secloud validate mission-control-plane
secloud validate mission-resource-mirror
secloud validate descriptor-pinning
secloud validate tool-output-taint
secloud validate workflow-injection-guard
secloud validate prompt-to-script-firewall
secloud validate mission-replay
secloud validate synthetic-failure-injection
secloud validate mission-resume
secloud validate boundary-quality
```

All S12 validators must be included in `secloud doctor`.

## Required proof artifacts

```text
.stealtheye/mission-gauntlet/gauntlet-run-plan.json
.stealtheye/mission-gauntlet/gauntlet-result.json
.stealtheye/mission-gauntlet/approval-count-report.json
.stealtheye/mission-gauntlet/github-capability-closure.json
.stealtheye/mission-gauntlet/connector-capability-manifest.json
.stealtheye/mission-gauntlet/current-main-head-proof.json
.stealtheye/mission-gauntlet/required-check-resolver-report.json
.stealtheye/mission-gauntlet/path-filter-pending-guard.json
.stealtheye/mission-gauntlet/mission-control-plane.json
.stealtheye/mission-gauntlet/mission-resource-mirror.json
.stealtheye/mission-gauntlet/workflow-injection-guard-report.json
.stealtheye/mission-gauntlet/prompt-to-script-firewall-report.json
.stealtheye/mission-gauntlet/mission-replay-report.json
.stealtheye/mission-gauntlet/synthetic-failure-report.json
.stealtheye/mission-gauntlet/resume-torture-report.json
.stealtheye/mission-gauntlet/boundary-quality-report.json
.stealtheye/mission-gauntlet/s12-proof.json
.stealtheye/command-outbox/latest.json
.stealtheye/command-outbox/history.jsonl
```

## Acceptance criteria

S12 is complete only if:

1. At least 5 real bounded missions run through S11 or the command-dispatch bridge.
2. Every mission records `ApprovalCountReportV0`.
3. Routine midpoint approvals are zero for successful routine missions.
4. At least one mission includes failed proof and successful repair loop.
5. At least one mission proves branch/PR reuse.
6. At least one mission proves post-merge current-main proof.
7. Required checks are resolved from actual workflow/check data.
8. Path-filter pending-check risk is tested.
9. Connector capability manifest records what the connector can and cannot do.
10. Command-dispatch bridge can trigger proof from an issue/comment if native dispatch is unavailable.
11. GitHub App need is decided by evidence, not guessing.
12. Mission resource mirror exposes receipt-first mission state.
13. Agentic workflow injection guard blocks untrusted GitHub/MCP/tool output from becoming instruction.
14. Prompt-to-script firewall prevents arbitrary model shell execution.
15. Mission resume test proves no duplicate branch, PR, commit, or merge.
16. Boundary stops are concise and single-action.
17. Current main HEAD proof is fresh after merge/truth update.
18. S0–S11 proof gates are not weakened.
19. No hidden autonomy, browser-cookie/session automation, secrets, paid compute, production deployment, database mutation, account permission changes, destructive irreversible action, validator weakening, schema weakening, proof weakening, or GitHub permission bypass is introduced.

## Explicit non-goals

Do not build:

```text
UI dashboard
random MCP expansion
cloud deployment
broad architecture rewrite
doc forest
prompt doc
more prompt-only rules
unsafe one-approval-for-everything
GitHub permission bypass
browser-cookie/session automation
```

## One-line goal

```text
S12 proves and hardens S11 by running real bounded missions through the one-accept executor, adding a connector leverage layer for high-frequency proof/mission/repo actions, closing GitHub token/workflow/merge gaps, mirroring mission receipts safely through MCP-style resources, and proving one initial approval with zero routine midpoint approvals.
```
