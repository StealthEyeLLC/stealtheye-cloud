# Merge-Aware Handoff

S9 adds a no-cleanup-PR handoff model. The PR branch must carry enough post-merge truth structure that the next tab can continue without requiring a separate stale-state cleanup PR.

## no-cleanup-PR target

The no-cleanup-PR target is zero stale setup language after the PR branch is proven. The state files should describe the active implementation branch before merge and a deterministic post-merge next action after merge.

## Merge metadata rule

The exact merge SHA cannot be known before merge. S9 state therefore records a merge metadata resolution field instead of pretending to know the future SHA.

## Required state checks

1. Active, Relay, Seal, Next Action, README, and build plan agree on the phase.
2. Setup-specific language is absent from implementation state.
3. The next action survives merge.
4. If a final merge SHA must be recorded, that recording is allowed only when truth requires it.
