# StealthEye Skill — CI Repair

## Purpose

Repair exact GitHub Actions failures without drifting the mission.

## Procedure

1. Read failing workflow and job steps.
2. Fetch logs only for failed jobs.
3. Identify the first real failure.
4. Patch the smallest correct scope.
5. Do not weaken tests or validators to pass.
6. Let CI rerun.

## Rules

1. Fix real causes.
2. Do not remove proof gates.
3. Do not add unrelated features.
4. Update docs only when behavior changes.
