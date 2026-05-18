# StealthEye Cloud Build Plan

## 0. Purpose

This document is the execution plan for building **StealthEye Cloud** in the public repository:

```text
StealthEyeLLC/stealtheye-cloud
```

The companion architecture document is:

```text
StealthEye_Cloud_Final_Technical_Specification.md
```

This build plan answers:

1. what gets built first
2. how large each drop should be
3. what files each drop must create
4. what GitHub work each drop must include
5. what CI must prove
6. how failed drops are repaired
7. how ChatGPT tab handoff works
8. when to merge
9. when to stop
10. when the build is complete

## 1. Build Doctrine

### 1.1 Big-drop rule

StealthEye Cloud is built in the largest practical final-form drops.

A drop is not a tiny patch. A drop is a coherent vertical slice that includes:

1. implementation
2. schemas
3. tests
4. fixtures
5. docs
6. GitHub Actions workflows
7. root agent files
8. `.stealtheye/` state artifacts
9. Relay/Seal/Active updates
10. proof summaries
11. final drop report

### 1.2 No watered-down builds

Do not build fake, toy, watered-down, throwaway, or prototype versions of systems that are meant to become core.

Narrow scope by selecting fewer capabilities per drop, not by weakening the architecture.

### 1.3 Final-form vertical slices

Each drop must use final names, final folder locations, final schema posture, final validation posture, and final handoff posture.

Do not create temporary file names that will be rewritten later unless they live under an explicitly experimental folder.

### 1.4 Public-safe rule

The public repo contains only the public proof kernel and public-safe implementation.

Do not include:

1. private strategy
2. private prompts
3. private commercial playbooks
4. client material
5. private memory
6. private API keys
7. secrets
8. paid-compute assumptions
9. private overlays
10. unreleased business tactics

### 1.5 One active tab rule

One ChatGPT tab is active until saturated.

When the active tab saturates, it must produce:

1. `STEALTHEYE_RELAY.md`
2. `STEALTHEYE_RELAY.json`
3. `STEALTHEYE_SEAL.json`
4. `STEALTHEYE_ACTIVE.md`
5. `NEXT_ACTION.md`

The next tab resumes from those files using the exact same procedure.

## 2. Human / ChatGPT Build Loop

### 2.1 Standard loop

For each mega drop:

1. ChatGPT generates the drop artifact.
2. User applies the drop to the repo.
3. User pushes the branch or lets the drop create the branch if applicable.
4. GitHub Actions runs public CI.
5. If CI passes, the PR is merged.
6. If CI fails, user gives ChatGPT the failure summary/log/artifact.
7. ChatGPT generates a repair drop.
8. Repeat until green, blocked, or boundary.

### 2.2 Merge rule

Merge only when:

1. required GitHub Actions are green
2. docs reflect implemented reality
3. Relay/Seal/Active are updated
4. no private leakage is present
5. no forbidden files are present
6. final drop report exists
7. next action is clear

### 2.3 Failure rule

On failure, do not redesign the phase.

Repair the actual failure.

Allowed repair work:

1. compile/type errors
2. failing tests
3. schema validation failures
4. CI workflow errors
5. browser proof failures
6. docs mismatch
7. missing fixture
8. missing artifact
9. invalid Relay/Seal/Active
10. public leakage risk

### 2.4 Stop conditions

Stop and ask the user only for:

1. secrets
2. passwords
3. paid compute
4. account permission changes
5. public release/tag outside approved scope
6. destructive irreversible action
7. private data exposure risk
8. legal/commercial commitment
9. unresolved product ambiguity
10. platform permission boundary

Do not ask for routine continuation, docs updates, test fixes, CI reruns, or handoff generation.

## 3. Repository Setup

### 3.1 Target repo

```text
StealthEyeLLC/stealtheye-cloud
```

### 3.2 Visibility

Current intended state:

```text
public
```

Reason:

1. public standard GitHub-hosted Actions are free for standard runners
2. public proof builds credibility
3. public proof kernel can be tested without exposing private strategy
4. no-local execution body depends on GitHub Actions

### 3.3 Branch model

Default branch:

```text
main
```

