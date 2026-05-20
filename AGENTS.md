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

## Default operating mode

When planning, evaluating, enumerating, choosing, researching, or prepping a StealthEye phase, automatically apply:

1. source-grounded full pass if repo/files are involved
2. web research if the topic is current, technical, protocol/security/GitHub-related, or uncertain
3. inventory before recommendation
4. enumerate options, upgrades, and failure modes
5. rank by leverage/friction
6. define proof path and acceptance criteria
7. list non-goals
8. give the minimum complete version
9. do not act unless the operator explicitly says execute, prep, or build
10. use one planning document maximum if repo prep is requested

## Build doctrine

Build in big, final-form drops. Do not create toy/prototype versions of core systems. A valid drop includes implementation, schemas, tests/fixtures, docs, workflows, state updates, and proof gates.

## Planning doctrine

Planning must stay compact. For each future S phase, use one planning document maximum unless the user explicitly asks for more. Do not create a document forest. Do not create prompt docs unless explicitly requested. Give implementation prompts directly in chat when the user asks for prompts.

## Approval doctrine

After mission approval, do not ask whether to continue, rerun safe CI, update tests, update docs, repair a compile/test failure, or generate handoff artifacts. Continue until green, blocked, saturated, or a true boundary is reached.

Stop only for secrets, passwords, paid compute, destructive irreversible action, private data exposure risk, account permission changes, production deployment, database mutation, browser-cookie/session-token automation, unresolved high-impact ambiguity, or a GitHub permission/ruleset boundary.

## Stack

- Rust owns durable core, schemas, validators, CLI, packets, Relay, Seal, memory, mission executor, mission gauntlet, connector leverage, proof models, build accelerator, S10 assistant optimizer contracts, and S11 mission executor contracts.
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

S0 through S11 are merged. S11 — One-Accept Mission Executor is complete.

S11 PR #21 merged at:

```text
da07e96c466f54086143a34422c47a60f6de1d2e
```

Important caveat: direct post-merge truth commit `8988e32fc61e2824dcc19eef30da2894112ea9f9` is present but not proven because no fresh workflow runs or combined statuses are visible for that commit through the connector.

## Selected next mission

```text
S12 — One-Accept Mission Gauntlet
```

## S12 objective

Prove and harden S11 by running real bounded missions through the one-accept executor, adding a connector leverage layer for high-frequency proof/mission/repo actions, closing GitHub token/workflow/merge gaps, mirroring mission receipts safely through MCP-style resources, and proving one initial approval with zero routine midpoint approvals.

## S12 planning doc

```text
docs/S12_ONE_ACCEPT_MISSION_GAUNTLET.md
```

## Next implementation branch

```text
build/s12-one-accept-mission-gauntlet
```
