# StealthEye Cloud

StealthEye Cloud is the no-local, ChatGPT-native GitHub/browser execution-body agent.

It is optimized for one active ChatGPT tab until saturation, public free CI proof, browser body proof, mission-level approvals, StealthEye Relay handoffs, and StealthEye Seal checkpoints.

## Current build state

Current status: **S0–S11 merged.**

Latest completed mission:

```text
S11 — One-Accept Mission Executor
```

S11 PR:

```text
#21
```

S11 merge SHA:

```text
da07e96c466f54086143a34422c47a60f6de1d2e
```

S11 prep PR:

```text
#20
```

S11 prep merge SHA:

```text
b416eadbdf5770dc9be75c716c032700d2f8e6f9
```

## S11 result

S11 implemented a GitHub-native one-accept mission executor:

```text
initial mission approval: 1
routine midpoint approvals: 0
human stops: true boundaries only
```

Implemented surface:

```text
crates/secloud-mission-executor
.github/workflows/mission-executor.yml
.github/workflows/proof-mission-executor.yml
scripts/s11-mission-executor-proof.mjs
scripts/check-s11-mission-executor-artifacts.mjs
.stealtheye/mission-executor/
docs/S11_FINAL_REPORT.md
```

## S9/S11 approval model

S9 preserves one-drop mode with one mission approval, batched repairs, and merge when green.

S11 extends that into one-accept mission execution.

## Post-merge proof freshness rule

```text
No direct post-merge truth commit counts as proven unless a fresh proof run verifies the resulting main HEAD.
```

This README is part of the direct post-merge truth update and must be verified on the resulting main HEAD before being treated as proven.

## Next action

```text
Await the next operator-selected mission. Do not start S12 until the operator chooses it.
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
