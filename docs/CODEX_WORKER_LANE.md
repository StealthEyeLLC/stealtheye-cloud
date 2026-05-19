# Codex Worker Lane

## Purpose

The Codex Worker Lane represents bounded coding-worker tasks while preserving ChatGPT as planner, reviewer, and final integrator.

## S3 contracts

1. `CodexTaskPacketV0`
2. `CodexResultImportV0`
3. `CodexUsageSnapshotV0`

## Rules

1. Codex is a worker lane, not the main brain.
2. Codex tasks must be bounded by mission scope.
3. Results must be imported and reviewed before adoption.
4. Usage should be tracked when available.
5. Paid compute or premium usage requires explicit approval when it matters.

## S3 status

S3 defines packet contracts and Skill instructions. It does not silently launch Codex.
