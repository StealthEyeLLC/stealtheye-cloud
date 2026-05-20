# StealthEye Cloud

StealthEye Cloud is the no-local, ChatGPT-native GitHub/browser execution-body agent.

It is optimized for one active ChatGPT tab until saturation, public free CI proof, browser body proof, mission-level approvals, StealthEye Relay handoffs, and StealthEye Seal checkpoints.

## Current build state

Current status: **S0–S11 merged. S12 — One-Accept Mission Gauntlet is selected for the next build.**

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

## S12 selected mission

```text
S12 — One-Accept Mission Gauntlet
```

S12 planning doc:

```text
docs/S12_ONE_ACCEPT_MISSION_GAUNTLET.md
```

S12 purpose:

```text
Prove and harden S11 by running real bounded missions through the one-accept executor, adding a connector leverage layer for high-frequency proof/mission/repo actions, closing GitHub token/workflow/merge gaps, mirroring mission receipts safely through MCP-style resources, and proving one initial approval with zero routine midpoint approvals.
```

## S9/S11 approval model

S9 preserves one-drop mode with one mission approval, batched repairs, and merge when green.

S11 extends that into one-accept mission execution.

S12 must prove and harden that path.

## Post-merge proof freshness rule

```text
No direct post-merge truth commit counts as proven unless a fresh proof run verifies the resulting main HEAD.
```

Current caveat:

```text
The direct post-S11 truth commit 8988e32fc61e2824dcc19eef30da2894112ea9f9 is present but not proven because no fresh workflow runs or combined statuses are visible for that commit through the connector.
```

S12 must include a command-dispatch or connector leverage path that can trigger and verify fresh main-head proof.

## Next action

```text
Implement S12 — One-Accept Mission Gauntlet from docs/S12_ONE_ACCEPT_MISSION_GAUNTLET.md on branch build/s12-one-accept-mission-gauntlet.
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
