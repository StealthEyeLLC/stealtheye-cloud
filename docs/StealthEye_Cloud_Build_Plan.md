# StealthEye Cloud Build Plan

## 0. Status

This document is the current build plan for `StealthEyeLLC/stealtheye-cloud`.

Current verified state:

```text
S0–S6 merged green
S6 PR #8 merge SHA: dcaf60dce2b466178c3cff1ee4545d06f3e5075f
Post-S6 cleanup PR #9 merge SHA: a5e6eccc37067cf264fd8859c69fc412da855bb8
```

S6 completed the zero-trust cross-cloud gateway readiness/enforcement substrate and was verified before merge with these gates green:

```text
proof-fast
proof-full
proof-e2e
proof-gateway
proof-windows-targeted
```

The next locked build wave is:

```text
S7 — First Real Activations
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

### 1.3 No watered-down builds

Narrow scope by selecting fewer capabilities per drop, not by weakening architecture.

Do not weaken tests, remove validators, hide failures, or fake green proof.

### 1.4 Public-safe rule

The public repo contains only public-safe implementation and proof artifacts.

Do not include secrets, private strategy, client data, private overlays, browser cookies, consumer session tokens, API keys, or paid-compute assumptions.

### 1.5 Cloud-only execution rule

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

### S0 — Foundation, Continuity, Packet Spine, and Cheap CI

Status: merged.

Purpose: canonical root files, Relay, Seal, Active, packet spine, forbidden-file policy, Rust workspace, and cheap CI.

### S1 — Mission Executor, Atomic Drop Rail, Authority Queue, and GitHub Evidence

Status: merged.

Purpose: mission/drop representation, authority queue, output shelf, GitHub evidence, failure cards, and CI-backed repair loop.

### S2 — Browser Body, Replay Proof, and Visual Evidence

Status: merged.

Purpose: Playwright browser proof, DOM/console/network/screenshot artifacts, replay contracts, and visual evidence surface.

### S3 — MCP/App Control Plane, Tool Registry, Skills, Workers, and Background Capability Reality

Status: merged.

Purpose: closed high-level tool registry, blocked raw tools, Skill spine, worker/background reality, and control/worker validators.

### S4 — Self-Improving Skills, Past-Session Search, Hypothesis Racing, and Public Proof Canvas

Status: merged.

Purpose: learning/search/hypothesis/proof-viewer crates, public proof canvas, S4 validators, and proof-canvas browser coverage.

### S5 — Full Hardening, Public Release Candidate, and First End-to-End Mission

Status: merged.

Purpose: hardening/release/e2e crates, release-candidate docs, `proof-e2e`, and first public no-local release-candidate proof.

### S6 — Zero-Trust Cross-Cloud Gateway

Status: merged.

Purpose: zero-trust readiness/enforcement substrate for gateway transport/session/origin/backpressure contracts, adapter lifecycle/integrity/catalog contracts, Gemini worker readiness, semantic normalization, data-tainting, external authority boundaries, knowledge mirror, notification, repo worker, mobile/game QA, Remediator readiness, full S6 schema inventory, validator surface, and dedicated `proof-gateway` workflow.

S6 did not activate live external services, automate browser sessions/cookies, or mutate production/database systems.

## 4. Next Build Wave

## 4.1 S7 — First Real Activations

### Objective

Activate selected working lanes. S7 capabilities must perform real actions and produce proof artifacts.

### Branch

```text
build/s7-first-real-activations
```

### Recommended S7 package

S7 should activate:

1. Mobile Browser Game Preview and Playtest Activation
2. Notification Lane Activation
3. Knowledge Mirror Export Activation

Optional follow-on S7 tracks:

1. Gemini Worker Lane Activation
2. Document/Web Ingest Activation
3. Vercel Preview Adapter Activation
4. Supabase Read-Only Schema Inspect Activation
5. Telemetry Intake Activation
6. Git Worker Activation
7. Android Emulator QA Activation

### S7 acceptance

S7 passes when the activated lanes perform real actions:

1. mobile preview/playtest link exists when mobile preview is activated
2. mobile QA artifacts exist
3. notification dispatch works when a configured secret exists, with dry-run proof otherwise
4. knowledge mirror bundle and semantic snapshot are generated
5. redaction passes
6. all activation validators pass
7. relevant proof workflows are green
8. docs state exactly which lanes are active and which are not

## 4.2 S8 — StealthEye Cloud Remediator

### Objective

Activate Remediator Mode as a flagship proof-driven remediation system.

Tagline:

```text
Broken repo in. Reproduced failure, bounded patch, green proof, remediation report out.
```

### Branch

```text
build/s8-remediator-mode
```

### Crate

```text
crates/secloud-remediator
```

### Required S8 modules

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
quote
```

### Required S8 workflow

Add:

```text
.github/workflows/proof-remediator.yml
```

### S8 acceptance

S8 passes when:

1. Remediator intake validates
2. permission envelope validates
3. reality-map contracts validate
4. command discovery contracts validate
5. reproduction contracts validate
6. failure taxonomy validates
7. proof plan validates
8. remediation report validates
9. commercial quote/risk artifacts validate without activating billing
10. `proof-remediator` is green
11. docs and handoff artifacts are updated

## 5. Activation Rules

S6 readiness lanes are not active capabilities.

S7/S8 active lanes must do real work.

Examples:

```text
Mobile QA Readiness Layer = S6 readiness
Mobile Browser Game Preview and Playtest Activation = S7 active capability
Remediator Readiness = S6 readiness
Remediator Mode = S8 active capability
```

## 6. Stop Conditions

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

## 7. Immediate Next Action

Begin:

```text
S7 — First Real Activations
```

Target branch:

```text
build/s7-first-real-activations
```
