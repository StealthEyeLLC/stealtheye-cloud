# Future Phase Default Prompt

Use this prompt structure for future StealthEye Cloud phases after S9.

```text
Continue StealthEye Cloud from current main. Read AGENTS.md, STEALTHEYE_DECISIONS.md, STEALTHEYE_ACTIVE.md, STEALTHEYE_RELAY.md, STEALTHEYE_RELAY.json, STEALTHEYE_SEAL.json, NEXT_ACTION.md, docs/StealthEye_Cloud_Build_Plan.md, docs/HANDOFF_AND_CONTINUATION.md, and the current phase spec.

Begin the selected phase on the named build branch.

Use S9 one-drop mode:
- one mission approval
- one coherent repo mutation/drop
- one PR
- one proof wave
- inspect all failures before patching
- batched repairs only
- merge when green

Do not ask for routine confirmation between safe files, commits, PR creation, CI inspection, exact failure repair, docs/state updates, or merge when green.

Stop only for secrets, paid compute, private data exposure risk, account permission changes, production deployment, database mutation, browser-cookie/session-token automation, destructive irreversible action, or unresolved high-impact ambiguity.

Do not weaken validators, schemas, proof gates, safety boundaries, or merge discipline.
```
