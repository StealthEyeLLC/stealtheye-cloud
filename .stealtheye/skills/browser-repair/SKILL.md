# StealthEye Skill — Browser Repair

## Purpose

Repair browser proof failures from Playwright and browser artifacts.

## Procedure

1. Read the failing browser workflow job.
2. Inspect console, network, DOM, screenshot, and summary artifacts when available.
3. Identify the first real browser failure.
4. Patch only the failing browser route, artifact path, config, or assertion.
5. Keep deterministic replay proof intact.

## Rules

1. Do not disable browser proof to pass.
2. Do not remove artifact validation.
3. Do not introduce paid browser cloud by default.
4. Preserve no-local execution.
