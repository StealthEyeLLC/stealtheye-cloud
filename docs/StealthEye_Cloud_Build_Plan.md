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
```

S8 activated:

```text
S8 — StealthEye Cloud Remediator
```

## 1. Build Doctrine

### 1.1 Big coherent drops

StealthEye Cloud is built in large, coherent, final-form drops. Each drop must include all files needed for the capability it claims:

1. implementation or validated contract crate
2. schemas
3. validators
4. tests or proof gates
5. docs
6. GitHub Actions workflow updates when needed
7. Active/Relay/Seal/Next Action updates
8. final report

### 1.2 No placeholders / no fake features

Do not ship fake integrations, placeholder capabilities, mock success paths labeled real, or docs-only feature claims.

A lane may be called `readiness` only when it builds real contracts, validators, schemas, docs, and proof gates.

A lane may be called `active` only when it performs the real action and CI/proof artifacts validate it.

### 1.3 Public-safe rule

The public repo contains only public-safe implementation and proof artifacts.

Do not include secrets, private strategy, client data, private overlays, browser cookies, consumer session tokens, API keys, or paid-compute assumptions.

### 1.4 Cloud-only execution rule

The default operating mode is GitHub-direct and no-local. Local/laptop work is disabled unless explicitly requested or a catastrophe blocks cloud-only progress.

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

Purpose: zero-trust readiness/enforcement substrate for gateway transport/session/origin/backpressure contracts, adapter lifecycle/integrity/catalog contracts, Gemini worker readiness, semantic normalization, data-tainting, external authority boundaries, knowledge mirror, notification, repo worker, mobile/game QA, Remediator readiness, full S6 schema inventory, validator surface, and dedicated `proof-gateway` workflow.

S6 did not activate live external services, automate browser sessions/cookies, or mutate production/database systems.

### S7 — First Real Activations

Status: merged.

Purpose: activated mobile browser game preview/playtest proof, notification dry-run/conditional dispatch, and knowledge mirror export.

S7 did not use browser-cookie/session-token automation, commit or print secrets, use paid compute, deploy production systems, mutate databases, or perform live external mirror sync.

### S8 — StealthEye Cloud Remediator

Status: merged.

Purpose: activated proof-driven Remediator Mode.

Crate:

```text
crates/secloud-remediator
```

Workflow:

```text
.github/workflows/proof-remediator.yml
```

S8 activated a public-safe remediation proof body. A repo is not remediated until the failing behavior is reproduced, a bounded patch is applied, and proof gates pass. If failure cannot be reproduced, Remediator emits diagnosis-only status and does not claim remediation.

### S8 modules

```text
intake
permissions
reality_map
command_discovery
environment
reproduction
failure_taxonomy
localization
repair_strategy
patch_tournament
proof_plan
report
quote_risk
```

### S8 acceptance passed

S8 passed with green:

1. `proof-fast`
2. `proof-full`
3. `proof-e2e`
4. `proof-gateway`
5. `proof-browser`
6. `proof-mobile`
7. `proof-activations`
8. `proof-remediator`
9. `proof-windows-targeted`

## 4. Active Build Wave

No active build wave is selected after S8. Choose or define S9 before implementation.

## 5. Stop Conditions

Stop for:

1. secrets
2. paid compute
3. account permission changes
4. private data exposure risk
5. deployment/production mutation without explicit approval
6. database mutation without explicit approval
7. legal/commercial commitment
8. platform auth boundary
9. unresolved high-impact ambiguity

Do not stop for routine continuation, docs updates, CI repair, validator wiring, state updates, or handoff generation.

## 6. Immediate Next Action

Choose or define S9, then begin it from current `main` in a new `build/s9-*` branch.
