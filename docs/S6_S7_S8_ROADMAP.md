# S6/S7/S8 Roadmap

## Status

S0 through S5 are merged green. S5 completed the first public no-local release-candidate spine.

Next locked wave:

```text
S6 — Zero-Trust Cross-Cloud Gateway
S7 — First Real Activations
S8 — StealthEye Cloud Remediator
```

## No-placeholder rule

A readiness lane is real only when it ships contracts, schemas, validators, docs, and proof gates.

An active lane is real only when it performs the actual external or runtime action and proof artifacts validate it.

Do not claim activated capability from readiness contracts.

## S6 purpose

S6 builds the enforcement and readiness substrate:

1. gateway transport/session/origin/auth policy
2. MCP adapter registry
3. adapter type-state enforcement
4. adapter descriptor integrity and rug-pull defense
5. Gemini worker readiness
6. semantic normalization and prompt topology isolation
7. data-tainting and indirect prompt injection isolation
8. backpressure governor
9. external auth policy
10. workflow injection guard
11. knowledge mirror readiness and redaction
12. notification readiness
13. git worker readiness
14. mobile QA readiness
15. document/web ingest safety contracts
16. production adapter contracts
17. telemetry adapter contracts
18. Remediator readiness

## S7 purpose

S7 activates selected lanes that perform real work.

Recommended first activations:

1. Mobile Browser Game Preview and Playtest Activation
2. Notification Lane Activation
3. Knowledge Mirror Export Activation

Later activations:

1. Gemini Worker Lane Activation
2. Document/Web Ingest Activation
3. Vercel Preview Adapter Activation
4. Supabase Read-Only Schema Inspect Activation
5. Telemetry Intake Activation
6. Git Worker Activation
7. Android Emulator QA Activation

## S8 purpose

S8 activates StealthEye Cloud Remediator.

Tagline:

```text
Broken repo in. Reproduced failure, bounded patch, green proof, remediation report out.
```

S8 must reproduce failures before claiming remediation. Diagnosis without reproduction is allowed, but it is not a completed remediation.

## Build order

```text
1. Docs/handoff update PR
2. S6 Zero-Trust Cross-Cloud Gateway
3. S7 First Real Activations
4. S8 StealthEye Cloud Remediator
```

## Current next action

Start S6 after this documentation/handoff PR merges green.
