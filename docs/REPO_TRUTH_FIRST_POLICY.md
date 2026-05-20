# Repo Truth First Policy

## Purpose

Repo Truth First makes current repository state outrank stale chat memory when work is repo-based.

## Required packet families

```text
RepoTruthFirstPolicyV0
RepoStateDigestV0
HandoffTruthSourceV0
ChatMemoryConflictFindingV0
CurrentMainTruthDigestV0
MergedPrTruthDigestV0
StaleStateFindingV0
TruthPrecedenceReportV0
```

## Truth precedence

Use this order when facts conflict:

1. current user instruction
2. current repo state
3. current CI/browser result
4. latest StealthEye Seal
5. `STEALTHEYE_ACTIVE.md`
6. `STEALTHEYE_DECISIONS.md`
7. `STEALTHEYE_RELAY.md` / `.json`
8. older chat memory

## Required checks

S10 must detect stale handoff/status language such as:

1. branch marked active after merge
2. PR marked pending after merge
3. Next Action points to completed work
4. Seal points to old mission state
5. README/build plan disagree with Active/Relay
6. chat memory conflicts with current main

## Acceptance

S10 implementation must add `secloud validate repo-truth-first`, current-main truth fixtures, stale-state invalid fixtures, and a repo-truth proof artifact.
