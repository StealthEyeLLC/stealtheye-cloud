# Build Cockpit Card

## Purpose

The Build Cockpit Card gives the operator a compact answer to what is active, what is green, what failed, whether human input is needed, and what happens next.

## Required packet families

```text
BuildCockpitCardV0
CurrentMissionStatusV0
ProofStatusSummaryV0
NextActionStatusV0
HumanNeededFlagV0
ActiveBranchCardV0
ActivePrCardV0
LatestGreenProofCardV0
OneSentenceStatusV0
CurrentPhaseSummaryV0
NextHumanActionV0
```

## Required card fields

A build cockpit card must include:

1. current mission
2. active branch
3. active PR
4. latest proof status
5. failed proof if any
6. human-needed flag
7. next exact action
8. one-sentence status
9. boundary reason if stopped

## Default one-sentence format

```text
S10 setup is active on build/s10-assistant-optimization-layer-setup; no implementation has started; next action is merge setup green and execute docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md in a new tab.
```

## Acceptance

S10 implementation must add `secloud validate build-cockpit`, valid/invalid cockpit card fixtures, and a build-cockpit proof artifact.
