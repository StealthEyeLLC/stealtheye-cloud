# S11 — One-Accept Mission Executor

## Purpose

S11 exists to solve one problem: approval spam.

The target operator experience is:

```text
initial mission approval: 1
routine midpoint approvals: 0
human stops: true boundaries only
```

S11 must move routine repo/build/proof/repair/merge work out of repeated ChatGPT tool calls and into one approved GitHub-native mission executor.

## Core thesis

```text
One approved MissionLeaseV0
→ one GitHub-native mission executor run
→ branch/reuse
→ batch repo mutation
→ PR/reuse
→ proof inspection
→ exact repair loop
→ merge when green if allowed
→ final report
```

S11 is not a prompt rule, not a docs phase, not random MCP chaining, and not a hidden approval bypass. It is a real executor path.

## Non-negotiable success test

S11 is successful only if a routine repo mission can run from one approved trigger without repeated assistant-side approvals for file writes, PR creation, CI inspection, repair, rerun, or merge.

If S11 cannot prove that, S11 has failed its purpose.

## Required implementation shape

One planning document only:

```text
docs/S11_ONE_ACCEPT_MISSION_EXECUTOR.md
```

One crate:

```text
crates/secloud-mission-executor
```

One executor workflow:

```text
.github/workflows/mission-executor.yml
```

One proof workflow:

```text
.github/workflows/proof-mission-executor.yml
```

One proof script:

```text
scripts/s11-mission-executor-proof.mjs
```

One artifact directory:

```text
.stealtheye/mission-executor/
```

One final report:

```text
docs/S11_FINAL_REPORT.md
```

No prompt doc. No setup PR. No subsystem document forest.

## Required S11 contracts

S11 must implement the following real upgrade families:

```text
GitHubCapabilityPreflightV0
ActionsWritePermissionCheckV0
ActionsPrCreationCheckV0
WorkflowDispatchCapabilityV0
RepositoryDispatchCapabilityV0
WorkflowTokenLimitFindingV0
GitHubAppUpgradeDecisionV0
BranchRulesetCompatibilityCheckV0
MergePermissionCheckV0
RequiredChecksResolverV0
BoundaryStopForGitHubPermissionV0
MissionLeaseV0
MissionAuthorityEnvelopeV0
OneAcceptMissionV0
ApprovedRoutineActionSetV0
ForbiddenActionSetV0
MissionStopConditionSetV0
MissionBudgetV0
MissionScopeHashV0
MissionPolicySimulatorV0
MissionExecutorRequestV0
MissionExecutorWorkflowV0
MissionExecutorStateV0
MissionExecutorReportV0
BatchRepoMutationV0
FileSetPatchV0
ExpectedShaGuardV0
AtomicCommitPlanV0
RollbackSnapshotV0
MutationReceiptV0
BranchControllerV0
ExistingBranchReuseV0
PrControllerV0
ExistingPrReuseV0
ProofControllerV0
WorkflowRunInspectorV0
CombinedStatusFallbackV0
CurrentHeadShaProofV0
ProofFreshnessCheckV0
ProofRepairLoopV0
FailureClusterV0
RepairBatchV0
RetryBudgetV0
KnownFailureAutopatchV0
MergeWhenGreenControllerV0
MergePolicyV0
RequiredChecksGreenV0
PostMergeVerificationV0
PostMergeProofFreshnessGateV0
DirectMainMutationRiskV0
PostMergeTruthCommitPolicyV0
WorkflowDispatchAfterDirectMutationV0
CurrentMainHeadProofV0
NoUnverifiedTruthCommitPolicyV0
BoundaryStopV0
ResumeAfterBoundaryPlanV0
MissionJournalV0
ActionReceiptV0
ProofReceiptV0
RepairReceiptV0
MergeReceiptV0
IdempotencyKeyV0
ActionDedupReceiptV0
MissionStatePersistenceV0
ToolDescriptorSnapshotV0
ToolResultTaintV0
McpAuthorizationGuardV0
ApprovalCountReportV0
NoRoutineMidpointApprovalProofV0
OneAcceptDemoMissionV0
MissionExecutorProofV0
```

## Mission lease rules

A mission lease may pre-approve these routine actions:

```text
read repo
create or reuse branch
batch create/update/delete files
commit and push
open or reuse PR
inspect CI and logs
classify proof failures
patch exact failures
rerun proof
update state/handoff/final report
merge when green if lease allows and GitHub permits it
```

A mission lease must never pre-approve:

```text
secrets
paid compute
production deployment
database mutation
account permission changes
private data exposure
browser-cookie/session-token automation
destructive irreversible action
scope expansion
unapproved external posting
legal/compliance signoff
```

## GitHub capability preflight

Before execution, S11 must determine whether GitHub can actually support one-accept completion:

```text
Can Actions write contents?
Can Actions create PRs?
Can Actions dispatch workflows?
Can Actions inspect workflow runs/logs?
Can Actions rerun failed jobs?
Can Actions merge PRs?
Will GITHUB_TOKEN pushes trigger required proof lanes?
Is repository_dispatch needed?
Is a GitHub App installation token needed?
Do branch rules allow merge-when-green?
```

