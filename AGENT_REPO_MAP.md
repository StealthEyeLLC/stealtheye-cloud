# Agent Repo Map

## Purpose

This file gives ChatGPT and future agent workers a fast map of the StealthEye Cloud repository.

## Current phase

S0–S9 are merged green. S9 — One-Drop Build Accelerator is complete. S10 — Assistant Optimization Layer is selected for setup. No S10 implementation has started.

## Current setup branch

```text
build/s10-assistant-optimization-layer-setup
```

## Next implementation branch

```text
build/s10-assistant-optimization-layer
```

## Next action

```text
Complete and merge the S10 setup docs PR, then start S10 implementation from current main using docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md.
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
- `crates/secloud-build-accelerator/` — S9 one-drop build accelerator contracts
- `crates/secloud-cli/` — `secloud` CLI validators
- `schemas/` — public JSON Schema contracts
- `scripts/` — public-safe activation, Remediator, and build-accelerator proof scripts
- `browser/playwright/` — browser proof tests and artifact validation
- `public/proof/` — public proof canvas
- `docs/` — canonical specs, build plan, phase docs, and handoff docs
- `.github/workflows/` — public proof workflows
- `.stealtheye/` — state, memory, Relay, Seal, packet, Skill, and worker artifacts

## S10 planned implementation folders/files

S10 implementation must add:

- `crates/secloud-assistant-optimizer/` — S10 assistant/operator optimization contracts
- `.github/workflows/proof-assistant-optimizer.yml` — S10 proof workflow
- `scripts/s10-assistant-optimizer-proof.mjs` — S10 proof artifact generator
- `scripts/check-s10-assistant-optimizer-artifacts.mjs` — S10 artifact checker
- `.stealtheye/assistant-optimizer/` — generated S10 proof artifacts

## Current proof workflows

- `proof-fast.yml` — fast Linux proof
- `proof-full.yml` — fuller Linux proof with clippy
- `proof-browser.yml` — Playwright/browser/proof-canvas proof
- `proof-mobile.yml` — mobile Playwright proof
- `proof-e2e.yml` — release-candidate end-to-end proof
- `proof-gateway.yml` — gateway/readiness/guard/Remediator validator proof
- `proof-activations.yml` — S7 active capability proof
- `proof-remediator.yml` — S8 active Remediator Mode proof
- `proof-build-accelerator.yml` — S9 one-drop build accelerator proof
- `proof-windows-targeted.yml` — targeted Windows proof

S10 implementation must add:

- `proof-assistant-optimizer.yml` — S10 assistant optimizer proof

## S10 docs to scan before implementation

- `docs/S10_ASSISTANT_OPTIMIZATION_LAYER.md`
- `docs/ASSISTANT_OPERATING_PROFILE.md`
- `docs/MISSION_INTAKE_OPTIMIZER.md`
- `docs/CONTEXT_LOAD_POLICY.md`
- `docs/REPO_TRUTH_FIRST_POLICY.md`
- `docs/LOW_ATTENTION_WORKDAY_MODE.md`
- `docs/HANDOFF_QUALITY_GATE.md`
- `docs/TOOL_USE_PLANNER.md`
- `docs/TOOL_FALLBACK_POLICY.md`
- `docs/PROMPT_COMPRESSION.md`
- `docs/SCOPE_DRIFT_GUARD.md`
- `docs/PROOF_AWARENESS_LAYER.md`
- `docs/REPAIR_INTELLIGENCE_LAYER.md`
- `docs/READ_ONLY_VERIFICATION_MODE.md`
- `docs/ASSISTANT_SELF_AUDIT.md`
- `docs/CAPABILITY_REALITY_MAP.md`
- `docs/BUILD_COCKPIT_CARD.md`
- `docs/AGENT_OBSERVABILITY_TRACE_DIGEST.md`
- `docs/MCP_AWARE_OPERATOR_POLICY.md`
- `docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md`

## Completed S9 implementation branch

```text
build/s9-one-drop-build-accelerator
```

## S9 merge truth

```text
PR #16
a5540d1fe77a0752a6a32b086a66b7b4bbec33ec
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
