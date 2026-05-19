# Agent Repo Map

## Purpose

This file gives ChatGPT and future agent workers a fast map of the StealthEye Cloud repository.

## Current phase

S0–S7 are merged green. S8 — StealthEye Cloud Remediator is active on `build/s8-remediator-mode`.

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
- `crates/secloud-relay/` — Relay validation
- `crates/secloud-seal/` — Seal validation
- `crates/secloud-mission/` — mission/drop validation contracts
- `crates/secloud-file-bundle/` — file bundle/drop contracts
- `crates/secloud-ci/` — CI evidence contracts
- `crates/secloud-control/` — closed high-level tool registry
- `crates/secloud-workers/` — worker/background surface registry
- `crates/secloud-learning/` — Skill/pattern learning contracts
- `crates/secloud-search/` — past-session search contracts
- `crates/secloud-hypothesis/` — bounded hypothesis racing contracts
- `crates/secloud-proof-viewer/` — public proof canvas contracts
- `crates/secloud-hardening/` — hardening/check contracts
- `crates/secloud-release/` — release-candidate contracts
- `crates/secloud-e2e/` — first end-to-end mission contracts
- `crates/secloud-gateway/` — gateway readiness and policy contracts
- `crates/secloud-mcp-adapters/` — adapter readiness contracts
- `crates/secloud-gemini-worker/` — model worker readiness contracts
- `crates/secloud-knowledge-mirror/` — knowledge mirror contracts
- `crates/secloud-notifications/` — notification contracts
- `crates/secloud-permission/` — external auth and permission boundaries
- `crates/secloud-guard/` — guard/taint/production boundary contracts
- `crates/secloud-repo-worker/` — repo worker readiness contracts
- `crates/secloud-mobile-qa/` — mobile/game QA readiness contracts
- `crates/secloud-repair-readiness/` — S6 Remediator readiness contracts
- `crates/secloud-activations/` — S7 active capability contracts
- `crates/secloud-remediator/` — S8 active Remediator Mode contracts
- `crates/secloud-cli/` — `secloud` CLI validators
- `schemas/` — public JSON Schema contracts
- `scripts/` — public-safe activation and Remediator proof scripts
- `browser/playwright/` — browser proof tests and artifact validation
- `public/proof/` — public proof canvas
- `docs/` — canonical specs, build plan, phase docs, and handoff docs
- `.github/workflows/` — public proof workflows
- `.stealtheye/` — state, memory, Relay, Seal, packet, Skill, and worker artifacts

## Current proof workflows

- `proof-fast.yml` — fast Linux proof
- `proof-full.yml` — fuller Linux proof with clippy
- `proof-browser.yml` — Playwright/browser/proof-canvas proof
- `proof-mobile.yml` — mobile Playwright proof
- `proof-e2e.yml` — release-candidate end-to-end proof
- `proof-gateway.yml` — gateway/readiness/guard/Remediator validator proof
- `proof-activations.yml` — S7 active capability proof
- `proof-remediator.yml` — S8 active Remediator Mode proof
- `proof-windows-targeted.yml` — targeted Windows proof

## Active implementation branch

```text
build/s8-remediator-mode
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