Build branches:

```text
build/s0-foundation-continuity-ci
build/s1-mission-executor-atomic-drop
build/s2-browser-body-proof
build/s3-mcp-app-control-workers
build/s4-learning-search-hypothesis-proof-viewer
build/s5-hardening-release-candidate
```

Repair branches may use:

```text
repair/s<N>-<short-failure-name>
```

But prefer repairing directly on the active build branch when practical.

### 3.4 PR model

One draft PR per drop.

Each PR body must include:

1. drop name
2. objective
3. files changed
4. schemas added
5. workflows added/changed
6. tests added
7. commands/GitHub Actions expected
8. Relay/Seal/Active updates
9. proof result
10. next action

### 3.5 Forbidden files

Do not create:

```text
CLAUDE.md
.github/copilot-instructions.md
.cursorrules
soul.md
MEMORY.md
rules.md
```

Use only the canonical StealthEye/agent files.

## 4. Canonical Root Files

The repo root must contain:

```text
AGENTS.md
llms.txt
llms-full.txt
AGENT_REPO_MAP.md
STEALTHEYE_ACTIVE.md
STEALTHEYE_DECISIONS.md
STEALTHEYE_RELAY.md
STEALTHEYE_RELAY.json
STEALTHEYE_SEAL.json
NEXT_ACTION.md
```

Each file must have exactly one purpose.

## 5. Canonical Folder Structure

The repo must converge on:

```text
/
  AGENTS.md
  llms.txt
  llms-full.txt
  AGENT_REPO_MAP.md
  STEALTHEYE_ACTIVE.md
  STEALTHEYE_DECISIONS.md
  STEALTHEYE_RELAY.md
  STEALTHEYE_RELAY.json
  STEALTHEYE_SEAL.json
  NEXT_ACTION.md
  README.md
  LICENSE
  Cargo.toml
  package.json
  pnpm-lock.yaml or package-lock.json
  rust-toolchain.toml

  crates/
    secloud-packets/
    secloud-relay/
    secloud-seal/
    secloud-memory/
    secloud-capabilities/
    secloud-mission/
    secloud-atomic-drop/
    secloud-github/
    secloud-ci/
    secloud-browser/
    secloud-mcp/
    secloud-cli/

  browser/
    playwright/
    stagehand/
    puppeteer/
    fixtures/

  schemas/
    packets/
    relay/
    seal/
    mission/
    workers/
    browser/
    capabilities/

  docs/
    STEALTHEYE_CLOUD_SPEC.md
    STEALTHEYE_CLOUD_BUILD_PLAN.md
    PUBLIC_KERNEL_BOUNDARY.md
    NO_LOCAL_BUILD_LAW.md
    CHATGPT_OPERATING_GUIDE.md
    MCP_APP_CONTROL_PLANE.md
    OSS_ASSIMILATION_MATRIX.md
    CODEX_WORKER_LANE.md
    BROWSER_BODY.md
    CI_PROOF_TIERS.md
    MEMORY_RELAY_SEAL.md
    CHATGPT_SKILLS.md

  tests/
    fixtures/
    golden/
    integration/

  scripts/
    verify.sh
    verify.ps1

  .github/
    workflows/
      proof-fast.yml
      proof-full.yml
      proof-browser.yml
      proof-windows-targeted.yml
      mission-executor.yml

  .stealtheye/
    active/
      current.md
      current.json

    decisions/
      canonical.md
      canonical.json

    relay/
      latest.md
      latest.json
      archive/

    seals/
      latest.json
      index.json
      archive/

    memory/
      work/
      patterns/
      oss/
      browser/
      ci/
      codex/

    packets/
      mission/
      approval/
      action/
      observation/
      failure/
      browser/
      repair/
      continuation/
      result/
      boundary/
      worker/

    schemas/

    skills/
      relay-generation/
      seal-generation/
      tab-resume/
      mission-executor/
      ci-repair/
      browser-repair/
      public-kernel-drop/
      oss-audit/
      codex-worker/
      spec-generation/

    workers/
      codex/
        queue/
        active/
        results/
        usage/

    capability/
      registry.json
      current.md

    drops/
      archive/
      reports/
```

