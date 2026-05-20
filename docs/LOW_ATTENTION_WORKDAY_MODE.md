# Low-Attention Workday Mode

## Purpose

Low-Attention Workday Mode minimizes operator interruptions while preserving true authority boundaries.

## Required packet families

```text
HumanAvailabilityProfileV0
LowAttentionWorkdayModeV0
UserAttentionBudgetV0
InterruptionAvoidancePolicyV0
CriticalOnlyStopPolicyV0
WorkdayProgressDigestV0
AttentionCostScoreV0
```

## Default behavior

When low-attention mode is active, the assistant must:

1. continue routine approved work without asking midpoint questions
2. send short progress updates only at useful milestones
3. avoid low-value status chatter
4. stop only for true boundaries
5. provide one exact required human action when blocked
6. produce a compact final summary

## Milestone update triggers

Send updates when:

1. branch is created
2. PR is opened
3. CI fails
4. failure cluster is identified
5. repair is pushed
6. CI is green
7. merge completes
8. a true boundary is hit
9. saturation handoff is needed

## Acceptance

S10 implementation must add `secloud validate low-attention-mode`, valid/invalid fixtures, and a low-attention proof report.
