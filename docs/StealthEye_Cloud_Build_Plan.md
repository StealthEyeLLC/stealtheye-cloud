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

S9 is selected as the next mission:

```text
S9 — One-Drop Build Accelerator
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

A lane may be called `readiness` only when it builds real contracts, schemas, validators, docs, and proof gates.

A lane may be called `active` only when it performs the real action and CI/proof artifacts validate it.

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

S9 must add a next-tab prompt artifact so the user has a single copy/paste source for future phases.

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

## 4. Active Build Wave

### S9 — One-Drop Build Accelerator

Status: selected, not implemented.

Target branch:

```text
build/s9-one-drop-build-accelerator
```

Purpose: make every future phase/project operate as close as possible to one mission approval, one coherent repo mutation/drop, one PR, one proof wave, batched repairs, and merge when green.

Recommended crate:

```text
crates/secloud-build-accelerator
```

Required S9 systems:

1. One-drop manifest and compiler contracts
2. Mission approval envelope
3. Approval compression policy
4. No-midpoint-ask policy
5. Tool-call bundling policy
6. Repo mutation batch model
7. Git tree/batch mutation plan model
8. Batch CI failure triage and repair plan
9. Merge-aware handoff model
10. Pre-merge/post-merge truth template
11. No-cleanup-PR gate
12. State consistency oracle
13. Single source of phase truth
14. State file generation model
15. Branch lifecycle automaton
16. PR lifecycle automaton
17. Touched-surface proof selector
18. Required checks manifest
19. Workflow path-filter simulator
20. CI wave counter
21. Repair cause memory
22. Validator registration generator/checker
23. Schema inventory auto-check
24. Workspace/crate registration check
25. Obsolete text scanner
26. Duplicate/conflicting doc detector
27. Merge readiness red-team checklist
28. Build velocity report
29. Confirmation friction ledger
30. Human availability profile for low-attention workdays
31. Tool fallback ladder
32. Existing branch/PR reuse policy
33. Partial-completion recovery
34. No silent downgrade detector
35. Capability activation ledger
36. Future phase default contract
37. Next-tab prompt artifact
38. Prompt pack for future tabs

Required S9 validators:

```text
secloud validate one-drop
secloud validate mission-approval
secloud validate approval-compression
secloud validate no-midpoint-ask
secloud validate tool-call-bundling
secloud validate repo-mutation-batch
secloud validate batch-repair
secloud validate merge-aware-handoff
secloud validate no-cleanup-pr
secloud validate state-consistency
secloud validate phase-truth
secloud validate branch-lifecycle
secloud validate pr-lifecycle
secloud validate proof-selector
secloud validate required-checks
secloud validate workflow-path-filter
secloud validate ci-wave-counter
secloud validate repair-cause-memory
secloud validate validator-registration
secloud validate schema-inventory
secloud validate workspace-registration
secloud validate obsolete-text
secloud validate doc-conflicts
secloud validate merge-readiness-red-team
secloud validate build-velocity
secloud validate confirmation-friction
secloud validate human-availability
secloud validate tool-fallback
secloud validate existing-work-reuse
secloud validate partial-completion-recovery
secloud validate no-silent-downgrade
secloud validate capability-activation-ledger
secloud validate future-phase-contract
secloud validate next-tab-prompt
```

Required S9 workflow:

```text
.github/workflows/proof-build-accelerator.yml
```

S9 acceptance:

1. `crates/secloud-build-accelerator` exists and compiles.
2. S9 schemas are added to the schema inventory.
3. S9 validators are callable through `secloud-cli`.
4. `secloud doctor` includes S9 validators.
5. `proof-build-accelerator` is green.
6. State consistency catches README/build-plan/Active/Relay/Seal/Next Action conflicts.
7. No-cleanup-PR gate can detect stale post-merge language.
8. Build velocity report and confirmation friction ledger are emitted as artifacts or validated fixtures.
9. `docs/PROMPTS/NEXT_TAB_PROMPT.md` exists for the next phase.
10. Future phase default contract is documented and validated.

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
9. browser-cookie/session-token automation
10. unresolved high-impact ambiguity

Do not stop for routine continuation, docs updates, CI repair, validator wiring, state updates, PR creation, or merge when green if the mission approval envelope covers it.

## 6. Immediate Next Action

Begin:

```text
S9 — One-Drop Build Accelerator
```

Target branch:

```text
build/s9-one-drop-build-accelerator
```
