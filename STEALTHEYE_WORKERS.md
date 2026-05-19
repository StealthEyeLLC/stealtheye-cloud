# StealthEye Workers

## Purpose

This file records real background and worker surfaces for StealthEye Cloud.

## Real surfaces

1. active ChatGPT tab
2. ChatGPT Skills
3. ChatGPT Tasks
4. Codex when user-launched/configured
5. GitHub Actions
6. Deep Research when user-started
7. Agent Mode when user-started
8. Apps/connectors when available
9. Relay/Seal handoffs

## Worker packets

1. `CodexTaskPacketV0`
2. `CodexResultImportV0`
3. `CodexUsageSnapshotV0`
4. `DeepResearchTaskPacketV0`
5. `ResearchResultImportV0`
6. `AgentModeTaskPacketV0`
7. `FeatureAvailabilityCheckV0`

## Blocked hidden-autonomy claims

StealthEye Cloud must not claim that ChatGPT can secretly create tabs, silently launch Deep Research, silently launch Agent Mode, or spend paid compute without approval.

## S3 status

S3 establishes representation and validation contracts. It does not create hidden background execution.
