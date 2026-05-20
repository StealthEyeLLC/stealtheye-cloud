# StealthEye Cloud Build Plan

## Status

Current verified state:

```text
S0–S9 merged green
S9 PR #16 merged
S9 merge SHA: a5540d1fe77a0752a6a32b086a66b7b4bbec33ec
S9 — One-Drop Build Accelerator: complete
Post-S9 cleanup PR #17 merged
S10 — Assistant Optimization Layer: selected for setup
No S10 implementation has started
```

Current setup branch:

```text
build/s10-assistant-optimization-layer-setup
```

Next implementation branch after setup merges:

```text
build/s10-assistant-optimization-layer
```

Next action:

```text
Complete S10 setup docs, merge the setup PR green, then use docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md in a new tab to implement S10.
```

## Completed build spine

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

## Selected next build spine entry

```text
S10 — Assistant Optimization Layer
```

## S9 completion truth

S9 merged through PR #16 at `a5540d1fe77a0752a6a32b086a66b7b4bbec33ec`.

S9 implementation branch:

```text
build/s9-one-drop-build-accelerator
```

S9 crate:

```text
crates/secloud-build-accelerator
```

S9 workflow:

```text
.github/workflows/proof-build-accelerator.yml
```

Green before merge:

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

## S9 doctrine now active

Future selected missions should use S9 one-drop mode: one mission approval, one coherent drop, one PR, one proof wave, batched exact repairs, and merge when green.

Fast mode reduces avoidable process friction only. It must not weaken validators, schemas, proof gates, safety boundaries, or merge discipline.

## S10 purpose

S10 is an operator optimization layer. It makes the ChatGPT/StealthEye Cloud active tab better at using the S9 one-drop rail.

S10 must make the assistant:

1. faster at starting missions
2. less needy during routine approved work
3. more reliable across tabs
4. better at reading current repo truth
5. better at loading only the right context
6. better at planning tool use and fallback
7. better at recovery after routine tool or CI failures
8. better at proof reasoning
9. better at minimizing operator attention burden
10. better at avoiding overclaims, stale state, and scope drift

## S10 core implementation shape

Recommended crate:

```text
crates/secloud-assistant-optimizer
```

Required workflow:

```text
.github/workflows/proof-assistant-optimizer.yml
```

Required proof script family:

```text
scripts/s10-assistant-optimizer-proof.mjs
scripts/check-s10-assistant-optimizer-artifacts.mjs
```

Required proof artifact directory:

```text
.stealtheye/assistant-optimizer/
```

## S10 core docs

```text
docs/S10_ASSISTANT_OPTIMIZATION_LAYER.md
docs/ASSISTANT_OPERATING_PROFILE.md
docs/MISSION_INTAKE_OPTIMIZER.md
docs/CONTEXT_LOAD_POLICY.md
docs/REPO_TRUTH_FIRST_POLICY.md
docs/LOW_ATTENTION_WORKDAY_MODE.md
docs/HANDOFF_QUALITY_GATE.md
docs/TOOL_USE_PLANNER.md
docs/TOOL_FALLBACK_POLICY.md
docs/PROMPT_COMPRESSION.md
docs/SCOPE_DRIFT_GUARD.md
docs/PROOF_AWARENESS_LAYER.md
docs/REPAIR_INTELLIGENCE_LAYER.md
docs/READ_ONLY_VERIFICATION_MODE.md
docs/ASSISTANT_SELF_AUDIT.md
docs/CAPABILITY_REALITY_MAP.md
docs/BUILD_COCKPIT_CARD.md
docs/AGENT_OBSERVABILITY_TRACE_DIGEST.md
docs/MCP_AWARE_OPERATOR_POLICY.md
```

## S10 required validator targets

S10 implementation must add these public CLI validator targets and include them in `secloud doctor`:

```text
assistant-optimizer
assistant-operating-profile
mission-intake
context-load-policy
repo-truth-first
tool-use-policy
tool-fallback-policy
low-attention-mode
progress-update-policy
handoff-quality
prompt-compression
scope-drift-guard
decision-carryover
proof-awareness
repair-intelligence
capability-map
read-only-verification
assistant-self-audit
operator-mistake-detector
build-cockpit
trace-digest
mcp-aware-operator-policy
output-mode-selector
future-mission-recommender
```

## S10 priority packet families

```text
AssistantOperatingProfileV0
MissionIntakePacketV0
ContextLoadPlanV0
RepoTruthFirstPolicyV0
ToolUsePlanV0
ToolFallbackPolicyV0
LowAttentionWorkdayModeV0
HandoffQualityReportV0
PromptCompressionProfileV0
ScopeDriftDetectorV0
ProofNeedClassifierV0
RepairTriagePlanV0
CapabilityRealityMapV0
ReadOnlyVerificationModeV0
AssistantSelfAuditV0
BuildCockpitCardV0
AssistantTraceDigestV0
McpAwareOperatorPolicyV0
```

## S10 acceptance criteria

S10 is complete only when:

1. `crates/secloud-assistant-optimizer` exists and compiles.
2. S10 schemas are materialized and registered in the global packet inventory.
3. all S10 validator targets are callable through `secloud-cli`.
4. `secloud doctor` includes all S10 validator targets.
5. `.github/workflows/proof-assistant-optimizer.yml` is green.
6. S10 proof artifacts validate.
7. S10 docs and prompt artifacts exist.
8. S10 catches stale handoff/status drift using fixtures.
9. S10 proves read-only verification mode with valid and invalid fixtures.
10. S10 preserves S9 one-drop mode.
11. no validators, schemas, workflows, proof gates, safety boundaries, or merge discipline are weakened.
12. no hidden autonomy claims, fake background work, browser-cookie/session-token automation, paid compute, secrets, production deployment, or database mutation are introduced.

## Immediate next action

```text
Merge the S10 setup docs PR green, then start S10 implementation from current main using docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md.
```