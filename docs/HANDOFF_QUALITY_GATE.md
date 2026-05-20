# Handoff Quality Gate

## Purpose

The Handoff Quality Gate makes every tab switch reliable and prevents stale Active/Relay/Seal/Next Action state from surviving a merge.

## Required packet families

```text
HandoffQualityReportV0
ContinuationPromptQualityV0
TabResumeReadinessV0
HandoffCompletenessCheckV0
RelaySealConsistencyV0
ActiveNextActionConsistencyV0
HandoffStalenessFindingV0
HandoffRepairSuggestionV0
```

## Required checks

S10 must validate:

1. Active matches Relay
2. Relay JSON matches Relay markdown
3. Seal matches latest mission
4. Next Action is exact
5. README and build plan are not stale
6. latest PR/merge state is resolved
7. implementation prompt exists
8. forbidden files are absent

## Acceptance

S10 implementation must add `secloud validate handoff-quality`, stale-handoff invalid fixtures, and a handoff-quality proof artifact.