## 6. Mega Drop Plan

The build uses six mega drops.

Each drop is intended to be as large as possible while still being reviewable and CI-repairable.

## 7. Drop S0 — Foundation, Continuity, Packet Spine, and Cheap CI

### 7.1 Objective

Bootstrap the public repo into a real StealthEye Cloud proof kernel.

S0 proves:

1. the repo has the canonical structure
2. root agent files exist
3. Relay/Seal/Active exist
4. schemas exist
5. validators exist
6. fast CI runs
7. future tabs can resume from artifacts
8. no forbidden files exist

### 7.2 Branch

```text
build/s0-foundation-continuity-ci
```

### 7.3 Deliverables

S0 must create:

1. root files
2. `.stealtheye/` structure
3. Rust workspace
4. core packet crate
5. Relay crate
6. Seal crate
7. CLI crate
8. JSON Schemas
9. schema fixtures
10. validator tests
11. docs
12. GitHub Actions fast proof
13. GitHub Actions full proof skeleton
14. Windows targeted proof skeleton
15. final S0 report

### 7.4 Core schemas

Create schemas for:

1. `StealthEyeRelayV0`
2. `StealthEyeSealV0`
3. `MissionPacketV0`
4. `MissionApprovalV0`
5. `ActionPacketV0`
6. `ObservationPacketV0`
7. `FailurePacketV0`
8. `RepairPacketV0`
9. `ContinuationPacketV0`
10. `ResultPacketV0`
11. `BoundaryPacketV0`
12. `BrowserObservationPacketV0`
13. `CodexTaskPacketV0`
14. `CapabilityRegistryV0`
15. `ToolRegistryV0`

### 7.5 CLI commands

Create `secloud` CLI with:

```text
secloud validate relay
secloud validate seal
secloud validate active
secloud validate decisions
secloud validate schemas
secloud doctor
secloud status
```

### 7.6 S0 acceptance

S0 passes when:

1. repo has canonical root files
2. `.stealtheye/` exists
3. Rust workspace builds
4. schemas validate
5. Relay validates
6. Seal validates
7. forbidden file scan passes
8. proof-fast green
9. proof-full green
10. targeted Windows proof can run
11. S0 final report exists
12. `NEXT_ACTION.md` points to S1

## 8. Drop S1 — Mission Executor, Atomic Drop Rail, Authority Queue, and GitHub Evidence

### 8.1 Objective

Build the execution core that lets ChatGPT generate large applyable drops and let the repo validate/apply/prove them through GitHub.

S1 proves:

1. drops can be represented
2. drops can be validated
3. drops can be planned
4. drops can be safely applied
5. authority queue exists
6. output shelf exists
7. GitHub evidence can be normalized
8. CI failures become failure cards
9. mission status is queryable

### 8.2 Branch

```text
build/s1-mission-executor-atomic-drop
```

### 8.3 Core systems

Implement:

1. `MissionPacketV0`
2. `MissionApprovalV0`
3. `MissionExecutorDispatchV0`
4. `AuthorityQueueV0`
5. `OutputShelfV0`
6. `AtomicDropPackageV0`
7. `FileSetBundleV0`
8. `DropPlanV0`
9. `DropValidationV0`
10. `DropApplyReportV0`
11. `GitHubEvidencePackV0`
12. `CiEvidenceCardV0`
13. `PrEvidenceCardV0`
14. `FailureCardV0`
15. `RepairPacketV0`
16. `OperatorStateV0`
17. `StealthEyeAutonomyStatusV0`

### 8.4 S1 acceptance

S1 passes when:

1. valid atomic drop fixture validates
2. invalid atomic drop fixtures fail for the right reasons
3. mission packet fixture validates
4. authority queue fixture validates
5. output shelf fixture validates
6. dry-run apply works
7. apply works in test sandbox
8. forbidden file scan still passes
9. GitHub evidence fixture normalizes
10. failure card fixture generates
11. mission-executor workflow validates
12. proof-fast green
13. proof-full green
14. S1 report exists
15. `NEXT_ACTION.md` points to S2

