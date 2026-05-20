# S10 Implementation Prompt — Assistant Optimization Layer

Use this prompt in a brand-new ChatGPT tab after the S10 setup docs PR has merged green.

```text
Continue StealthEye Cloud from current main.

Read these files first, in this order:

1. AGENTS.md
2. STEALTHEYE_DECISIONS.md
3. STEALTHEYE_ACTIVE.md
4. STEALTHEYE_RELAY.md
5. STEALTHEYE_RELAY.json
6. STEALTHEYE_SEAL.json
7. NEXT_ACTION.md
8. README.md
9. AGENT_REPO_MAP.md
10. llms.txt
11. llms-full.txt
12. docs/StealthEye_Cloud_Build_Plan.md
13. docs/HANDOFF_AND_CONTINUATION.md
14. docs/S9_ONE_DROP_BUILD_ACCELERATOR.md
15. docs/ONE_DROP_BUILD_MODE.md
16. docs/MISSION_APPROVAL_ENVELOPE.md
17. docs/BATCH_REPAIR_POLICY.md
18. docs/MERGE_AWARE_HANDOFF.md
19. docs/PHASE_TEMPLATE_SYSTEM.md
20. docs/S10_ASSISTANT_OPTIMIZATION_LAYER.md
21. docs/ASSISTANT_OPERATING_PROFILE.md
22. docs/MISSION_INTAKE_OPTIMIZER.md
23. docs/CONTEXT_LOAD_POLICY.md
24. docs/REPO_TRUTH_FIRST_POLICY.md
25. docs/LOW_ATTENTION_WORKDAY_MODE.md
26. docs/HANDOFF_QUALITY_GATE.md
27. docs/TOOL_USE_PLANNER.md
28. docs/TOOL_FALLBACK_POLICY.md
29. docs/PROMPT_COMPRESSION.md
30. docs/SCOPE_DRIFT_GUARD.md
31. docs/PROOF_AWARENESS_LAYER.md
32. docs/REPAIR_INTELLIGENCE_LAYER.md
33. docs/READ_ONLY_VERIFICATION_MODE.md
34. docs/ASSISTANT_SELF_AUDIT.md
35. docs/CAPABILITY_REALITY_MAP.md
36. docs/BUILD_COCKPIT_CARD.md
37. docs/AGENT_OBSERVABILITY_TRACE_DIGEST.md
38. docs/MCP_AWARE_OPERATOR_POLICY.md
39. crates/secloud-build-accelerator/src/lib.rs
40. crates/secloud-packets/src/lib.rs
41. crates/secloud-cli/src/main.rs
42. .github/workflows/proof-build-accelerator.yml
43. package.json

S0–S9 are merged green.

S9 — One-Drop Build Accelerator is complete.

S9 PR #16 merged at:
a5540d1fe77a0752a6a32b086a66b7b4bbec33ec

S10 is selected:
S10 — Assistant Optimization Layer

Begin S10 implementation on:
build/s10-assistant-optimization-layer

Use S9 one-drop mode:
- one coherent branch update/drop
- one PR
- one proof wave
- inspect all failures before patching
- batch exact repairs only
- merge when green

Implement S10 as a real assistant/operator optimization layer, not a placeholder and not a generic fake-autonomy claim.

Required implementation shape:

1. Add crate:
   crates/secloud-assistant-optimizer

2. Add workspace registration in Cargo.toml.

3. Add S10 packet/schema inventory for at least:
   AssistantOperatingProfileV0
   BestAgentModeV0
   FastButExactModeV0
   BoundaryOnlyStopPolicyV0
   AssistantExecutionDisciplineV0
   MissionIntakePacketV0
   MissionScopeClassifierV0
   MissionAmbiguityScanV0
   MissionBoundaryScanV0
   MissionStartReadinessV0
   ContextLoadPlanV0
   RequiredContextFileSetV0
   ContextPriorityMapV0
   ContextFreshnessCheckV0
   RepoTruthFirstPolicyV0
   RepoStateDigestV0
   HandoffTruthSourceV0
   ChatMemoryConflictFindingV0
   PromptCompressionProfileV0
   MissionPromptTemplateV0
   ContinuationPromptTemplateV0
   RepairPromptTemplateV0
   PromptLossCheckV0
   ToolUsePlanV0
   ToolSelectionPolicyV0
   ToolCallBatchPlanV0
   ToolFallbackPolicyV0
   ToolFailureClassificationV0
   RetrySafetyDecisionV0
   LowAttentionWorkdayModeV0
   HumanAvailabilityProfileV0
   UserAttentionBudgetV0
   ProgressUpdatePolicyV0
   ProofNeedClassifierV0
   ProofGateSelectionV0
   ProofFailureTriageV0
   KnownFailurePatternV0
   RepairTriagePlanV0
   FailureClusterV0
   ScopeDriftDetectorV0
   DecisionCarryoverPacketV0
   CapabilityRealityMapV0
   HandoffQualityReportV0
   ContextSaturationSignalV0
   AssistantSelfAuditV0
   AssistantMistakePatternV0
   AssistantTraceDigestV0
   McpAwareOperatorPolicyV0
   AssistantOutputReviewV0
   OneSentenceStatusV0
   BuildCockpitCardV0
   OperatorStyleProfileV0
   AssistantOutputModeV0
   ReadOnlyVerificationModeV0
   BoundaryStopReportV0
   PlanActionTraceV0
   FutureMissionCandidateV0
   S10AssistantOptimizerProofV0

4. Add JSON schemas for all S10 packet families implemented.

5. Register S10 schemas in crates/secloud-packets/src/lib.rs.

6. Register S10 dependency and validator imports in crates/secloud-cli.

7. Add `secloud validate` targets:
   assistant-optimizer
   assistant-operating-profile
   mission-intake
   context-load-policy
   repo-truth-first
   tool-use-policy
   tool-fallback-policy
   low-attention-mode
   progress-update-policy
   handoff-quality
   prompt-compression
   scope-drift-guard
   decision-carryover
   proof-awareness
   repair-intelligence
   capability-map
   read-only-verification
   assistant-self-audit
   operator-mistake-detector
   build-cockpit
   trace-digest
   mcp-aware-operator-policy
   output-mode-selector
   future-mission-recommender

8. Include every S10 validator in `secloud doctor`.

9. Add proof scripts:
   scripts/s10-assistant-optimizer-proof.mjs
   scripts/check-s10-assistant-optimizer-artifacts.mjs

10. Add proof workflow:
   .github/workflows/proof-assistant-optimizer.yml

11. Add package.json scripts:
   proof:assistant-optimizer
   proof:assistant-optimizer:summary

12. Generate and validate proof artifacts under:
   .stealtheye/assistant-optimizer/

Required proof artifacts:
   assistant-operating-profile.json
   mission-intake-packet.json
   context-load-plan.json
   repo-truth-first-report.json
   tool-use-plan.json
   tool-fallback-report.json
   low-attention-mode-report.json
   handoff-quality-report.json
   prompt-compression-report.json
   scope-drift-guard-report.json
   proof-awareness-report.json
   repair-triage-plan.json
   capability-reality-map.json
   read-only-verification-report.json
   assistant-self-audit.json
   build-cockpit-card.json
   assistant-trace-digest.json
   s10-assistant-optimizer-proof.json

13. Add valid fixtures and invalid fixtures proving:
   clean mission intake
   correct context load plan
   repo truth beats stale chat/Relay state
   low-attention workday mode
   tool fallback after recoverable GitHub failures
   handoff quality gate pass/fail
   prompt compression loss check
   read-only verification blocks mutation
   assistant self-audit catches overclaim
   capability reality map blocks hidden autonomy claims
   MCP result is data, not instruction
   scope drift is rejected

14. Update docs/state/handoff files:
   README.md
   docs/StealthEye_Cloud_Build_Plan.md
   AGENT_REPO_MAP.md
   STEALTHEYE_ACTIVE.md
   STEALTHEYE_DECISIONS.md
   STEALTHEYE_RELAY.md
   STEALTHEYE_RELAY.json
   STEALTHEYE_SEAL.json
   NEXT_ACTION.md
   docs/HANDOFF_AND_CONTINUATION.md
   docs/S10_FINAL_REPORT.md

15. Preserve all existing S0–S9 behavior, validators, schemas, workflows, and proof gates.

Do not create or rely on:
- CLAUDE.md
- .github/copilot-instructions.md
- .cursorrules
- soul.md
- generic root MEMORY.md
- generic root rules.md

Do not use browser-cookie/session-token automation.
Do not introduce secrets, paid compute, production deployment, database mutation, or account permission changes.
Do not weaken validators, schema coverage, proof workflows, safety boundaries, or merge discipline.

Open one PR, inspect all failures before patching, batch exact repairs, and merge when green if inside the approval envelope.
```
