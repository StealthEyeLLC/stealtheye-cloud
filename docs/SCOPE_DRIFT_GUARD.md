# Scope Drift Guard

## Purpose

The Scope Drift Guard keeps S10 and all later phases from expanding beyond the approved mission.

## Required packet families

```text
ScopeDriftDetectorV0
PlanDeviationFindingV0
UnapprovedExpansionFindingV0
PhaseBoundaryGuardV0
NoNoveltyUnlessUsefulPolicyV0
ScopeContainmentReceiptV0
FeatureCreepFindingV0
```

## Rule

New capability is allowed only when it does at least one of these:

1. reduces operator friction
2. increases correctness
3. strengthens proof
4. improves continuation
5. preserves safety or public-safe boundaries

Otherwise, do not add it.

## Required behavior

The assistant must identify:

1. planned scope
2. unexpected additions
3. skipped required steps
4. unapproved expansions
5. phase boundary violations
6. novelty that is not useful

## Acceptance

S10 implementation must add `secloud validate scope-drift-guard`, drift valid/invalid fixtures, and a scope-drift proof artifact.
