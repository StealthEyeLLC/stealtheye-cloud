# StealthEye Skill — Seal Generation

## Purpose

Generate or update StealthEye Seal artifacts only at necessary mission boundaries.

## Required outputs

1. `STEALTHEYE_SEAL.json`
2. `.stealtheye/seals/latest.json` when applicable

## Rules

1. Use only allowed Seal types: MISSION, APPROVAL, PROOF, BLOCKED, HANDOFF, FINAL.
2. Do not create separate receipt families.
3. Do not generate Seals for routine reads or trivial edits.
4. Include next action and boundary status.
