# StealthEye Cloud

StealthEye Cloud is the no-local, ChatGPT-native GitHub/browser execution-body agent.

It is optimized for one active ChatGPT tab until saturation, public free CI proof, browser body proof, mission-level approvals, StealthEye Relay handoffs, and StealthEye Seal checkpoints.

## Current build state

Current status: **S0–S9 merged green. S10 — Assistant Optimization Layer is selected for setup. No S10 implementation has started.**

Latest completed mission:

```text
S9 — One-Drop Build Accelerator
```

Next selected mission:

```text
S10 — Assistant Optimization Layer
```

S10 setup branch:

```text
build/s10-assistant-optimization-layer-setup
```

S10 implementation branch after setup merges:

```text
build/s10-assistant-optimization-layer
```

S10 purpose:

```text
Make ChatGPT/StealthEye Cloud faster, less needy, more reliable across tabs, better at context loading, better at tool use, better at recovery, better at proof reasoning, and better at minimizing the operator's attention burden.
```

S9 PR:

```text
#16
```

S9 merge SHA:

```text
a5540d1fe77a0752a6a32b086a66b7b4bbec33ec
```

S9 green before merge:

```text
proof-fast
proof-full
proof-browser
proof-mobile
proof-e2e
proof-gateway
proof-activations
proof-remediator
proof-build-accelerator
proof-windows-targeted
```

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
S8 — StealthEye Cloud Remediator
S9 — One-Drop Build Accelerator
```

Selected next spine entry:

```text
S10 — Assistant Optimization Layer
```

S9 makes every future phase/project operate as close as possible to:

```text
one mission approval
→ one coherent repo mutation/drop
→ one PR
→ one proof wave
→ batched repairs
→ merge when green
```

S10 makes the assistant/operator layer use that rail better:

```text
repo truth first
context load compiler
mission intake optimizer
tool use planner
tool fallback ladder
low-attention workday mode
handoff quality gate
prompt compression
scope drift guard
proof awareness
repair intelligence
capability reality map
read-only verification
assistant self-audit
build cockpit card
trace digest
MCP-aware operator policy
```

S10 does not reduce proof strength. It must preserve S9 one-drop mode, validators, schema coverage, workflow proof, safety boundaries, and green-before-merge discipline.

## Recent completion truth

```text
S6 PR #8 merge SHA: dcaf60dce2b466178c3cff1ee4545d06f3e5075f
Post-S6 cleanup PR #9 merge SHA: a5e6eccc37067cf264fd8859c69fc412da855bb8
S7 PR #11 merge SHA: d814507740b1ab9a58dd5a2e9a4e079e21bf1d78
S8 PR #13 merge SHA: 12081b4d311844b62aecafb5ff045414e94a4a7c
Post-S8 cleanup PR #14 merge SHA: e45b0e75fc9b1f8a9e1ed09db90d69037fe9c11d
S9 PR #16 merge SHA: a5540d1fe77a0752a6a32b086a66b7b4bbec33ec
Post-S9 cleanup PR #17 merged
```

## Start here

1. Read `AGENTS.md`.
2. Read `STEALTHEYE_DECISIONS.md`.
3. Read `STEALTHEYE_ACTIVE.md`.
4. Read `STEALTHEYE_RELAY.md` and `STEALTHEYE_SEAL.json` when resuming.
5. Perform `NEXT_ACTION.md` unless a boundary is required.

## Next action

```text
Complete the S10 setup docs PR, then use docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md in a new tab to implement S10.
```

## Canonical docs

- `docs/StealthEye_Cloud_Final_Technical_Specification.md`
- `docs/StealthEye_Cloud_Build_Plan.md`
- `docs/S8_STEALTHEYE_CLOUD_REMEDIATOR.md`
- `docs/S9_ONE_DROP_BUILD_ACCELERATOR.md`
- `docs/S10_ASSISTANT_OPTIMIZATION_LAYER.md`
- `docs/ASSISTANT_OPERATING_PROFILE.md`
- `docs/MISSION_INTAKE_OPTIMIZER.md`
- `docs/CONTEXT_LOAD_POLICY.md`
- `docs/REPO_TRUTH_FIRST_POLICY.md`
- `docs/LOW_ATTENTION_WORKDAY_MODE.md`
- `docs/HANDOFF_QUALITY_GATE.md`
- `docs/TOOL_USE_PLANNER.md`
- `docs/TOOL_FALLBACK_POLICY.md`
- `docs/PROMPT_COMPRESSION.md`
- `docs/SCOPE_DRIFT_GUARD.md`
- `docs/PROOF_AWARENESS_LAYER.md`
- `docs/REPAIR_INTELLIGENCE_LAYER.md`
- `docs/READ_ONLY_VERIFICATION_MODE.md`
- `docs/ASSISTANT_SELF_AUDIT.md`
- `docs/CAPABILITY_REALITY_MAP.md`
- `docs/BUILD_COCKPIT_CARD.md`
- `docs/AGENT_OBSERVABILITY_TRACE_DIGEST.md`
- `docs/MCP_AWARE_OPERATOR_POLICY.md`
- `docs/ONE_DROP_BUILD_MODE.md`
- `docs/MISSION_APPROVAL_ENVELOPE.md`
- `docs/BATCH_REPAIR_POLICY.md`
- `docs/MERGE_AWARE_HANDOFF.md`
- `docs/PHASE_TEMPLATE_SYSTEM.md`
- `docs/HANDOFF_AND_CONTINUATION.md`
- `docs/PROMPTS/NEXT_TAB_PROMPT.md`
- `docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md`
- `docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md`

## Current proof workflows

- `proof-fast.yml`
- `proof-full.yml`
- `proof-browser.yml`
- `proof-mobile.yml`
- `proof-e2e.yml`
- `proof-gateway.yml`
- `proof-activations.yml`
- `proof-remediator.yml`
- `proof-build-accelerator.yml`
- `proof-windows-targeted.yml`

S10 implementation must add:

```text
.github/workflows/proof-assistant-optimizer.yml
```

## Canonical operating model

```text
one active ChatGPT tab until saturation
Relay + Seal + Active + Next Action at handoff
mission-level approval, not per-file approval
GitHub branch + PR per coherent drop
CI proof before merge
repair exact failures only
merge only when green
```

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
