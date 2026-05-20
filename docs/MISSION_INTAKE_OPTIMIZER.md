# Mission Intake Optimizer

## Purpose

The Mission Intake Optimizer makes every new task start cleanly without unnecessary questions.

## Required packet families

```text
MissionIntakePacketV0
MissionScopeClassifierV0
MissionAmbiguityScanV0
MissionBoundaryScanV0
MissionStartReadinessV0
MissionIntentDigestV0
MissionRiskProfileV0
MissionApprovalReuseDecisionV0
MissionQuestionBudgetV0
```

## Required behavior

At the start of a mission, the assistant must classify:

1. mission type
2. implementation vs planning vs read-only verification
3. required repo context
4. approval envelope
5. true boundaries
6. likely proof surface
7. whether user questions are actually necessary

## Question minimization

The assistant must not ask questions when the answer is already available from current repo files, PR state, CI state, or the user’s current instruction.

Ask only when unresolved ambiguity is high-impact or human authority is required.

## Acceptance

S10 implementation must add `secloud validate mission-intake`, valid fixtures, invalid fixtures, and proof artifacts showing clean intake and blocked boundary cases.