## 9. Drop S2 — Browser Body, Replay Proof, and Visual Evidence

### 9.1 Objective

Build the first-class browser body.

S2 proves:

1. browser smoke runs in public CI
2. screenshots/DOM/console/network are captured
3. browser failures become repair cards
4. replay fixtures exist
5. browser proof can create release/demo artifacts
6. browser body obeys no-local law

### 9.2 Branch

```text
build/s2-browser-body-proof
```

### 9.3 Core systems

Implement:

1. `BrowserRunRequestV0`
2. `BrowserObservationPacketV0`
3. `BrowserRepairPacketV0`
4. `BrowserArtifactIndexV0`
5. `BrowserReplayPackV0`
6. `BrowserRouteSmokeV0`
7. `BrowserConsoleFailureV0`
8. `BrowserNetworkFailureV0`
9. `BrowserScreenshotRefV0`
10. `BrowserDomSketchV0`
11. `ExplorationToReplayCandidateV0`
12. `VisualEvidenceCardV0`

### 9.4 S2 acceptance

S2 passes when:

1. Playwright smoke green
2. browser artifact index produced
3. screenshot artifact produced
4. DOM sketch produced
5. console/network capture works
6. browser repair card fixture validates
7. replay pack fixture validates
8. proof-browser green
9. proof-fast green
10. proof-full green
11. S2 report exists
12. `NEXT_ACTION.md` points to S3

## 10. Drop S3 — MCP/App Control Plane, Tool Registry, Skills, Workers, and Background Capability Reality

### 10.1 Objective

Build the controlled gateway layer and real background/autonomy surfaces.

S3 proves:

1. one controlled tool registry exists
2. no random MCP tool chaos exists
3. tool schemas are deterministic
4. capabilities are queryable
5. Skills are defined
6. Tasks/Deep Research/Agent Mode/Codex reality is represented
7. Codex worker queue exists
8. background worker board exists

### 10.2 Branch

```text
build/s3-mcp-app-control-workers
```

### 10.3 Core systems

Implement:

1. `ToolIdentityV0`
2. `ToolCapabilityV0`
3. `ToolPermissionEnvelopeV0`
4. `ToolResultEnvelopeV0`
5. `ToolHealthV0`
6. `ToolRegistryV0`
7. `ToolCapabilitySearchV0`
8. `ToolPackManifestV0`
9. `StealthEyeCapabilitiesV0`
10. `StealthEyeWorkersV0`
11. `CodexTaskPacketV0`
12. `CodexResultImportV0`
13. `CodexUsageSnapshotV0`
14. `DeepResearchTaskPacketV0`
15. `ResearchResultImportV0`
16. `AgentModeTaskPacketV0`
17. `FeatureAvailabilityCheckV0`

### 10.4 S3 acceptance

S3 passes when:

1. tool registry validates
2. blocked tool names test passes
3. all Skill folders validate
4. Codex queue schema validates
5. worker board validates
6. capabilities file validates
7. background reality docs exist
8. no hidden autonomy claims appear
9. proof-fast green
10. proof-full green
11. proof-browser still green
12. S3 report exists
13. `NEXT_ACTION.md` points to S4

## 11. Drop S4 — Self-Improving Skills, Past-Session Search, Hypothesis Racing, and Public Proof Canvas

### 11.1 Objective

Build the compounding intelligence layer without model training or paid default infrastructure.

S4 proves:

1. successful patterns become Skill candidates
2. Skills can be promoted by rule
3. past sessions can be searched
4. hard failures can create candidate branches/packets
5. reducer can choose a candidate
6. public proof surface can summarize mission state

### 11.2 Branch

```text
build/s4-learning-search-hypothesis-proof-viewer
```

### 11.3 Core systems

Implement:

1. `FeedbackSignalV0`
2. `PatternCandidateV0`
3. `SkillCandidateV0`
4. `SkillPromotionDecisionV0`
5. `PastSessionSearchV0`
6. `PastSessionSearchResultV0`
7. `TemplateToSkillCompilerV0`
8. `SkillTemplateIndexV0`
9. `HypothesisRaceV0`
10. `CandidateBranchV0`
11. `CandidateReducerV0`
12. `RemoteComputerUseBodyV0`
13. `StealthEyeCanvasStateV0`
14. `PublicProofSummaryV0`

