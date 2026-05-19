# Agent Mode Packets

## Purpose

Agent Mode packets represent tasks that require user-started Agent Mode or equivalent interactive action mode.

## S3 contract

`AgentModeTaskPacketV0`

## Rules

1. `requires_user_start` must be true.
2. A task packet is a request, not proof that Agent Mode ran.
3. Imported results must be reviewed and proved before adoption.
4. StealthEye Cloud must not claim it silently launched Agent Mode.

## S3 status

S3 defines the packet and launch guide. It does not create hidden background execution.
