# StealthEye Cloud

StealthEye Cloud is the no-local, ChatGPT-native GitHub/browser execution-body agent.

It is optimized for one active ChatGPT tab until saturation, public free CI proof, browser body proof, mission-level approvals, StealthEye Relay handoffs, and StealthEye Seal checkpoints.

## Current build state

Current status: **S0–S10 merged. S11 — One-Accept Mission Executor is in implementation on `build/s11-one-accept-mission-executor`.**

Latest completed mission:

```text
S10 — Assistant Optimization Layer
```

S10 PR:

```text
#19
```

S10 merge SHA:

```text
fd2bcda27a281fb080aaef472bd87123e4fe02b6
```

S11 prep PR:

```text
#20
```

S11 prep merge SHA:

```text
b416eadbdf5770dc9be75c716c032700d2f8e6f9
```

Important caveat solved by S11 policy:

```text
S10 PR #19 was green before merge. A direct post-merge truth commit was made afterward at 7e500a4cb52eca01f9ebc2708d62e6ea70a74ee2. That direct post-merge truth commit did not spawn a fresh Actions run through the connector, so it is not separately CI-verified.

S11 adds PostMergeProofFreshnessGateV0: no direct post-merge truth commit counts as proven unless a fresh workflow_dispatch proof run verifies the resulting main HEAD.
```

## S11 implementation surface

```text
crates/secloud-mission-executor
.github/workflows/mission-executor.yml
.github/workflows/proof-mission-executor.yml
scripts/s11-mission-executor-proof.mjs
scripts/check-s11-mission-executor-artifacts.mjs
.stealtheye/mission-executor/
docs/S11_FINAL_REPORT.md
```

## S11 target operator experience

```text
initial mission approval: 1
routine midpoint approvals: 0
human stops: true boundaries only
```

## Next action

```text
Open the S11 implementation PR, run the relevant proof lanes, inspect and patch only real failures, and merge when green.
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
