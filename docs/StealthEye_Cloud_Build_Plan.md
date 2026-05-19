# StealthEye Cloud Build Plan

## 0. Status

This document is the current build plan for `StealthEyeLLC/stealtheye-cloud`.

Current verified state:

```text
S0–S8 merged green
S6 PR #8 merge SHA: dcaf60dce2b466178c3cff1ee4545d06f3e5075f
Post-S6 cleanup PR #9 merge SHA: a5e6eccc37067cf264fd8859c69fc412da855bb8
S7 PR #11 merge SHA: d814507740b1ab9a58dd5a2e9a4e079e21bf1d78
S8 PR #13 merge SHA: 12081b4d311844b62aecafb5ff045414e94a4a7c
Post-S8 cleanup PR #14 merge SHA: e45b0e75fc9b1f8a9e1ed09db90d69037fe9c11d
```

Active mission:

```text
S9 — One-Drop Build Accelerator
```

## 1. Build Doctrine

### 1.1 Big coherent drops

StealthEye Cloud is built in large, coherent, final-form drops. Each drop must include implementation or contract crate, schemas, validators, tests or proof gates, docs, workflows, state updates, and final report.

### 1.2 One-drop / one-approval target

S9 upgrades the doctrine so future phases target:

```text
one mission approval
→ one coherent repo mutation/drop
→ one PR
→ one proof wave
→ batched repairs
→ merge when green
```

Routine work under a mission approval should not trigger repeated confirmation requests.

### 1.3 No placeholders / no fake features

Do not ship fake integrations, placeholder capabilities, mock success paths labeled real, or docs-only feature claims.

### 1.4 No weakening

Fast mode never means weaker proof. S9 must preserve or strengthen validators, schema inventory, workflow proof, safety boundaries, and green-before-merge discipline.

### 1.5 Public-safe rule

The public repo contains only public-safe implementation and proof artifacts.

Do not include secrets, private strategy, client data, private overlays, browser cookies, consumer session tokens, API keys, or paid-compute assumptions.

## 2. Handoff Doctrine

Use one active ChatGPT tab until saturation. At handoff, update:

```text
STEALTHEYE_RELAY.md
STEALTHEYE_RELAY.json
STEALTHEYE_SEAL.json
STEALTHEYE_ACTIVE.md
NEXT_ACTION.md
```

The next tab resumes by reading those files in that order, then performing `NEXT_ACTION.md` unless a true boundary is present.

## 3. Completed Build Spine

S0 through S8 are merged green.

### S6 — Zero-Trust Cross-Cloud Gateway

Status: merged.

### S7 — First Real Activations

Status: merged.

### S8 — StealthEye Cloud Remediator

Status: merged.

Crate:

```text
crates/secloud-remediator
```

Workflow:

```text
.github/workflows/proof-remediator.yml
```

## 4. Active Build Wave

### S9 — One-Drop Build Accelerator

Status: implementation branch active.

Target branch:

```text
build/s9-one-drop-build-accelerator
```

Purpose: make every future phase/project operate as close as possible to one mission approval, one coherent repo mutation/drop, one PR, one proof wave, batched repairs, and merge when green.

Crate:

```text
crates/secloud-build-accelerator
```

Required workflow:

```text
.github/workflows/proof-build-accelerator.yml
```

Required S9 validators are registered through `secloud validate ...` and included in `secloud doctor`.

S9 acceptance:

1. `crates/secloud-build-accelerator` exists and compiles.
2. S9 schemas are added to the schema inventory.
3. S9 validators are callable through `secloud-cli`.
4. `secloud doctor` includes S9 validators.
5. `proof-build-accelerator` is green.
6. State consistency catches README/build-plan/Active/Relay/Seal/Next Action conflicts.
7. No-cleanup-PR gate detects stale post-merge language.
8. Build velocity report and confirmation friction ledger are emitted as artifacts.
9. `docs/PROMPTS/NEXT_TAB_PROMPT.md` and `docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md` exist.
10. Future phase default contract is documented and validated.

## 5. Stop Conditions

Stop for secrets, paid compute, account permission changes, private data exposure risk, deployment/production mutation without explicit approval, database mutation without explicit approval, legal/commercial commitment, platform auth boundary, browser-cookie/session-token automation, or unresolved high-impact ambiguity.

Do not stop for routine continuation, docs updates, CI repair, validator wiring, state updates, PR creation, or merge when green if the mission approval envelope covers it.

## 6. Immediate Next Action

Open the S9 implementation PR from:

```text
build/s9-one-drop-build-accelerator
```

Run the full proof wave, batch exact repairs if needed, and merge when green.
