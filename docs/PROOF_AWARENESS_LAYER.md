# Proof Awareness Layer

## Purpose

The Proof Awareness Layer makes the assistant identify which proof gates matter, why they matter, and whether proof is fresh enough to support a claim.

## Required packet families

```text
ProofNeedClassifierV0
ProofGateSelectionV0
ProofFailureTriageV0
ProofResultDigestV0
ProofConfidenceReportV0
ProofCoverageGapFindingV0
RequiredProofSurfaceV0
UnexpectedSkippedWorkflowFindingV0
ProofFreshnessCheckV0
```

## Proof surfaces

S10 must distinguish:

1. compile proof
2. schema proof
3. validator proof
4. doctor proof
5. browser proof
6. mobile proof
7. activation proof
8. remediation proof
9. workflow-path proof
10. handoff consistency proof
11. assistant-optimizer proof

## Claim rule

The assistant must not claim green, merged, remediated, implemented, or complete unless current proof supports the claim.

## Acceptance

S10 implementation must add `secloud validate proof-awareness`, proof gap fixtures, stale-green fixtures, and a proof-awareness artifact.
