# S9 — One-Drop Build Accelerator

## Objective

S9 exists to make every future StealthEye Cloud phase or project operate as close as possible to:

```text
one mission approval
→ one coherent repo mutation/drop
→ one PR
→ one proof wave
→ batched repairs
→ merge when green
```

S9 is not a product feature lane. It is a build-operating-system upgrade for speed, lower confirmation friction, cleaner handoffs, and fewer cleanup PRs.

## Branch

```text
build/s9-one-drop-build-accelerator
```

## Primary crate

```text
crates/secloud-build-accelerator
```

## Core rule

Routine work covered by a mission approval must continue without midpoint confirmation requests.

Stop only for true boundaries:

1. secrets
2. paid compute
3. account permission changes
4. private data exposure risk
5. production deployment without explicit approval
6. database mutation without explicit approval
7. browser-cookie/session-token automation
8. unresolved high-impact ambiguity

## Required systems

### 1. One-drop manifest

Defines the complete planned drop before implementation begins.

Required packets:

```text
OneDropPlanV0
OneDropManifestV0
OneDropCompilerV0
DropCompletenessCheckV0
DropApplicationReportV0
DropPlanHashV0
DropManifestHashV0
DropContentHashV0
DropProofHashV0
```

### 2. Mission approval envelope

Turns one user approval into explicit safe authority for routine phase work.

Required packets:

```text
MissionApprovalEnvelopeV0
ApprovalCompressionPolicyV0
ApprovalReuseDecisionV0
RoutineActionClassV0
BoundaryActionClassV0
UnnecessaryConfirmationFindingV0
NoMidpointAskPolicyV0
RoutineContinuationDecisionV0
BoundaryStopDecisionV0
```

### 3. Repo mutation batch model

Represents one coherent repo update instead of many file-by-file improvisations.

Required packets:

```text
RepoMutationBatchV0
FileChangeSetV0
BranchUpdatePlanV0
CommitBatchPlanV0
GitTreeMutationPlanV0
GitTreeEntryV0
GitBlobBatchV0
GitCommitAssemblyV0
GitRefUpdatePlanV0
GitBatchMutationReceiptV0
ToolCallBundlePolicyV0
ToolCallBatchPlanV0
ToolCallOverheadReportV0
```

### 4. Batch CI repair

Forbids one-failure-at-a-time repair loops unless only one failure exists.

Required packets:

```text
CiFailureBatchV0
CiFailureDigestV0
FailedWorkflowSetV0
FailedStepDigestV0
BatchRepairPlanV0
RepairCommitPlanV0
RepairWaveReportV0
RepairCauseMemoryV0
RecurringFailurePatternV0
RepairPreventionRuleV0
```

### 5. Merge-aware handoff

Prevents post-merge cleanup PRs caused by stale pending language.

Required packets:

```text
MergeAwareNextActionV0
PostMergeTruthTemplateV0
PreMergeTruthTemplateV0
PostMergeResolutionFieldV0
MergeShaPlaceholderV0
TruthAfterMergeRuleV0
HandoffStateTransitionV0
MergeReadinessEnvelopeV0
PostMergeCleanupAvoidanceV0
PostMergeDriftPredictionV0
CleanupPrRiskV0
PostMergeStateFitnessV0
MergeShaResolutionV0
MergeMetadataLookupV0
PostMergeShaBindingV0
```

### 6. State consistency oracle

Checks README, build plan, Active, Relay, Seal, Next Action, final report, and PR body for conflicts.

Required packets:

```text
StateConsistencyReportV0
ActiveRelaySealDiffV0
NextActionConsistencyV0
ReadmeBuildPlanConsistencyV0
PhaseStatusConflictV0
ObsoletePhraseScanV0
StalePhaseReferenceFindingV0
PostMergeStaleTextFindingV0
DuplicateDocFindingV0
DocConflictFindingV0
CanonicalDocMapV0
```

### 7. Phase truth and lifecycle automata

Makes phase/branch/PR status deterministic.

Required packets:

```text
StealthEyePhaseStateV0
PhasePreflightLockV0
PreflightScopeSnapshotV0
PreflightApprovalHashV0
BranchLifecycleV0
BranchStateTransitionV0
BranchCompletionStateV0
PrLifecycleV0
PrStateTransitionV0
PrMergeGateV0
```

### 8. Proof selection and CI optimization

Maps touched surfaces to required proof gates and prevents surprise skipped workflows.

Required packets:

```text
TouchedSurfaceV0
ProofSelectorV0
RequiredProofSetV0
OptionalProofSetV0
RequiredChecksManifestV0
WorkflowPathFilterSimulationV0
ExpectedWorkflowRunSetV0
UnexpectedWorkflowSkipFindingV0
CiCostProfileV0
WorkflowTriggerBudgetV0
PathFilterPlanV0
ProofLadderPlanV0
RerunSuppressionPolicyV0
ProofEscalationDecisionV0
CiWaveCounterV0
CiWaveSummaryV0
ExcessiveCiWaveFindingV0
```