Default path:

```text
GITHUB_TOKEN + workflow_dispatch + explicit workflow permissions
```

Upgrade path:

```text
GitHub App installation token only if preflight proves GITHUB_TOKEN blocks the mission path
```

## Post-merge proof freshness requirement

S11 must solve the exact post-S10 caveat:

```text
S10 PR #19 was green before merge.
A direct post-merge truth commit was made afterward.
That direct post-merge truth commit did not spawn a fresh Actions run through the connector.
Therefore direct post-merge truth updates are not separately CI-proven unless a fresh proof run verifies the resulting main HEAD.
```

S11 rule:

```text
No direct post-merge truth commit counts as proven unless a fresh workflow_dispatch proof run verifies the resulting main HEAD.
```

Preferred policy:

```text
Include truth/state updates before merge whenever possible.
If a direct post-merge truth update is unavoidable, trigger fresh proof on main and record CurrentMainHeadProofV0.
```

## Proof/repair loop

The executor must:

```text
run required proof
inspect every failed workflow/log
cluster failures
patch exact causes only
commit repair batch
rerun proof
repeat within retry budget
stop only on boundary or exhausted retry budget
```

Known autopatch categories:

```text
missing Cargo workspace entry
missing CLI validator registration
missing schema registration
missing package.json script
workflow path filter miss
README/build-plan stale status
Relay/Seal/Active mismatch
doctor missing validator
proof script stale phase expectation
post-merge truth commit without fresh proof
```

## Merge-when-green

If `merge_allowed = true` and required checks are green on the current PR head, the executor should merge without asking again.

If GitHub permissions, branch protection, rulesets, or required reviews block merge, the executor must emit `BoundaryStopV0` with one clear required human action.

## Mission state and idempotency

The executor must persist:

```text
.stealtheye/mission-executor/state.json
.stealtheye/mission-executor/journal.jsonl
.stealtheye/mission-executor/result.json
.stealtheye/mission-executor/boundary-stop.json
```

Reruns must not create duplicate branches, duplicate PRs, duplicate commits, or duplicate proof loops.

## Required validators

```text
secloud validate mission-lease
secloud validate mission-executor-request
secloud validate github-capability-preflight
secloud validate batch-repo-mutation
secloud validate branch-controller
secloud validate pr-controller
secloud validate proof-controller
secloud validate proof-repair-loop
secloud validate merge-when-green
secloud validate post-merge-proof-freshness
secloud validate boundary-stop
secloud validate mission-journal
secloud validate mission-state
secloud validate idempotency
secloud validate approval-count-proof
secloud validate mission-executor
```

All S11 validators must be included in `secloud doctor`.

## Required proof artifacts

```text
.stealtheye/mission-executor/github-capability-preflight.json
.stealtheye/mission-executor/mission-lease.json
.stealtheye/mission-executor/mission-executor-request.json
.stealtheye/mission-executor/mission-plan.json
.stealtheye/mission-executor/batch-repo-mutation.json
.stealtheye/mission-executor/pr-controller-report.json
.stealtheye/mission-executor/proof-controller-report.json
.stealtheye/mission-executor/proof-repair-loop-report.json
.stealtheye/mission-executor/merge-when-green-report.json
.stealtheye/mission-executor/post-merge-proof-freshness-report.json
.stealtheye/mission-executor/boundary-stop-report.json
.stealtheye/mission-executor/mission-journal.jsonl
.stealtheye/mission-executor/approval-count-report.json
.stealtheye/mission-executor/mission-executor-proof.json
```

## Acceptance criteria

S11 is complete only if:

1. GitHub capability preflight validates.
2. MissionLeaseV0 validates.
3. MissionExecutorRequestV0 validates.
4. `mission-executor.yml` exists and runs by `workflow_dispatch`.
5. Executor can create or reuse a branch.
6. Executor can apply batch repo mutations.
7. Executor can commit and push.
8. Executor can open or reuse a PR.
9. Executor can inspect workflow runs and logs.
10. Executor can classify failed proof.
11. Executor can apply exact repair batches.
12. Executor can rerun proof.
13. Executor can merge when green if lease and GitHub permissions allow.
14. Executor emits BoundaryStopV0 for real boundaries.
15. Executor records MissionJournalV0.
16. Reruns are idempotent.
17. ApprovalCountReportV0 proves routine midpoint approvals are zero in the demo path.
18. PostMergeProofFreshnessGateV0 prevents unverified direct truth commits from being treated as proven.
19. S0–S10 proof gates are not weakened.
20. No hidden autonomy, browser-cookie/session automation, secrets, paid compute, production deployment, database mutation, account permission changes, destructive irreversible actions, validator weakening, schema weakening, or proof weakening are introduced.

## Explicit non-goals

Do not build:

```text
document forest
setup PR
prompt doc
random MCP chain
schemas without executor
assistant behavior rules without runtime
fake no-approval claim
unbounded autonomy
browser session/cookie automation
GitHub permission bypass
```

## One-line goal

```text
S11 builds a real GitHub-native mission executor so one approved mission lease can complete routine repo/build/proof/repair/merge work without repeated operator confirmations.
```
