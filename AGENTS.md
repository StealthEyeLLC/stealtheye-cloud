# AGENTS.md

## Product

This repository is **StealthEye Cloud**: the no-local, ChatGPT-native GitHub/browser execution-body agent.

## Primary operating mode

Use one active ChatGPT tab until saturation. When saturated, generate and update:

1. `STEALTHEYE_RELAY.md`
2. `STEALTHEYE_RELAY.json`
3. `STEALTHEYE_SEAL.json`
4. `STEALTHEYE_ACTIVE.md`
5. `NEXT_ACTION.md`

The next tab resumes from those artifacts without asking the user to re-explain.

## Truth order

Use this order when facts conflict:

1. current user instruction
2. current repo state
3. current CI/browser result
4. latest StealthEye Seal
5. `STEALTHEYE_ACTIVE.md`
6. `STEALTHEYE_DECISIONS.md`
7. `STEALTHEYE_RELAY.md` / `.json`
8. older chat memory

## Build doctrine

Build in big, final-form drops. Do not create toy/prototype versions of core systems. A valid drop includes implementation, schemas, tests/fixtures, docs, workflows, state updates, and proof gates.

## Planning doctrine

Planning must stay compact. For each future S phase, use one planning document maximum unless the user explicitly asks for more. Do not create a document forest. Do not create prompt docs unless explicitly requested. Give implementation prompts directly in chat when the user asks for prompts.

## Approval doctrine

After mission approval, do not ask whether to continue, rerun safe CI, update tests, update docs, repair a compile/test failure, or generate handoff artifacts. Continue until green, blocked, saturated, or a true boundary is reached.

Stop only for secrets, passwords, paid compute, destructive irreversible action, private data exposure risk, account permission changes, production deployment, database mutation, browser-cookie/session-token automation, unresolved high-impact ambiguity, or a GitHub permission/ruleset boundary.

## Stack

- Rust owns durable core, schemas, validators, CLI, packets, Relay, Seal, memory, mission executor, proof models, build accelerator, S10 assistant optimizer contracts, and S11 mission executor contracts.
- TypeScript owns browser proof and browser observation.
- JSON Schema owns public packet contracts.
- Markdown owns agent-readable state and docs.
- GitHub Actions owns no-local execution proof.

## Required root files

- `AGENTS.md`
- `llms.txt`
- `llms-full.txt`
- `AGENT_REPO_MAP.md`
- `STEALTHEYE_ACTIVE.md`
- `STEALTHEYE_DECISIONS.md`
- `STEALTHEYE_RELAY.md`
- `STEALTHEYE_RELAY.json`
- `STEALTHEYE_SEAL.json`
- `NEXT_ACTION.md`

## Forbidden files

Do not create or rely on:

- `CLAUDE.md`
- `.github/copilot-instructions.md`
- `.cursorrules`
- `soul.md`
- generic root `MEMORY.md`
- generic root `rules.md`

## Current state

S0 through S10 are merged. S10 — Assistant Optimization Layer is complete.

S10 PR #19 merged at:

```text
fd2bcda27a281fb080aaef472bd87123e4fe02b6
```

Important caveat: the direct post-S10 truth commit `7e500a4cb52eca01f9ebc2708d62e6ea70a74ee2` did not spawn a fresh Actions run through the connector, so that direct truth commit is not separately CI-verified.

## Selected next mission

```text
S11 — One-Accept Mission Executor
```

## S11 objective

Build a real GitHub-native mission executor so one approved mission lease can complete routine repo/build/proof/repair/merge work without repeated operator confirmations.

## S11 planning doc

```text
docs/S11_ONE_ACCEPT_MISSION_EXECUTOR.md
```

## Next implementation branch

```text
build/s11-one-accept-mission-executor
```
