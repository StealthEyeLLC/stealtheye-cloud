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

Define or choose S11.

## S9 final status

S9 is complete and merged through PR #16 at merge SHA `a5540d1fe77a0752a6a32b086a66b7b4bbec33ec`.

## S10 final status

S10 is complete and merged through PR #19 at merge SHA `fd2bcda27a281fb080aaef472bd87123e4fe02b6`.

S10 — Assistant Optimization Layer adds assistant/operator optimization contracts, schemas, fixtures, proof artifacts, and proof workflow without claiming hidden autonomy.

## S11 handoff rule

Do not ask the user to re-explain S0–S10. Use repo docs and Relay/Seal/Active/Next Action as continuation truth.

Do not reopen S6, S7, S8, S9, or S10 architecture unless the user explicitly asks.

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
