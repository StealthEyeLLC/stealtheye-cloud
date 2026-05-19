# Self-Improving Skills

## Purpose

Self-Improving Skills convert observed execution patterns into proposed Skill improvements without model training, hidden authority, or private-data leakage.

## S4 scope

S4 establishes representation and validation contracts for:

1. `FeedbackSignalV0`
2. `PatternCandidateV0`
3. `SkillCandidateV0`
4. `SkillPromotionDecisionV0`
5. `SkillTemplateIndexV0`
6. `TemplateToSkillCompilerV0`

## Executable crate

```text
crates/secloud-learning
```

## Promotion rules

A Skill candidate requires:

1. evidence
2. preserved human authority
3. no hidden background claims
4. no private data in public Skill files
5. rollback or rejection path

## Boundary

S4 does not train models, self-modify silently, or promote Skills without explicit promotion decisions. It creates a governed proposal and validation loop.
