# Tool Use Planner

## Purpose

The Tool Use Planner makes tool selection deliberate, batched, and low-friction.

## Required packet families

```text
ToolUsePlanV0
ToolSelectionPolicyV0
ToolCallBatchPlanV0
ToolCallEfficiencyReportV0
ToolAvoidanceDecisionV0
ToolPreflightCheckV0
ToolResultTrustLevelV0
ToolCallDeduplicationReportV0
ToolCallBudgetV0
```

## Required behavior

Before heavy execution, the assistant must identify:

1. intended tool class
2. required reads vs writes
3. batching opportunity
4. duplicate-read avoidance
5. fallback ladder
6. trust tier for tool results
7. true boundary risk

## Acceptance

S10 implementation must add `secloud validate tool-use-policy`, valid/invalid fixtures, and a tool-use proof artifact.
