# Next Action

Current state:

```text
S0–S9 merged green.
S9 — One-Drop Build Accelerator is complete.
S10 — Assistant Optimization Layer is selected for setup.
No S10 implementation has started.
```

S9 PR:

```text
#16
```

S9 merge SHA:

```text
a5540d1fe77a0752a6a32b086a66b7b4bbec33ec
```

Post-S9 cleanup PR:

```text
#17 merged
```

Current setup branch:

```text
build/s10-assistant-optimization-layer-setup
```

S10 implementation branch after setup merges:

```text
build/s10-assistant-optimization-layer
```

Immediate next action:

```text
Complete and merge the S10 setup docs PR, then start S10 implementation from current main using docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md.
```

S10 implementation must read these docs first:

```text
AGENTS.md
STEALTHEYE_DECISIONS.md
STEALTHEYE_ACTIVE.md
STEALTHEYE_RELAY.md
STEALTHEYE_RELAY.json
STEALTHEYE_SEAL.json
NEXT_ACTION.md
README.md
AGENT_REPO_MAP.md
docs/StealthEye_Cloud_Build_Plan.md
docs/HANDOFF_AND_CONTINUATION.md
docs/S9_ONE_DROP_BUILD_ACCELERATOR.md
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

Do not reopen:

```text
S6 architecture
S7 architecture
S8 architecture
S8 proof-driven remediation claim rule
S9 selected name and purpose
S9 merged proof truth
S10 selected name and purpose
```

No-weakening invariant:

```text
S10 improves the assistant/operator layer only. It must not weaken S9 one-drop mode, validators, schemas, proof gates, safety boundaries, or merge discipline.
```
