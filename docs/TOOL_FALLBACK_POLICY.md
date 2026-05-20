# Tool Fallback Policy

## Purpose

The Tool Fallback Policy lets the assistant recover from routine tool failures without interrupting the operator.

## Required packet families

```text
ToolFallbackPolicyV0
ToolFailureClassificationV0
FallbackAttemptReportV0
RetrySafetyDecisionV0
ExistingBranchReuseDecisionV0
ExistingPrReuseDecisionV0
ShaMismatchRecoveryV0
WorkflowStatusFallbackV0
GitHubOperationRepairV0
```

## Recoverable examples

```text
SHA mismatch       → refetch and retry
branch exists      → reuse branch
PR exists          → continue PR
status API empty   → inspect workflow runs
merge payload bad  → correct payload and retry
path blocked       → use neutral public-safe path/name if safe
```

## Stop conditions

Stop instead of retrying when the failure implies secrets, paid compute, private data exposure, account permission changes, destructive irreversible action, or unresolved high-impact ambiguity.

## Acceptance

S10 implementation must add `secloud validate tool-fallback-policy`, recovery fixtures, blocked retry fixtures, and a tool-fallback proof artifact.