### 9. Registration and downgrade guards

Prevents missing schema/crate/validator/workflow registrations and silent regressions.

Required packets:

```text
ValidatorRegistrationPlanV0
ValidatorRegistrationReportV0
MissingValidatorHookFindingV0
SchemaInventoryDiffV0
UnregisteredSchemaFindingV0
OrphanSchemaFindingV0
WorkspaceRegistrationCheckV0
CrateManifestFindingV0
MissingWorkspaceMemberFindingV0
CapabilityDowngradeFindingV0
NoSilentDowngradePolicyV0
BoundaryCarryoverSetV0
BoundaryRegressionFindingV0
ForbiddenCapabilityScanV0
```

### 10. Human attention and recovery

Optimizes for low-attention workdays, tab saturation, and partial drop recovery.

Required packets:

```text
HumanAvailabilityProfileV0
ConfirmationFrictionEventV0
FrictionCauseV0
FrictionAvoidancePlanV0
HumanAttentionBudgetV0
ToolFallbackLadderV0
ToolFailureClassificationV0
FallbackAttemptReportV0
ExistingWorkReusePolicyV0
ExistingBranchDecisionV0
ExistingPrDecisionV0
PartialDropRecoveryV0
InterruptedMissionStateV0
ResumeFromPartialDropPlanV0
SaturationHandoffPromptV0
ContinuationPromptTemplateV0
TabTransferPacketV0
NextTabPromptArtifactV0
BuildCockpitCardV0
CurrentPhaseStatusV0
NextHumanActionV0
ProofStatusSummaryV0
OneSentenceStatusV0
```

### 11. Future phase default contract

Every future phase must inherit the same default build contract.

Required packets:

```text
FuturePhaseDefaultContractV0
CapabilityActivationLedgerV0
CapabilityStatusV0
BuildArtifactRetentionPolicyV0
ProofArtifactRetentionV0
PhaseTemplateV0
PhaseChecklistV0
PhaseDocPackV0
PhaseProofPlanV0
PhaseHandoffTemplateV0
PhaseFinalReportTemplateV0
PhaseReleaseNoteV0
ReleaseNoteGenerationPolicyV0
BuildVelocityReportV0
```

## Required validators

```text
secloud validate one-drop
secloud validate mission-approval
secloud validate approval-compression
secloud validate no-midpoint-ask
secloud validate tool-call-bundling
secloud validate repo-mutation-batch
secloud validate batch-repair
secloud validate merge-aware-handoff
secloud validate no-cleanup-pr
secloud validate state-consistency
secloud validate phase-truth
secloud validate branch-lifecycle
secloud validate pr-lifecycle
secloud validate proof-selector
secloud validate required-checks
secloud validate workflow-path-filter
secloud validate ci-wave-counter
secloud validate repair-cause-memory
secloud validate validator-registration
secloud validate schema-inventory
secloud validate workspace-registration
secloud validate obsolete-text
secloud validate doc-conflicts
secloud validate merge-readiness-red-team
secloud validate build-velocity
secloud validate confirmation-friction
secloud validate human-availability
secloud validate tool-fallback
secloud validate existing-work-reuse
secloud validate partial-completion-recovery
secloud validate no-silent-downgrade
secloud validate capability-activation-ledger
secloud validate future-phase-contract
secloud validate next-tab-prompt
```

## Required workflow

```text
.github/workflows/proof-build-accelerator.yml
```

## Required docs

```text
docs/ONE_DROP_BUILD_MODE.md
docs/MISSION_APPROVAL_ENVELOPE.md
docs/BATCH_REPAIR_POLICY.md
docs/MERGE_AWARE_HANDOFF.md
docs/PHASE_TEMPLATE_SYSTEM.md
docs/PROMPTS/NEXT_TAB_PROMPT.md
```

## Acceptance

S9 is complete only when:

1. `crates/secloud-build-accelerator` exists and compiles.
2. all S9 schemas are registered in the inventory.
3. all S9 validators are callable through `secloud-cli`.
4. `secloud doctor` includes all S9 validators.
5. `proof-build-accelerator` is green.
6. state consistency catches README/build-plan/Active/Relay/Seal/Next Action conflicts.
7. no-cleanup-PR gate detects stale post-merge language.
8. build velocity and confirmation friction artifacts validate.
9. next-tab prompt artifact exists.
10. future phase default contract validates.
11. docs and handoff state point to the next phase without requiring a cleanup PR.

## Non-goals

S9 does not bypass platform/tool permission prompts. It reduces avoidable assistant/process confirmations and makes each accepted action larger.

S9 does not authorize secrets, paid compute, production deployment, database mutation, browser-cookie/session-token automation, or private data exposure.
