# Context Load Policy

## Purpose

The Context Load Policy makes every StealthEye Cloud tab read the right files first and avoid reconstructing repo truth from stale chat memory.

## Required packet families

```text
ContextLoadPlanV0
RequiredContextFileSetV0
ContextPriorityMapV0
ContextFreshnessCheckV0
ContextSaturationRiskV0
ContextSkipJustificationV0
ContextLoadReceiptV0
ContextConflictScanV0
ContextMinimumViableReadSetV0
ContextOverreadFindingV0
```

## Default required read set

For repo-based work, the assistant must read or explicitly justify skipping:

```text
AGENTS.md
STEALTHEYE_DECISIONS.md
STEALTHEYE_ACTIVE.md
STEALTHEYE_RELAY.md
STEALTHEYE_RELAY.json
STEALTHEYE_SEAL.json
NEXT_ACTION.md
README.md
AGENT_REPO_MAP.md
docs/StealthEye_Cloud_Build_Plan.md
docs/HANDOFF_AND_CONTINUATION.md
current phase spec
relevant crate files
relevant CLI validator files
relevant schema files
relevant workflow files
```

## Freshness rule

Current `main`, current branch, PR state, and CI status outrank old Relay text and old chat memory.

## Acceptance

S10 implementation must add `secloud validate context-load-policy`, valid and invalid fixtures, and a context-load proof artifact that names required files and skipped-file justifications.
