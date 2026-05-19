# Next Tab Prompt — S9 One-Drop Build Accelerator Continuation

Paste this into a brand-new ChatGPT tab only if the current tab saturates before S9 is merged.

```text
Continue StealthEye Cloud from current repo state. Read AGENTS.md, STEALTHEYE_DECISIONS.md, STEALTHEYE_ACTIVE.md, STEALTHEYE_RELAY.md, STEALTHEYE_RELAY.json, STEALTHEYE_SEAL.json, NEXT_ACTION.md, docs/StealthEye_Cloud_Build_Plan.md, docs/S9_ONE_DROP_BUILD_ACCELERATOR.md, and docs/HANDOFF_AND_CONTINUATION.md.

S0–S8 are merged green. Continue S9 — One-Drop Build Accelerator from branch build/s9-one-drop-build-accelerator or its active PR.

S9’s purpose is to make every future phase/project operate as close as possible to:
one mission approval → one coherent repo mutation/drop → one PR → one proof wave → batched repairs → merge when green.

Use fast mode:
- prefer batch file creation/updates through Git tree/blob commits where available instead of many update_file calls
- preserve one coherent branch update before PR whenever possible
- keep one PR
- run the proof wave
- inspect all failures before patching
- batch repair commits
- do not add final state-only rerun commits unless required for truth
- merge when green

This is a mission-level approval. Do not ask for routine confirmation between safe files, commits, PR creation, CI inspection, exact failure repair, docs/state updates, or merge when green.

Stop only for secrets, paid compute, private data exposure risk, account permission changes, production deployment, database mutation, browser-cookie/session-token automation, destructive irreversible action, or unresolved high-impact ambiguity.

No-weakening invariant: fast mode reduces avoidable process friction only. It must not weaken validators, schemas, proof gates, safety boundaries, or merge discipline.

If S9 is already merged green, continue from current main and define S10 or the next selected mission using S9 one-drop mode.
```
