# Agent Repo Map

## Purpose

This file gives ChatGPT and future agent workers a fast map of the StealthEye Cloud repository.

## Current phase

S0–S10 are merged. S10 — Assistant Optimization Layer is complete. S11 — One-Accept Mission Executor is selected.

## Current prep branch

```text
plan/s11-one-accept-mission-executor
```

## Next implementation branch

```text
build/s11-one-accept-mission-executor
```

## Next action

```text
Implement S11 — One-Accept Mission Executor from docs/S11_ONE_ACCEPT_MISSION_EXECUTOR.md.
```

## Root files

- `AGENTS.md` — operating rules and agent instructions
- `llms.txt` — compact LLM index
- `llms-full.txt` — expanded LLM index
- `STEALTHEYE_ACTIVE.md` — current mission state
- `STEALTHEYE_DECISIONS.md` — canonical decisions
- `STEALTHEYE_RELAY.md` — human-readable handoff
- `STEALTHEYE_RELAY.json` — machine-readable handoff
- `STEALTHEYE_SEAL.json` — latest universal checkpoint
- `NEXT_ACTION.md` — one exact next action
- `README.md` — public repo summary and current state

## Current implementation folders

- `crates/secloud-packets/` — packet type names and schema inventory
- `crates/secloud-build-accelerator/` — S9 one-drop build accelerator contracts
- `crates/secloud-assistant-optimizer/` — S10 assistant/operator optimization contracts
- `crates/secloud-cli/` — `secloud` CLI validators
- `schemas/` — public JSON Schema contracts
- `scripts/` — public-safe proof scripts
- `fixtures/s10-assistant-optimizer/` — S10 valid and invalid fixtures
- `browser/playwright/` — browser proof tests and artifact validation
- `public/proof/` — public proof canvas
- `docs/` — canonical specs, build plan, phase docs, and handoff docs
- `.github/workflows/` — public proof workflows
- `.stealtheye/` — state, Relay, Seal, packet, Skill, and worker artifacts

## Planned S11 implementation surface

- `docs/S11_ONE_ACCEPT_MISSION_EXECUTOR.md` — one S11 planning document
- `crates/secloud-mission-executor/` — S11 mission executor contracts
- `.github/workflows/mission-executor.yml` — one-accept mission execution workflow
- `.github/workflows/proof-mission-executor.yml` — S11 proof workflow
- `scripts/s11-mission-executor-proof.mjs` — S11 proof script
- `.stealtheye/mission-executor/` — S11 proof/state artifacts
- `docs/S11_FINAL_REPORT.md` — final implementation report

No prompt doc and no subsystem document forest.

## Current proof workflows

- `proof-fast.yml`
- `proof-full.yml`
- `proof-browser.yml`
- `proof-mobile.yml`
- `proof-e2e.yml`
- `proof-gateway.yml`
- `proof-activations.yml`
- `proof-remediator.yml`
- `proof-build-accelerator.yml`
- `proof-assistant-optimizer.yml`
- `proof-windows-targeted.yml`

## Recent merge truth

```text
S9 PR #16 merge SHA: a5540d1fe77a0752a6a32b086a66b7b4bbec33ec
S10 PR #19 merge SHA: fd2bcda27a281fb080aaef472bd87123e4fe02b6
Post-S10 direct truth commit: 7e500a4cb52eca01f9ebc2708d62e6ea70a74ee2, not separately CI-verified
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
