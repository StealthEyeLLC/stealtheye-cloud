# Handoff and Continuation

## Purpose

This document defines the exact StealthEye Cloud handoff procedure for continuing work from one ChatGPT tab to the next.

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

The next active tab must read, in order:

1. `AGENTS.md`
2. `STEALTHEYE_DECISIONS.md`
3. `STEALTHEYE_ACTIVE.md`
4. `STEALTHEYE_RELAY.md`
5. `STEALTHEYE_RELAY.json`
6. `STEALTHEYE_SEAL.json`
7. `NEXT_ACTION.md`

Then it performs `NEXT_ACTION.md` unless a true boundary is present.

## Current continuation target

After this docs/handoff update merges, continue with:

```text
S6 — Zero-Trust Cross-Cloud Gateway
```

Target branch:

```text
build/s6-zero-trust-cross-cloud-gateway
```

## Current operating envelope

Allowed:

1. GitHub-direct cloud-only implementation
2. one branch per coherent drop
3. one PR per coherent drop
4. CI proof before merge
5. exact failure repair
6. merge when green if approved envelope allows it

Stop for:

1. secrets
2. paid compute
3. account permission changes
4. deployment/production mutation
5. database mutation
6. private data exposure risk
7. unresolved high-impact product ambiguity

## Handoff rule

Do not ask the user to re-explain the S6/S7/S8 plan. The repo docs are the source of continuation truth.
