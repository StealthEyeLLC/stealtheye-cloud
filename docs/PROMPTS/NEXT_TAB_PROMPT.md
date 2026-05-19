# Next Tab Prompt — S9 One-Drop Build Accelerator

Paste this into a brand-new ChatGPT tab after the S9 setup PR merges green.

```text
Continue StealthEye Cloud from current main. Read AGENTS.md, STEALTHEYE_DECISIONS.md, STEALTHEYE_ACTIVE.md, STEALTHEYE_RELAY.md, STEALTHEYE_RELAY.json, STEALTHEYE_SEAL.json, NEXT_ACTION.md, docs/StealthEye_Cloud_Build_Plan.md, docs/S9_ONE_DROP_BUILD_ACCELERATOR.md, and docs/HANDOFF_AND_CONTINUATION.md.

S0–S8 are merged green. S9 is selected as One-Drop Build Accelerator.

Begin S9 — One-Drop Build Accelerator on branch build/s9-one-drop-build-accelerator.

S9’s purpose is to make every future phase/project operate as close as possible to:
one mission approval → one coherent repo mutation/drop → one PR → one proof wave → batched repairs → merge when green.

Use fast mode:
- prefer batch file creation/updates through Git tree/blob commits where available instead of many update_file calls
- create one coherent branch update before opening PR
- open one PR
- run one CI wave
- inspect all failures before patching
- batch repair commits
- do not add final state-only rerun commits unless required for truth
- merge when green

This is a mission-level approval. Do not ask for routine confirmation between safe files, commits, PR creation, CI inspection, exact failure repair, docs/state updates, or merge when green.

Stop only for secrets, paid compute, private data exposure risk, account permission changes, production deployment, database mutation, browser-cookie/session-token automation, destructive irreversible action, or unresolved high-impact ambiguity.

S9 must implement a real build-acceleration layer, not a placeholder. It must include crates/secloud-build-accelerator, S9 schemas, CLI validators, doctor integration, proof-build-accelerator workflow, docs, prompt artifacts, state consistency checks, no-cleanup-PR checks, batch repair checks, mission approval envelope checks, build velocity report checks, and future phase default contract checks.

Required systems:
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

Required workflow:
.github/workflows/proof-build-accelerator.yml

Do not reopen S6, S7, or S8 architecture. Do not weaken validators. Do not add placeholders. Do not fake automation. No Claude/Copilot/Cursor/soul files. No browser-cookie/session-token automation. No secrets, paid compute, production deployment, or database mutation without explicit approval.
```
