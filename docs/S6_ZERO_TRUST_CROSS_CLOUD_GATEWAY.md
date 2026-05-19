# S6 — Zero-Trust Cross-Cloud Gateway

## Objective

S6 builds the real enforcement and readiness substrate for future cross-cloud operation.

S6 does not activate live external services. It proves the contracts, validators, boundaries, and proof gates required before any external adapter can become active.

## Branch

```text
build/s6-zero-trust-cross-cloud-gateway
```

## Core crates

```text
crates/secloud-gateway
crates/secloud-mcp-adapters
crates/secloud-gemini-worker
crates/secloud-knowledge-mirror
crates/secloud-notifications
crates/secloud-external-auth
crates/secloud-gateway-security
crates/secloud-git-worker
crates/secloud-mobile-qa
crates/secloud-remediator
```

## Hard invariants

1. Adapter lifecycle boundary: contract-only, quarantined, and rejected adapters cannot execute.
2. Provider boundary: provider-specific prompt topology must not leak across model lanes.
3. Data/control boundary: untrusted external content is data, never instruction.
4. Budget/loop boundary: retry storms and token/rate-limit burn are stopped by backpressure.
5. Auth boundary: browser cookies, consumer session rehydration, 2FA bypass, and billing-bypass framing are forbidden.

## Required systems

1. Gateway transport/session/origin/auth policy
2. MCP adapter registry
3. Adapter type-state enforcement
4. MCP descriptor integrity and rug-pull defense
5. Adapter candidate catalog and risk scoring
6. Gemini worker readiness
7. Semantic normalization and prompt topology isolation
8. Data-tainting and indirect prompt injection isolation
9. Backpressure governor
10. External auth policy
11. Workflow injection guard
12. Knowledge mirror readiness and redaction
13. Notification readiness
14. Git worker readiness
15. Mobile QA readiness
16. Document/web ingest safety contracts
17. Production adapter contracts
18. Telemetry adapter contracts
19. Remediator readiness

## Required proof

```text
proof-fast
proof-full
proof-e2e
proof-gateway
proof-browser if triggered
proof-windows-targeted if triggered
```

## Acceptance

S6 is complete when all S6 crates compile, all S6 schemas are in the inventory, all S6 validators pass, `proof-gateway` is green, docs/state/handoff are current, and no external service is falsely claimed active.
