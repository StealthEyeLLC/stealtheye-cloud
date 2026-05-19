# StealthEye Skill — Codex Worker

## Purpose

Create and import Codex worker packets while preserving ChatGPT as the primary planner and reviewer.

## Procedure

1. Use Codex only for bounded coding tasks.
2. Represent work through `CodexTaskPacketV0`.
3. Import results through `CodexResultImportV0`.
4. Track usage through `CodexUsageSnapshotV0`.
5. Review results before adoption.

## Rules

1. Codex is a worker lane, not the main brain.
2. Do not use Codex for trivial edits.
3. Do not spend paid compute without approval.
4. Do not import worker output as truth without proof.
