# Batch Repair Policy

S9 requires repair work to inspect all failures before patching, then group related fixes into the smallest coherent repair commits.

## Rules

1. Run the proof wave.
2. Inspect all failing workflows/jobs/steps before patching.
3. Identify shared root causes.
4. Patch exact failures only.
5. Do not weaken tests, validators, schemas, docs checks, or CI requirements.
6. Rerun after the batch repair.
7. Avoid final state-only rerun commits unless truth requires them.

## Forbidden repair shortcuts

- deleting validators to pass CI
- removing schema inventory entries instead of adding missing schemas
- skipping required workflows
- replacing real proof with placeholder reports
- changing state docs to hide unresolved failures
