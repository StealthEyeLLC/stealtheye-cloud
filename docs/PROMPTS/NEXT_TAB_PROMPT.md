# Next Tab Prompt — S10 Setup Continuation

Paste this into a brand-new ChatGPT tab only if the current tab saturates before the S10 setup docs PR merges.

```text
Continue StealthEye Cloud from current repo state. Read AGENTS.md, STEALTHEYE_DECISIONS.md, STEALTHEYE_ACTIVE.md, STEALTHEYE_RELAY.md, STEALTHEYE_RELAY.json, STEALTHEYE_SEAL.json, NEXT_ACTION.md, README.md, AGENT_REPO_MAP.md, llms.txt, llms-full.txt, docs/StealthEye_Cloud_Build_Plan.md, docs/HANDOFF_AND_CONTINUATION.md, docs/S9_ONE_DROP_BUILD_ACCELERATOR.md, docs/S10_ASSISTANT_OPTIMIZATION_LAYER.md, and docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md.

S0–S9 are merged green.

S9 — One-Drop Build Accelerator is complete.

S9 PR #16 merged.

S9 merge SHA:
a5540d1fe77a0752a6a32b086a66b7b4bbec33ec

Post-S9 cleanup PR #17 merged.

S10 is selected:
S10 — Assistant Optimization Layer

Current setup branch:
build/s10-assistant-optimization-layer-setup

No S10 implementation has started.

Current task: finish, prove, and merge the S10 setup docs PR only.

Do not implement S10 inside the setup PR. Do not add crates, schemas, validators, proof-assistant-optimizer workflow, or runtime feature claims in this setup branch.

After setup merges green, start a new implementation tab and use:
docs/PROMPTS/S10_IMPLEMENTATION_PROMPT.md

Implementation branch after setup merges:
build/s10-assistant-optimization-layer

Use S9 one-drop mode for S10 implementation:
- one mission approval
- one coherent repo mutation/drop
- one PR
- one proof wave
- inspect all failures before patching
- batch exact repairs only
- merge when green

Stop only for true boundaries.

No-weakening invariant: S10 improves the assistant/operator layer only. It must not weaken S9 one-drop mode, validators, schemas, proof gates, safety boundaries, or merge discipline.
```
