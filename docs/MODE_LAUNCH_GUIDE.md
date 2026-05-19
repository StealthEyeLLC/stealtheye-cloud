# Mode Launch Guide

## Purpose

This guide explains which ChatGPT/OpenAI surfaces require user initiation and how StealthEye Cloud represents them.

## User-started modes

1. Deep Research must be started by the user when needed.
2. Agent Mode must be started by the user when needed.
3. Codex worker tasks must be launched/configured through available user/tool surfaces.
4. ChatGPT Tasks can schedule reminders or heartbeat prompts.

## Repo representation

1. `DeepResearchTaskPacketV0` represents a research task request.
2. `ResearchResultImportV0` imports research results.
3. `AgentModeTaskPacketV0` represents an Agent Mode task request.
4. `FeatureAvailabilityCheckV0` records availability of plan/platform features.

## Rule

Represent mode requests clearly. Do not pretend they ran unless the user actually started them and results were imported.
