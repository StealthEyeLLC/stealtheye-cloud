# Agent Observability Trace Digest

## Purpose

The Agent Observability Trace Digest gives StealthEye Cloud a lightweight, public-safe record of assistant intent, tool actions, decisions, and outcomes.

It is not invasive telemetry and not private logging. It is a compact repo-native proof/continuation artifact.

## Required packet families

```text
AssistantTraceDigestV0
ActionIntentTraceV0
ToolCallTraceSummaryV0
DecisionTraceV0
OutcomeTraceV0
PlanActionTraceV0
TraceCompressionReportV0
LoopDetectionFindingV0
PublicSafeTraceDigestV0
TraceRedactionPolicyV0
```

## Required behavior

A trace digest should capture:

1. intended action
2. planned step
3. tool/action class used
4. result summary
5. decision made
6. skipped step justification
7. unexpected action finding if any
8. loop or retry risk if any
9. public-safe redaction status
10. next action

## Public-safe rule

Trace digests must not include secrets, private data, private strategy, hidden prompts, credential material, browser cookies, session tokens, or sensitive account details.

## Acceptance

S10 implementation must add `secloud validate trace-digest`, valid trace fixtures, redaction invalid fixtures, and an assistant-trace proof artifact.
