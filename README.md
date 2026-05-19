# StealthEye Cloud

StealthEye Cloud is the no-local, ChatGPT-native GitHub/browser execution-body agent.

It is optimized for one active ChatGPT tab until saturation, public free CI proof, browser body proof, mission-level approvals, StealthEye Relay handoffs, and StealthEye Seal checkpoints.

## Current build state

Current status before S8 merge: **S0–S7 merged green; S8 implementation branch active**.

Completed spine:

```text
S0 — Foundation, continuity, packet spine, and cheap CI
S1 — Mission executor, atomic drop rail, authority queue, and GitHub evidence
S2 — Browser body, replay proof, and visual evidence
S3 — MCP/App control plane, tool registry, Skills, workers, and background capability reality
S4 — Self-improving Skills, past-session search, hypothesis racing, and public proof canvas
S5 — Full hardening, public release candidate, and first end-to-end mission
S6 — Zero-Trust Cross-Cloud Gateway
S7 — First Real Activations
```

Active branch:

```text
build/s8-remediator-mode
```

S8 activates:

```text
StealthEye Cloud Remediator
```

A repo is not remediated until failing behavior is reproduced, a bounded patch is applied, and proof gates pass. If failure cannot be reproduced, Remediator emits diagnosis-only status and does not claim remediation.

## Recent completion truth

```text
S6 PR #8 merge SHA: dcaf60dce2b466178c3cff1ee4545d06f3e5075f
Post-S6 cleanup PR #9 merge SHA: a5e6eccc37067cf264fd8859c69fc412da855bb8
S7 PR #11 merge SHA: d814507740b1ab9a58dd5a2e9a4e079e21bf1d78
```

## Start here

1. Read `AGENTS.md`.
2. Read `STEALTHEYE_DECISIONS.md`.
3. Read `STEALTHEYE_ACTIVE.md`.
4. Read `STEALTHEYE_RELAY.md` and `STEALTHEYE_SEAL.json` when resuming.
5. Perform `NEXT_ACTION.md` unless a boundary is required.

## Canonical docs

- `docs/StealthEye_Cloud_Final_Technical_Specification.md`
- `docs/StealthEye_Cloud_Build_Plan.md`
- `docs/S6_S7_S8_ROADMAP.md`
- `docs/S6_ZERO_TRUST_CROSS_CLOUD_GATEWAY.md`
- `docs/S7_FIRST_REAL_ACTIVATIONS.md`
- `docs/S8_STEALTHEYE_CLOUD_REMEDIATOR.md`
- `docs/S8_FINAL_REPORT.md`
- `docs/HANDOFF_AND_CONTINUATION.md`

## Current proof workflows

- `proof-fast.yml`
- `proof-full.yml`
- `proof-browser.yml`
- `proof-mobile.yml`
- `proof-e2e.yml`
- `proof-gateway.yml`
- `proof-activations.yml`
- `proof-remediator.yml`
- `proof-windows-targeted.yml`

## Canonical operating model

```text
one active ChatGPT tab until saturation
Relay + Seal + Active + Next Action at handoff
GitHub branch + PR per coherent drop
CI proof before merge
repair exact failures only
merge only when green
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
