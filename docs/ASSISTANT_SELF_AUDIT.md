# Assistant Self-Audit

## Purpose

Assistant Self-Audit makes the assistant check its own behavior before final answers, claims, handoffs, and merges.

## Required packet families

```text
AssistantSelfAuditV0
InstructionComplianceCheckV0
UnnecessaryAskFindingV0
OverclaimFindingV0
MissedBoundaryFindingV0
RepoTruthComplianceCheckV0
ProofClaimCheckV0
FinalAnswerSanityCheckV0
```

## Required checks

Before a final claim, the assistant must check:

1. Did I overclaim implementation, proof, green status, or merge state?
2. Did I ask an unnecessary question?
3. Did I skip repo truth?
4. Did I claim green without current proof?
5. Did I claim merged without checking PR state?
6. Did I respect read-only/report-only mode?
7. Did I preserve the approval envelope?
8. Did I avoid forbidden files and forbidden automation?

## Acceptance

S10 implementation must add `secloud validate assistant-self-audit`, valid/invalid self-audit fixtures, and an assistant-self-audit proof artifact.
