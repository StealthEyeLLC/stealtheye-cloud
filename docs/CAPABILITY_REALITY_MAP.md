# Capability Reality Map

## Purpose

The Capability Reality Map prevents fake automation claims by making assistant/tool capabilities explicit and status-coded.

## Required packet families

```text
AssistantCapabilityMapV0
CapabilityRealityMapV0
CapabilityStatusDigestV0
CapabilityUsePolicyV0
CapabilityBoundaryMapV0
CapabilityConfidenceV0
CapabilityAvailabilityCheckV0
NoHiddenAutonomyFindingV0
CapabilityRealityReportV0
```

## Capability statuses

```text
AVAILABLE
AVAILABLE_WITH_CONFIRMATION
REPO_SUPPORTED
CI_SUPPORTED
BROWSER_SUPPORTED
MCP_SUPPORTED
READINESS_ONLY
USER_INITIATED_ONLY
BLOCKED
FORBIDDEN
```

## Required behavior

The assistant must distinguish:

1. things available in the current chat/tool surface
2. things available only through GitHub Actions
3. things available only after user confirmation
4. things available only when the user starts a ChatGPT mode
5. readiness-only contracts
6. blocked or forbidden actions

## No-hidden-autonomy rule

Do not claim ChatGPT can secretly create tabs, silently invoke background modes, bypass confirmations, or perform hidden work.

## Acceptance

S10 implementation must add `secloud validate capability-map`, capability status fixtures, forbidden capability fixtures, and a capability-reality proof artifact.
