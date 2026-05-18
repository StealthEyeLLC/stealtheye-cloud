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

## Approval doctrine

After mission approval, do not ask whether to continue, rerun safe CI, update tests, update docs, repair a compile/test failure, or generate handoff artifacts. Continue until green, blocked, saturated, or a true boundary is reached.

Stop only for secrets, passwords, paid compute, destructive irreversible action, private data exposure risk, account permission changes, or unresolved high-impact ambiguity.

## Stack

- Rust owns durable core, schemas, validators, CLI, packets, Relay, Seal, memory, mission executor, and proof models.
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

## S0 current objective

Implement the foundation, continuity, packet spine, and cheap CI proof.
