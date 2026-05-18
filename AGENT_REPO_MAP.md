# Agent Repo Map

## Purpose

This file gives ChatGPT and future agent workers a fast map of the StealthEye Cloud repository.

## Current phase

S0 — Foundation, Continuity, Packet Spine, and Cheap CI.

## Root files

- `AGENTS.md` — operating rules and agent instructions
- `llms.txt` — compact LLM index
- `llms-full.txt` — expanded LLM index
- `STEALTHEYE_ACTIVE.md` — current mission state
- `STEALTHEYE_DECISIONS.md` — canonical decisions
- `STEALTHEYE_RELAY.md` — human-readable handoff
- `STEALTHEYE_RELAY.json` — machine-readable handoff
- `STEALTHEYE_SEAL.json` — latest universal checkpoint
- `NEXT_ACTION.md` — one exact next action

## S0 implementation folders

- `crates/secloud-packets/` — packet type names and schema inventory
- `crates/secloud-relay/` — Relay validation
- `crates/secloud-seal/` — Seal validation
- `crates/secloud-cli/` — `secloud` CLI
- `schemas/` — public JSON Schema contracts
- `tests/fixtures/` — validation fixtures
- `.github/workflows/` — public proof workflows
- `.stealtheye/` — active state, memory, Relay, Seal, packet, Skill, and worker artifacts

## Proof workflows

- `proof-fast.yml` — fast Linux proof
- `proof-full.yml` — fuller Linux proof
- `proof-windows-targeted.yml` — manual/targeted Windows proof

## Forbidden files

Do not add `CLAUDE.md`, `.github/copilot-instructions.md`, `.cursorrules`, `soul.md`, generic `MEMORY.md`, or generic `rules.md`.
