# Mission Executor

## Purpose

The Mission Executor is the S1 public-safe execution spine for StealthEye Cloud.

It represents the path from:

```text
Mission approval -> dispatch packet -> validated file bundle -> proof workflow -> evidence summary -> operator state
```

## S1 scope

S1 does not provide secret-bearing or destructive execution.

S1 establishes:

1. mission dispatch schema
2. authority queue schema
3. output shelf schema
4. file bundle validation rules
5. proof workflow entrypoint
6. operator state schema
7. autonomy status schema
8. public-safe evidence packets

## Current executable pieces

1. `crates/secloud-mission`
2. `crates/secloud-file-bundle`
3. `crates/secloud-ci`
4. `.github/workflows/mission-executor.yml`

## Boundaries

The workflow cannot read secrets, spend money, deploy, release, or perform destructive actions.

S1 is the first step toward future bundled drops; it does not yet replace GitHub PR review and CI proof.
