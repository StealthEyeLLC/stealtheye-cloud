# Repair Intelligence Layer

## Purpose

The Repair Intelligence Layer makes repair work surgical, batched, and proof-driven.

## Required packet families

```text
RepairTriagePlanV0
FailureClusterV0
RepairPriorityV0
RepairRegressionRiskV0
RepairScopeGuardV0
RepairConfidenceScoreV0
RepairBatchPlanV0
RepairMinimalityCheckV0
RepairRerunDecisionV0
```

## Required behavior

The assistant must:

1. inspect all failures before patching
2. cluster related failures
3. identify minimal repair scope
4. avoid speculative broad rewrites
5. patch exact failures only
6. rerun proof after the batch
7. avoid cleanup-only rerun commits unless truth requires them

## Relationship to S8 and S9

S8 defines proof-driven remediation claims.

S9 defines one-drop and batch-repair process discipline.

S10 improves the assistant’s decision quality when applying those rules.

## Acceptance

S10 implementation must add `secloud validate repair-intelligence`, clustered-failure fixtures, regression-risk fixtures, and a repair-triage proof artifact.
