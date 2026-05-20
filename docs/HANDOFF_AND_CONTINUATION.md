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

Post-S9 truth cleanup.

Cleanup branch:

```text
build/post-s9-truth-cleanup
```

If the cleanup is already merged, continue from current `main`.

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

No S10 implementation has started.

The next action is to define or choose S10.

Fast mode reduces avoidable process friction only. It must not weaken validators, schemas, proof gates, safety boundaries, or merge discipline.
