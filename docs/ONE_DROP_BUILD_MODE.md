# One-Drop Build Mode

S9 makes StealthEye Cloud default to one mission approval -> one coherent repo mutation/drop -> one PR -> one proof wave -> batched repairs -> merge when green.

## Contract

A valid one-drop build includes implementation, schemas, validators, docs, workflow proof, state updates, final report, and merge-aware continuation text.

## Non-negotiable strength rule

One-drop mode reduces avoidable process friction only. It does not weaken validators, schemas, safety boundaries, CI gates, branch discipline, or merge requirements.

## Required sequence

1. Capture mission approval envelope.
2. Snapshot current repo/state truth.
3. Create one coherent branch mutation batch.
4. Open one PR.
5. Run one proof wave.
6. Inspect all failures before patching.
7. Apply batched repairs for exact failures only.
8. Merge when green.
9. Resolve merge-aware state without a cleanup PR whenever the merge metadata is available.
