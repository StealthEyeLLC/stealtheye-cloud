# Hypothesis Racing

## Purpose

Hypothesis Racing lets StealthEye Cloud evaluate bounded candidate approaches when a hard failure or uncertain implementation path appears.

## S4 scope

S4 establishes representation and validation contracts for:

1. `HypothesisRaceV0`
2. `CandidateBranchV0`
3. `CandidateReducerV0`
4. `RaceDecisionV0`

## Executable crate

```text
crates/secloud-hypothesis
```

## Stop conditions

1. proof passed
2. all candidates failed
3. budget boundary
4. authority boundary
5. private data risk

## Boundary

Hypothesis Racing is bounded and public-safe. It must not create unbounded swarms, hidden branches, paid compute, or uncontrolled worker activity.
