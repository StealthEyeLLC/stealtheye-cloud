# GitHub Evidence Packs

## Purpose

GitHub Evidence Packs normalize public-safe proof from pull requests, workflow runs, and branch state into agent-readable artifacts.

## S1 contracts

S1 introduces:

1. `GitHubEvidencePackV0`
2. `PrEvidenceCardV0`
3. `FailureCardV0`

## S1 posture

Evidence packs summarize public repo proof. They do not replace GitHub Actions, PR review, or StealthEye Seal. Seal remains the single checkpoint artifact.

## Future expansion

Later drops may import workflow artifacts, browser artifacts, and worker result summaries into a richer output shelf.
