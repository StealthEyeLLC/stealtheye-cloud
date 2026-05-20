# Prompt Compression

## Purpose

Prompt Compression makes continuation prompts shorter, stronger, and less lossy.

## Required packet families

```text
PromptCompressionProfileV0
MissionPromptTemplateV0
ContinuationPromptTemplateV0
RepairPromptTemplateV0
PostMergePromptTemplateV0
PromptLossCheckV0
PromptStrengthScoreV0
PromptRedundancyFindingV0
PromptCarryoverDigestV0
PromptExecutableChecklistV0
```

## Required behavior

Every continuation prompt must preserve:

1. current state
2. approval envelope
3. exact next action
4. branch and PR state
5. proof requirements
6. do-not-reopen decisions
7. true boundaries
8. no-weakening invariants
9. required scan files
10. implementation branch name

## Acceptance

S10 implementation must add `secloud validate prompt-compression`, prompt loss fixtures, prompt strength fixtures, and a prompt-compression proof artifact.
