# StealthEye Cloud Build Plan

## 0. Status

This document is the current build plan for `StealthEyeLLC/stealtheye-cloud`.

Current verified state:

```text
S0–S5 merged green
latest verified S5 merge: a190d7347569cc3a59d91678ddc5dec9d9e48c1b
```

S5 completed the first public no-local release-candidate spine with these gates green:

```text
proof-fast
proof-full
proof-browser
proof-e2e
proof-windows-targeted
```

The next locked build wave is:

```text
S6 — Zero-Trust Cross-Cloud Gateway
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

## 4. Next Build Wave

## 4.1 S6 — Zero-Trust Cross-Cloud Gateway

### Objective

Build the real enforcement and readiness substrate for cross-cloud operation.

S6 does not activate live external services. It proves the control plane, adapter lifecycle, auth boundaries, taint boundaries, normalization boundaries, backpressure, readiness lanes, and Remediator contracts.

### Branch

```text
build/s6-zero-trust-cross-cloud-gateway
```

### Crates

```text
crates/secloud-gateway
crates/secloud-mcp-adapters
crates/secloud-gemini-worker
crates/secloud-knowledge-mirror
crates/secloud-notifications
crates/secloud-external-auth
crates/secloud-gateway-security
crates/secloud-git-worker
crates/secloud-mobile-qa
crates/secloud-remediator
```

### Required S6 systems

1. Gateway transport/session/origin/auth policy
2. MCP adapter registry
3. Adapter type-state enforcement
4. MCP descriptor integrity / rug-pull defense
5. Adapter candidate catalog and risk scoring
6. Gemini worker readiness
7. Semantic normalization and prompt topology isolation
8. Data-tainting and indirect prompt injection isolation
9. Backpressure governor
10. External auth policy
11. Workflow injection guard
12. Knowledge mirror readiness and redaction
13. Notification readiness
14. Git worker readiness
15. Mobile QA readiness
16. Document/web ingest safety contracts
17. Production adapter contracts
18. Telemetry adapter contracts
19. Remediator readiness

### Required S6 validators

```text
secloud validate gateway
secloud validate gateway-transport
secloud validate mcp-adapters
secloud validate adapter-type-state
secloud validate adapter-integrity
secloud validate adapter-catalog
secloud validate gemini-worker
secloud validate normalization
secloud validate prompt-topology
secloud validate data-tainting
secloud validate injection-isolation
secloud validate backpressure
secloud validate external-auth
secloud validate workflow-security
secloud validate knowledge-mirror
secloud validate semantic-snapshot
secloud validate notifications
secloud validate git-worker
secloud validate mobile-qa
secloud validate game-qa
secloud validate document-ingest
secloud validate web-ingest
secloud validate production-adapters
secloud validate database-boundary
secloud validate telemetry-adapters
secloud validate telemetry-redaction
secloud validate remediator
secloud validate remediation-intake
secloud validate remediation-permissions
secloud validate remediation-reproduction
secloud validate remediation-failure-taxonomy
secloud validate remediation-proof-plan
secloud validate remediation-report
```

`secloud doctor` must include all S6 validators.

### Required S6 workflow

Add:

```text
.github/workflows/proof-gateway.yml
```

Required green gates:

```text
proof-fast
proof-full
proof-e2e
proof-gateway
proof-browser if triggered
proof-windows-targeted if triggered
```

### S6 acceptance

S6 passes when:

1. all S6 crates compile
2. all S6 schema files exist and are in packet inventory
3. all S6 validators pass
4. `proof-gateway` is green
5. `secloud doctor` includes S6 validators
6. docs are updated
7. Active/Relay/Seal/Next Action are updated
8. no external service is falsely claimed active
9. no forbidden session automation exists
10. no forbidden files exist

## 4.2 S7 — First Real Activations

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

## 4.3 S8 — StealthEye Cloud Remediator

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
5. deployment/production mutation
6. database mutation
7. legal/commercial commitment
8. platform auth boundary
9. unresolved high-impact ambiguity

Do not stop for routine continuation, docs updates, CI repair, validator wiring, state updates, or handoff generation.

## 7. Immediate Next Action

Create and prove the documentation/handoff update PR from:

```text
build/s6-s8-roadmap-docs-handoff
```

Then begin:

```text
S6 — Zero-Trust Cross-Cloud Gateway
```

Target branch:

```text
build/s6-zero-trust-cross-cloud-gateway
```
