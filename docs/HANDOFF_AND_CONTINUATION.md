# Handoff and Continuation

## Purpose

This document defines the StealthEye Cloud handoff procedure for continuing work from one ChatGPT tab to the next.

## Canonical handoff files

At saturation or before a major context switch, update:

```text
STEALTHEYE_RELAY.md
STEALTHEYE_RELAY.json
STEALTHEYE_SEAL.json
STEALTHEYE_ACTIVE.md
NEXT_ACTION.md
```

## Resume order

The next active tab must read:

1. `AGENTS.md`
2. `STEALTHEYE_DECISIONS.md`
3. `STEALTHEYE_ACTIVE.md`
4. `STEALTHEYE_RELAY.md`
5. `STEALTHEYE_RELAY.json`
6. `STEALTHEYE_SEAL.json`
7. `NEXT_ACTION.md`

Then it performs `NEXT_ACTION.md` unless a true boundary is present.

## Current continuation target

S10 setup docs.

Setup branch:

```text
build/s10-assistant-optimization-layer-setup
```

If the setup is already merged, continue from current `main` and begin S10 implementation using:

```text
docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md
```

## S9 final status

S9 is complete and merged through PR #16 at merge SHA `a5540d1fe77a0752a6a32b086a66b7b4bbec33ec`.

S9 — One-Drop Build Accelerator makes future phases and projects operate as close as possible to:

```text
one mission approval
→ one coherent repo mutation/drop
→ one PR
→ one proof wave
→ batched repairs
→ merge when green
```

S9 merged green after these workflows passed:

```text
proof-fast
proof-full
proof-browser
proof-mobile
proof-e2e
proof-gateway
proof-activations
proof-remediator
proof-build-accelerator
proof-windows-targeted
```

## S10 selected status

S10 is selected as:

```text
S10 — Assistant Optimization Layer
```

S10 setup branch is docs/state/prompt only. No S10 implementation has started.

S10 implementation branch after setup merges:

```text
build/s10-assistant-optimization-layer
```

S10 exists to make the active ChatGPT/StealthEye Cloud tab faster, less needy, more reliable across tabs, better at context loading, better at tool use, better at recovery, better at proof reasoning, and better at minimizing operator attention burden.

## S10 implementation handoff rule

The implementation tab must read `docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md` and all files listed in that prompt before building.

Do not ask the user to re-explain S0–S9 or the S10 objective. Use the repo docs, S10 spec, S10 support docs, and prompt artifact as continuation truth.

Do not reopen S6, S7, S8, or S9 architecture unless the user explicitly asks.

Fast mode reduces avoidable process friction only. It must not weaken validators, schemas, proof gates, safety boundaries, or merge discipline.

## Boundary reminders

Do not create or rely on:

```text
CLAUDE.md
.github/copilot-instructions.md
.cursorrules
soul.md
generic root MEMORY.md
generic root rules.md
```

Do not use browser-cookie/session-token automation.

Do not introduce secrets, paid compute, production deployment, database mutation, private data exposure, or account permission changes without explicit approval.
