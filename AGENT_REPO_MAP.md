# Agent Repo Map

## Purpose

This file gives ChatGPT and future agent workers a fast map of the StealthEye Cloud repository.

## Current phase

S0–S5 merged green. Current branch updates docs and handoff for the S6/S7/S8 build wave.

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
- `crates/secloud-cli/` — `secloud` CLI validators
- `schemas/` — public JSON Schema contracts
- `browser/playwright/` — browser proof tests and artifact validation
- `public/proof/` — public proof canvas
- `docs/` — canonical specs, build plan, phase docs, and handoff docs
- `.github/workflows/` — public proof workflows
- `.stealtheye/` — state, memory, Relay, Seal, packet, Skill, and worker artifacts

## Current proof workflows

- `proof-fast.yml` — fast Linux proof
- `proof-full.yml` — fuller Linux proof with clippy
- `proof-browser.yml` — Playwright/browser/proof-canvas proof
- `proof-e2e.yml` — release-candidate end-to-end proof
- `proof-windows-targeted.yml` — targeted Windows proof

## Next build wave docs

- `docs/S6_S7_S8_ROADMAP.md`
- `docs/S6_ZERO_TRUST_CROSS_CLOUD_GATEWAY.md`
- `docs/S7_FIRST_REAL_ACTIVATIONS.md`
- `docs/S8_STEALTHEYE_CLOUD_REMEDIATOR.md`
- `docs/HANDOFF_AND_CONTINUATION.md`
- `docs/TECHNICAL_SPEC_STATUS_ADDENDUM.md`

## Next implementation branch

```text
build/s6-zero-trust-cross-cloud-gateway
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
