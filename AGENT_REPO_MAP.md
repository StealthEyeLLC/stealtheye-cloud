# Agent Repo Map

## Purpose

This file gives ChatGPT and future agent workers a fast map of the StealthEye Cloud repository.

## Current phase

S0–S10 are merged green. S10 — Assistant Optimization Layer is complete.

## Current branch

```text
main
```

## Next action

```text
Define or choose S11.
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
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
