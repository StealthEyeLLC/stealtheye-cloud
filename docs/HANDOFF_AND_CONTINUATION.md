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

Continue:

```text
S8 — StealthEye Cloud Remediator
```

Active implementation branch:

```text
build/s8-remediator-mode
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
4. production deployment without explicit approval
5. database mutation without explicit approval
6. browser-cookie/session-token automation
7. private data exposure risk
8. unresolved high-impact product ambiguity

## S7 final status

S7 is complete and merged. It activated mobile browser playtest proof, notification dry-run/conditional dispatch, and knowledge mirror export. It did not commit or print secrets, use paid compute, deploy production systems, mutate databases, perform live external mirror sync, or automate browser sessions/cookies.

## S8 active status

S8 implementation package is active. It adds `crates/secloud-remediator`, active S8 validators, `proof-remediator`, and a synthetic broken-repo remediation proof that reproduces failure, applies a bounded patch, reruns proof green, emits remediation artifacts, and preserves diagnosis-only handling for unreproduced failures.

## S8 handoff rule

Do not ask the user to re-explain the S8 plan. Use the repo docs and merged S6/S7 substrate as continuation truth. Continue S8 on `build/s8-remediator-mode`.

Do not reopen S6 or S7 architecture unless the user explicitly asks.
