# S9 — One-Drop Build Accelerator

## Objective

S9 exists to make every future StealthEye Cloud phase or project operate as close as possible to:

```text
one mission approval
→ one coherent repo mutation/drop
→ one PR
→ one proof wave
→ batched repairs
→ merge when green
```

S9 is a build-operating-system upgrade for speed, lower confirmation friction, cleaner handoffs, fewer cleanup PRs, and stricter proof preservation.

## Branch

```text
build/s9-one-drop-build-accelerator
```

## Primary crate

```text
crates/secloud-build-accelerator
```

## Core rule

Routine work covered by a mission approval must continue without midpoint confirmation requests.

Stop only for true boundaries:

1. secrets
2. paid compute
3. account permission changes
4. private data exposure risk
5. production deployment without explicit approval
6. database mutation without explicit approval
7. browser-cookie/session-token automation
8. destructive irreversible action
9. unresolved high-impact ambiguity

## No-weakening invariant

S9 reduces avoidable process friction only. It does not weaken validators, schemas, proof gates, safety boundaries, CI coverage, or merge discipline.

## Implemented systems

1. One-drop manifest and compiler contracts
2. Mission approval envelope
3. Approval compression policy
4. No-midpoint-ask policy
5. Tool-call bundling policy
6. Repo mutation batch model
7. Batch CI failure triage and repair policy
8. Merge-aware handoff and no-cleanup-PR gate
9. State consistency oracle
10. Phase truth and lifecycle automata
11. Proof selection and CI optimization contracts
12. Registration and downgrade guards
13. Human attention and partial recovery rules
14. Future phase default contract
15. Build velocity and confirmation friction artifacts

## Required validators

The following are callable through `secloud validate` and included in `secloud doctor`:

```text
one-drop
mission-approval
approval-compression
no-midpoint-ask
tool-call-bundling
repo-mutation-batch
batch-repair
merge-aware-handoff
no-cleanup-pr
state-consistency
phase-truth
branch-lifecycle
pr-lifecycle
proof-selector
required-checks
workflow-path-filter
ci-wave-counter
repair-cause-memory
validator-registration
schema-inventory
workspace-registration
obsolete-text
doc-conflicts
merge-readiness-red-team
build-velocity
confirmation-friction
human-availability
tool-fallback
existing-work-reuse
partial-completion-recovery
no-silent-downgrade
capability-activation-ledger
future-phase-contract
next-tab-prompt
```

## Required workflow

```text
.github/workflows/proof-build-accelerator.yml
```

## Required docs

```text
docs/ONE_DROP_BUILD_MODE.md
docs/MISSION_APPROVAL_ENVELOPE.md
docs/BATCH_REPAIR_POLICY.md
docs/MERGE_AWARE_HANDOFF.md
docs/PHASE_TEMPLATE_SYSTEM.md
docs/PROMPTS/NEXT_TAB_PROMPT.md
docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md
docs/S9_FINAL_REPORT.md
```

## Acceptance

S9 is complete only when:

1. `crates/secloud-build-accelerator` exists and compiles.
2. all S9 schemas are registered in the inventory.
3. all S9 validators are callable through `secloud-cli`.
4. `secloud doctor` includes all S9 validators.
5. `proof-build-accelerator` is green.
6. state consistency catches README/build-plan/Active/Relay/Seal/Next Action conflicts.
7. no-cleanup-PR gate detects stale post-merge language.
8. build velocity and confirmation friction artifacts validate.
9. next-tab and future phase prompt artifacts exist.
10. future phase default contract validates.
11. docs and handoff state point forward without requiring a cleanup PR.

## Non-goals

S9 does not bypass platform/tool permission prompts. It reduces avoidable assistant/process confirmations and makes each accepted action larger.

S9 does not authorize secrets, paid compute, production deployment, database mutation, browser-cookie/session-token automation, private data exposure, or validator weakening.