### 11.4 S4 acceptance

S4 passes when:

1. FeedbackSignal fixture validates
2. PatternCandidate fixture validates
3. SkillCandidate fixture validates
4. SkillPromotionDecision fixture validates
5. PastSessionSearch works on fixture archive
6. HypothesisRace fixture validates
7. CandidateReducer fixture validates
8. Canvas summary builds
9. no training dependency exists
10. no paid default exists
11. proof-fast green
12. proof-full green
13. proof-browser green
14. S4 report exists
15. `NEXT_ACTION.md` points to S5

## 12. Drop S5 — Full Hardening, Public Release Candidate, and First End-to-End Mission

### 12.1 Objective

Harden the entire system and prove one complete end-to-end mission.

S5 proves:

1. a mission can start
2. mission approval exists
3. drop is validated
4. drop is applied
5. CI proof runs
6. browser proof runs
7. failure card can be generated
8. repair can be generated
9. Relay updates
10. Seal updates
11. next tab can resume
12. public proof surface is updated
13. final report is generated

### 12.2 Branch

```text
build/s5-hardening-release-candidate
```

### 12.3 S5 acceptance

S5 passes when:

1. all required workflows are green
2. end-to-end mission fixture passes
3. Relay validates
4. Seal validates
5. Active validates
6. Canvas builds
7. no forbidden files exist
8. no private leakage detected
9. Windows targeted proof passes
10. browser proof passes
11. final report exists
12. release candidate checklist passes
13. `NEXT_ACTION.md` points to post-S5 build or public docs polish

## 13. Repair Drop Protocol

### 13.1 Naming

Repair drops use:

```text
REPAIR_S<N>_<SHORT_FAILURE>.md
```

or if using scripts:

```text
APPLY_REPAIR_S<N>_<SHORT_FAILURE>.ps1
APPLY_REPAIR_S<N>_<SHORT_FAILURE>.sh
```

### 13.2 Required repair contents

Every repair drop must include:

1. failure summary
2. root cause
3. files changed
4. why the fix is correct
5. tests updated
6. docs updated if needed
7. proof to rerun
8. expected outcome
9. updated Active/Next Action if needed
10. updated Seal only if boundary/checkpoint requires it

### 13.3 Repair rules

Do not:

1. remove tests to pass
2. weaken validation to pass
3. fake success
4. skip docs if behavior changed
5. bypass schema validation
6. bypass CI
7. introduce private content
8. introduce forbidden files
9. switch architecture
10. add unrelated features

## 14. GitHub Actions Cost Policy

Use public standard GitHub-hosted runners.

Default:

```text
ubuntu-latest
```

Targeted only:

```text
windows-latest
```

Do not use by default:

1. larger runners
2. self-hosted runners
3. paid browser cloud
4. paid compute
5. GPU runners
6. macOS runners
7. private heavy CI
8. deployment environments
9. secret-bearing workflows
10. long-running scheduled builds

## 15. Build Completion Criteria

The build is complete when:

1. S0 through S5 are merged
2. all required workflows pass
3. end-to-end mission fixture passes
4. browser proof works
5. Relay/Seal handoff works
6. mission executor works
7. atomic drop rail works
8. Skills validate
9. capabilities registry validates
10. worker board validates
11. public proof canvas builds
12. docs match implementation
13. no forbidden files exist
14. no private leakage exists
15. final `STEALTHEYE_SEAL.json` has `seal_type: FINAL`

## 16. Immediate Next Action

Create and apply the first mega drop:

```text
Drop S0 — Foundation, Continuity, Packet Spine, and Cheap CI
```

Target branch:

```text
build/s0-foundation-continuity-ci
```

Expected first artifact from ChatGPT:

```text
APPLY_STEALTHEYE_CLOUD_S0_FOUNDATION.ps1
```

Optional Unix equivalent:

```text
apply_stealtheye_cloud_s0_foundation.sh
```

S0 must be a real repo-changing drop that creates the foundation, not a prompt-only plan.
