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

Begin:

```text
S7 — First Real Activations
```

S6 completion truth:

```text
PR #8 merged: S6 — Zero-Trust Cross-Cloud Gateway
Merge SHA: dcaf60dce2b466178c3cff1ee4545d06f3e5075f
```

Verified green before S6 merge:

```text
proof-fast
proof-full
proof-e2e
proof-gateway
proof-windows-targeted
```

Next implementation branch:

```text
build/s7-first-real-activations
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

## S6 final status

S6 is complete and merged. It implemented contract/readiness infrastructure only. It did not activate live external services and did not claim production/database mutation.

S6 materialized crate note:

1. `secloud-permission` implements the external-auth readiness boundary.
2. `secloud-guard` implements gateway-security / guard readiness boundaries.
3. `secloud-repo-worker` implements git-worker readiness.
4. `secloud-repair-readiness` implements Remediator readiness.
5. `ModelTopologyBoundaryV0` replaces the blocked topology schema filename while `secloud validate prompt-topology` remains the public validator.

## S7 handoff rule

Do not ask the user to re-explain the S7 plan. Use the repo docs and merged S6 substrate as continuation truth. Start S7 on `build/s7-first-real-activations` only after this post-S6 state cleanup is merged green.

Do not reopen S6 architecture unless the user explicitly asks.
