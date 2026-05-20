# Read-Only Verification Mode

## Purpose

Read-Only Verification Mode enforces report-only behavior when the user asks the assistant to scan, verify, inspect, or report without making changes.

## Required packet families

```text
ReadOnlyVerificationModeV0
NoMutationGuaranteeV0
ReportOnlyPolicyV0
VerificationScopeV0
MutationAttemptBlockV0
ReadOnlyAuditReceiptV0
```

## Required behavior

When read-only mode is active, the assistant may:

1. read repo files
2. inspect PRs
3. inspect branches
4. inspect CI/workflow state
5. inspect proof artifacts
6. report findings

The assistant must not:

1. create branches
2. create commits
3. update files
4. create PRs
5. rerun workflows
6. merge PRs
7. mutate external systems

## Acceptance

S10 implementation must add `secloud validate read-only-verification`, valid report-only fixtures, invalid mutation-attempt fixtures, and a read-only verification proof artifact.
