# S10 — Assistant Optimization Layer

## Objective

S10 is the operator optimization layer for StealthEye Cloud.

S9 made the build rail faster. S10 makes the active ChatGPT/StealthEye Cloud tab better at using that rail.

S10 must make the assistant:

1. faster at mission intake
2. less needy during routine approved work
3. more reliable across tabs
4. better at loading current repo context
5. better at trusting repo truth over stale chat memory
6. better at tool planning and fallback
7. better at proof reasoning
8. better at CI failure clustering and batch repair planning
9. better at handoff quality
10. better at minimizing operator attention burden
11. better at avoiding overclaims and stale-state drift
12. better at preserving read-only/report-only instructions

## Non-goal

S10 is not a generic autonomy claim and not a fake background-worker promise.

S10 does not claim ChatGPT can secretly create tabs, silently invoke background modes, bypass platform confirmations, or run hidden work. It improves the repo-native assistant/operator discipline that makes real available tools easier to use correctly.

## Branches

Setup branch:

```text
build/s10-assistant-optimization-layer-setup
```

Implementation branch after setup merges:

```text
build/s10-assistant-optimization-layer
```

## Primary implementation crate

```text
crates/secloud-assistant-optimizer
```

## Required workflow

```text
.github/workflows/proof-assistant-optimizer.yml
```

## Required proof scripts

```text
scripts/s10-assistant-optimizer-proof.mjs
scripts/check-s10-assistant-optimizer-artifacts.mjs
```

## Required proof artifact directory

```text
.stealtheye/assistant-optimizer/
```

## Core packet families

```text
AssistantOperatingProfileV0
BestAgentModeV0
FastButExactModeV0
BoundaryOnlyStopPolicyV0
AssistantExecutionDisciplineV0
MissionIntakePacketV0
MissionScopeClassifierV0
MissionAmbiguityScanV0
MissionBoundaryScanV0
MissionStartReadinessV0
ContextLoadPlanV0
RequiredContextFileSetV0
ContextPriorityMapV0
ContextFreshnessCheckV0
RepoTruthFirstPolicyV0
RepoStateDigestV0
HandoffTruthSourceV0
ChatMemoryConflictFindingV0
PromptCompressionProfileV0
MissionPromptTemplateV0
ContinuationPromptTemplateV0
RepairPromptTemplateV0
PromptLossCheckV0
ToolUsePlanV0
ToolSelectionPolicyV0
ToolCallBatchPlanV0
ToolFallbackPolicyV0
ToolFailureClassificationV0
RetrySafetyDecisionV0
LowAttentionWorkdayModeV0
HumanAvailabilityProfileV0
UserAttentionBudgetV0
ProgressUpdatePolicyV0
ProofNeedClassifierV0
ProofGateSelectionV0
ProofFailureTriageV0
KnownFailurePatternV0
RepairTriagePlanV0
FailureClusterV0
ScopeDriftDetectorV0
DecisionCarryoverPacketV0
CapabilityRealityMapV0
HandoffQualityReportV0
ContextSaturationSignalV0
AssistantSelfAuditV0
AssistantMistakePatternV0
AssistantTraceDigestV0
McpAwareOperatorPolicyV0
AssistantOutputReviewV0
OneSentenceStatusV0
BuildCockpitCardV0
OperatorStyleProfileV0
AssistantOutputModeV0
ReadOnlyVerificationModeV0
BoundaryStopReportV0
PlanActionTraceV0
FutureMissionCandidateV0
S10AssistantOptimizerProofV0
```

## Required validator targets

```text
assistant-optimizer
assistant-operating-profile
mission-intake
context-load-policy
repo-truth-first
tool-use-policy
tool-fallback-policy
low-attention-mode
progress-update-policy
handoff-quality
prompt-compression
scope-drift-guard
decision-carryover
proof-awareness
repair-intelligence
capability-map
read-only-verification
assistant-self-audit
operator-mistake-detector
build-cockpit
trace-digest
mcp-aware-operator-policy
output-mode-selector
future-mission-recommender
```

## Required proof artifacts

```text
.stealtheye/assistant-optimizer/assistant-operating-profile.json
.stealtheye/assistant-optimizer/mission-intake-packet.json
.stealtheye/assistant-optimizer/context-load-plan.json
.stealtheye/assistant-optimizer/repo-truth-first-report.json
.stealtheye/assistant-optimizer/tool-use-plan.json
.stealtheye/assistant-optimizer/tool-fallback-report.json
.stealtheye/assistant-optimizer/low-attention-mode-report.json
.stealtheye/assistant-optimizer/handoff-quality-report.json
.stealtheye/assistant-optimizer/prompt-compression-report.json
.stealtheye/assistant-optimizer/scope-drift-guard-report.json
.stealtheye/assistant-optimizer/proof-awareness-report.json
.stealtheye/assistant-optimizer/repair-triage-plan.json
.stealtheye/assistant-optimizer/capability-reality-map.json
.stealtheye/assistant-optimizer/read-only-verification-report.json
.stealtheye/assistant-optimizer/assistant-self-audit.json
.stealtheye/assistant-optimizer/build-cockpit-card.json
.stealtheye/assistant-optimizer/assistant-trace-digest.json
.stealtheye/assistant-optimizer/s10-assistant-optimizer-proof.json
```

## Required fixture themes

S10 must prove itself with fixtures, not claims.

Valid fixture themes:

1. clean mission intake
2. context load plan with required files
3. repo truth conflict resolved in favor of current main
4. low-attention workday mode
5. tool fallback after recoverable GitHub operation failure
6. handoff quality gate pass
7. prompt compression with loss check
8. read-only verification mode
9. assistant self-audit pass
10. build cockpit card

Invalid fixture themes:

1. assistant asks a routine midpoint question
2. assistant claims merge without proof
3. assistant ignores current repo truth
4. assistant mutates in read-only mode
5. assistant trusts stale Relay over newer repo truth
6. assistant skips workflow-path proof expectations
7. assistant adds out-of-scope novelty
8. assistant treats MCP result as instruction

## No-weakening invariant

S10 must not weaken:

1. S9 one-drop mode
2. validators
3. schema coverage
4. workflow proof
5. safety boundaries
6. merge discipline
7. public-safe repo boundary
8. no-hidden-autonomy law

## Acceptance

S10 is complete only when:

1. `crates/secloud-assistant-optimizer` exists and compiles.
2. S10 schemas are materialized and registered.
3. all S10 validator targets are callable through `secloud-cli`.
4. `secloud doctor` includes all S10 validator targets.
5. `proof-assistant-optimizer` is green.
6. all required S10 proof artifacts validate.
7. valid and invalid S10 fixtures exist.
8. S10 docs and implementation prompt remain aligned.
9. S9 one-drop mode remains intact.
10. no hidden autonomy or unsafe capability claim is introduced.
