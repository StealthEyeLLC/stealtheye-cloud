# StealthEye Cloud Technical Specification

Version: 1.0 Final Build-Ready Specification  
Status: Final idea-phase lock / build-ready  
Repository: `StealthEyeLLC/stealtheye-cloud`  
Primary mode: public proof kernel, no-local GitHub/browser execution body

---

## 0. Executive Summary

StealthEye Cloud is the no-local, ChatGPT-native agent execution system designed to make GPT-5.5 Thinking operate as the primary intelligence while GitHub, public CI, browser automation, Codex, Tasks, Skills, and structured handoff artifacts provide the body.

It is distinct from CLA:

```text
CLA = Cloud Local Agent / local-body lineage
StealthEye Cloud = no-local ChatGPT-first cloud/browser/GitHub execution-body lineage
```

StealthEye Cloud is optimized for:

1. one active ChatGPT tab until saturation
2. tab-to-tab continuity through StealthEye Relay
3. one universal receipt/checkpoint through StealthEye Seal
4. public free GitHub Actions proof
5. no laptop requirement
6. browser body proof
7. mission-level approval
8. bundled writes
9. low confirmation friction
10. maximum practical output on a $20 ChatGPT plan

The system law:

```text
ChatGPT plans and repairs.
GitHub executes and proves.
Browser body observes.
Relay transfers context.
Seal checkpoints truth.
Memory remembers only execution-relevant state.
```

---

## 1. Product Identity

### 1.1 Name

Use:

```text
StealthEye Cloud
```

Internal acronym may be:

```text
SCA = StealthEye Cloud Agent
```

Public-facing wording should prefer `StealthEye Cloud`.

### 1.2 Repository

Primary public proof-kernel repo:

```text
StealthEyeLLC/stealtheye-cloud
```

Recommended private overlay later:

```text
StealthEyeLLC/stealtheye-cloud-private
```

### 1.3 Positioning

StealthEye Cloud is not:

1. a local daemon
2. a Tauri shell first
3. a Cursor replacement dependent on Cursor
4. a Claude/Copilot adapter
5. a giant generic agent framework
6. a fake autonomous background bot
7. a paid cloud-agent platform
8. a local laptop requirement

StealthEye Cloud is:

1. ChatGPT-native
2. no-local by default
3. GitHub/public-CI powered
4. browser-body enabled
5. schema/packet-driven
6. continuity-first
7. public-proof friendly
8. mission-approved
9. big-drop buildable
10. optimized for ChatGPT-to-ChatGPT continuation

---

## 2. Governing Laws

### 2.1 ChatGPT-first law

ChatGPT 5.5 Thinking is the primary intelligence and control surface.

All repo files, packet formats, handoffs, summaries, and proof artifacts must be shaped for ChatGPT to read, decide, act, repair, and continue quickly.

### 2.2 No-local law

The default build must not require the user's laptop.

The default execution body is:

```text
GitHub Actions + public proof repo + browser CI + Codex worker lane when launched/configured
```

Local execution may exist only as optional developer convenience, not as the primary path.

### 2.3 Public proof-kernel law

The public repo contains only public-safe proof-kernel implementation.

It may contain:

1. packet schemas
2. Relay/Seal/Memory core
3. browser proof body
4. GitHub Actions proof workflows
5. public fixtures
6. public docs
7. OSS assimilation matrix
8. mission executor public-safe core
9. tool registry public-safe core
10. Skills templates

It must not contain:

1. private strategy
2. private prompts
3. client data
4. secrets
5. private commercial plans
6. private memory
7. private overlays
8. sensitive worker routing

### 2.4 One-tab saturation law

Use one active ChatGPT tab until it saturates.

At saturation, generate:

1. `STEALTHEYE_RELAY.md`
2. `STEALTHEYE_RELAY.json`
3. `STEALTHEYE_SEAL.json`
4. `STEALTHEYE_ACTIVE.md`
5. `NEXT_ACTION.md`

The next tab resumes from those artifacts without the user re-explaining.

### 2.5 Mission approval law

One mission approval should authorize a bounded unit of work.

Do not ask per file, per test, per docs update, per safe rerun, or per repairable CI failure.

Stop only for:

1. secrets
2. passwords
3. paid compute
4. public release/tag/merge if not approved
5. destructive irreversible actions
6. private data exposure risk
7. account permission changes
8. legal/commercial commitments
9. true product ambiguity

### 2.6 Big-drop law

Build in large coherent final-form drops.

Every major drop should include:

1. code
2. schemas
3. tests
4. fixtures
5. docs
6. workflows
7. Relay/Seal/Active updates
8. proof gates
9. final report

No watered-down MVPs. Narrow scope by reducing capability count, not by weakening architecture.

### 2.7 No hidden autonomy law

StealthEye Cloud must never assume ChatGPT can secretly create tabs, silently switch modes, silently run Deep Research, or silently invoke Agent Mode.

Use only real available surfaces:

1. active ChatGPT tab
2. Skills
3. Tasks
4. Codex when launched/configured
5. GitHub Actions
6. Deep Research when user-started
7. Agent Mode when user-started
8. Apps/connectors/MCP where available
9. Relay/Seal handoffs

### 2.8 One receipt type law

Use one universal receipt/checkpoint artifact:

```text
StealthEye Seal
```

Do not create many receipt families.

Seal types are only:

```text
MISSION
APPROVAL
PROOF
BLOCKED
HANDOFF
FINAL
```

### 2.9 OSS assimilation law

Adopt only free/open-source tools or patterns that create real capability, speed, or power gains with low friction.

Prefer:

```text
patterns first, dependencies second
```

Avoid:

1. paid defaults
2. local-first defaults
3. AGPL/license-risk defaults
4. bulky runtimes
5. uncontrolled tool chaos
6. fake autonomy

### 2.10 Forbidden adapter file law

Do not create or rely on:

```text
CLAUDE.md
.github/copilot-instructions.md
.cursorrules
soul.md
MEMORY.md
rules.md
```

The user is not using Claude or Copilot, wants to move away from Cursor, and does not want vague spiritual/persona files.

---

## 3. Primary Architecture

### 3.1 System formula

```text
ChatGPT 5.5 Thinking
+ StealthEye Project context
+ one-tab saturation
+ StealthEye Relay
+ StealthEye Seal
+ StealthEye Memory
+ ChatGPT-native packet VM
+ public GitHub Actions execution body
+ browser body
+ mission executor
+ atomic drop rail
+ tool registry
+ Codex worker lane
+ Skills
+ Tasks
+ OSS pattern assimilation
```

### 3.2 Layer stack

```text
Human layer
  Natural language between user and ChatGPT

Agent layer
  ChatGPT-native packets, Skills, Relay, Seal, Active state

Control layer
  Mission approval, mission executor, tool registry, capability registry

Execution layer
  GitHub Actions, public repo, Codex worker lane, browser body

Proof layer
  CI summaries, browser artifacts, evidence packs, StealthEye Seal

Memory layer
  Active/Decision/Work/Pattern/Relay memory
```

### 3.3 Language stack

Use:

```text
Rust core
TypeScript browser body
JSON Schema packet language
GitHub Actions YAML execution
Markdown agent state/docs
```

Rust owns:

1. packets
2. schemas
3. validators
4. Relay
5. Seal
6. memory core
7. mission executor
8. atomic drop rail
9. CI evidence models
10. CLI

TypeScript owns:

1. Playwright browser proof
2. Stagehand exploration
3. Puppeteer specialist capture
4. DOM sketches
5. browser summaries
6. visual artifact generation

JSON Schema owns:

1. packet contracts
2. fixture validation
3. MCP/App tool schemas
4. worker packet contracts
5. Relay/Seal contracts

Markdown owns:

1. root agent instructions
2. Active state
3. Decisions
4. Relay
5. Next Action
6. docs
7. Skills

---

## 4. Canonical Files and Folders

### 4.1 Required root files

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
README.md
LICENSE
```

### 4.2 Forbidden root files

```text
CLAUDE.md
.github/copilot-instructions.md
.cursorrules
soul.md
MEMORY.md
rules.md
```

### 4.3 Required `.stealtheye/` structure

```text
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

### 4.4 Docs structure

```text
docs/
  STEALTHEYE_CLOUD_SPEC.md
  STEALTHEYE_CLOUD_BUILD_PLAN.md
  PUBLIC_KERNEL_BOUNDARY.md
  NO_LOCAL_BUILD_LAW.md
  CHATGPT_OPERATING_GUIDE.md
  MEMORY_RELAY_SEAL.md
  CI_PROOF_TIERS.md
  BROWSER_BODY.md
  MCP_APP_CONTROL_PLANE.md
  CODEX_WORKER_LANE.md
  OSS_ASSIMILATION_MATRIX.md
  INTERNAL_REPO_ASSIMILATION.md
  CHATGPT_SKILLS.md
  BACKGROUND_CAPABILITY_REALITY.md
  MODE_LAUNCH_GUIDE.md
  GITHUB_ACTIONS_EXECUTION_BODY.md
  DROP_REPORT_TEMPLATE.md
```

---

## 5. StealthEye Memory Core

### 5.1 Memory law

```text
StealthEye Memory remembers work, not vibes.
```

Memory stores only execution-relevant state.

### 5.2 Memory layers

#### Layer 1 — Active Mission Memory

Files:

```text
STEALTHEYE_ACTIVE.md
.stealtheye/active/current.md
.stealtheye/active/current.json
```

Stores:

1. current mission
2. current branch
3. current PR
4. current approval
5. current blocker
6. latest CI status
7. latest browser status
8. next exact action
9. saturation status
10. active proof tier

#### Layer 2 — Canonical Decision Memory

Files:

```text
STEALTHEYE_DECISIONS.md
.stealtheye/decisions/canonical.md
.stealtheye/decisions/canonical.json
```

Stores stable decisions only:

1. product name
2. architecture laws
3. language stack
4. forbidden files
5. no-local law
6. browser law
7. MCP/App control law
8. OSS assimilation law
9. worker lane law
10. do-not-reopen list

#### Layer 3 — Work Memory

Folder:

```text
.stealtheye/memory/work/
```

Stores compact records of:

1. missions completed
2. branches used
3. PRs created
4. CI failures
5. CI fixes
6. browser failures
7. browser fixes
8. worker results
9. public proof outcomes
10. final reports

#### Layer 4 — Pattern Memory

Folder:

```text
.stealtheye/memory/patterns/
```

Stores reusable patterns:

1. recurring failures
2. successful repairs
3. useful drop structures
4. browser repair patterns
5. CI routing patterns
6. Codex task patterns
7. OSS adoption patterns
8. anti-patterns to avoid

#### Layer 5 — Relay Memory

Folder:

```text
.stealtheye/relay/
```

Stores handoffs.

### 5.3 Memory update triggers

Update memory only when:

1. mission starts
2. mission scope changes
3. approval changes
4. CI fails
5. CI passes
6. browser proof fails
7. browser proof passes
8. worker returns result
9. user makes canonical decision
10. mission is handed off
11. mission finishes
12. reusable pattern is confirmed

No memory updates for trivial reads or small routine actions.

### 5.4 Truth hierarchy

Use this order:

```text
current user instruction
current repo state
current CI/browser result
latest StealthEye Seal
STEALTHEYE_ACTIVE.md
STEALTHEYE_DECISIONS.md
STEALTHEYE_RELAY.md/json
older chat memory
```

---

## 6. StealthEye Relay

### 6.1 Relay law

```text
Every handoff uses the same structure, every time.
```

### 6.2 Relay purpose

Relay is executable context for:

1. tab-to-tab handoff
2. project-to-project handoff
3. mission-to-mission handoff
4. action-to-action handoff
5. worker-to-main handoff
6. Codex-to-ChatGPT import
7. final mission handoff

### 6.3 Relay files

Required current files:

```text
STEALTHEYE_RELAY.md
STEALTHEYE_RELAY.json
```

Archive structure:

```text
.stealtheye/relay/archive/YYYY-MM-DD_HHMM_mission-slug/
  RELAY.md
  RELAY.json
  ACTIVE.md
  SEAL.json
  NEXT_ACTION.md
```

### 6.4 Relay Markdown sections

Every Relay must have exactly these sections:

1. Resume Command
2. Current Mission
3. Current State
4. Latest Verified Result
5. Active Approval Envelope
6. Next Exact Action
7. Decisions That Must Not Drift
8. Do Not Reopen
9. Open Questions / Boundaries
10. Required Files / Repos / Branches
11. Latest Seal
12. Failure / Blocker State
13. Codex / Worker State
14. Browser State
15. Public / Private Boundary

### 6.5 Resume command

Every Relay begins with:

```text
Resume this StealthEye Cloud mission from this Relay. Use the latest Seal as truth, obey the Active Approval Envelope, do not reopen frozen decisions, and perform the Next Exact Action unless a boundary is required.
```

### 6.6 Relay validation

A Relay is invalid if:

1. no resume command
2. no current mission
3. no latest Seal reference
4. no next exact action
5. no approval status
6. missing do-not-reopen section
7. stale branch/PR information
8. unresolved contradiction
9. missing public/private boundary
10. missing source context

---

## 7. StealthEye Seal

### 7.1 Seal law

```text
One receipt type. Many contexts. Only when necessary.
```

### 7.2 Seal purpose

Seal checkpoints what is true.

Relay transfers context. Memory remembers. Seal checkpoints.

### 7.3 Seal types

Only:

```text
MISSION
APPROVAL
PROOF
BLOCKED
HANDOFF
FINAL
```

### 7.4 Seal generation triggers

Generate a Seal only for:

1. mission start
2. mission approval
3. public branch push
4. CI green
5. browser green
6. blocked state
7. worker result import
8. handoff
9. public release/merge boundary
10. mission final result

Do not generate Seals for:

1. every file read
2. every small edit
3. every log read
4. every search
5. every routine rerun
6. every browser screenshot
7. every internal packet

### 7.5 Seal schema fields

`StealthEyeSealV0` includes:

1. `seal_id`
2. `seal_type`
3. `created_at`
4. `product`
5. `mission_id`
6. `action_id`
7. `source_context`
8. `repo`
9. `branch`
10. `commit_sha`
11. `pr_url`
12. `approval_id`
13. `approval_scope_hash`
14. `artifact_hashes`
15. `ci_status`
16. `browser_status`
17. `worker_status`
18. `input_summary`
19. `output_summary`
20. `next_action`
21. `boundary_status`
22. `supersedes`
23. `content_hash`

### 7.6 Seal storage

```text
STEALTHEYE_SEAL.json
.stealtheye/seals/latest.json
.stealtheye/seals/index.json
.stealtheye/seals/archive/YYYY-MM-DD/<seal_id>.json
```

---

## 8. ChatGPT-Native Packet VM

### 8.1 Packet law

Everything below the human conversation layer must be optimized for ChatGPT to read, decide, act, repair, and continue with minimal translation.

### 8.2 Core packets

Required packets:

1. `MissionPacketV0`
2. `MissionApprovalPacketV0`
3. `ActionPacketV0`
4. `ScopePacketV0`
5. `FileSetPacketV0`
6. `RunPacketV0`
7. `ObservationPacketV0`
8. `FailurePacketV0`
9. `BrowserObservationPacketV0`
10. `BrowserRepairPacketV0`
11. `RepairPacketV0`
12. `BoundaryPacketV0`
13. `ResultPacketV0`
14. `ContinuationPacketV0`
15. `PromotionPacketV0`

### 8.3 Worker packets

Required worker packets:

1. `CodexTaskPacketV0`
2. `CodexResultImportV0`
3. `CodexUsageSnapshotV0`
4. `DeepResearchTaskPacketV0`
5. `ResearchResultImportV0`
6. `AgentModeTaskPacketV0`
7. `WorkerScorecardV0`

### 8.4 Learning packets

Required learning packets:

1. `FeedbackSignalV0`
2. `PatternCandidateV0`
3. `SkillCandidateV0`
4. `SkillPromotionDecisionV0`
5. `PastSessionSearchV0`
6. `TemplateToSkillCompilerV0`
7. `SkillTemplateIndexV0`

### 8.5 Hard-failure packets

Required hard-failure packets:

1. `HypothesisRaceV0`
2. `CandidateBranchV0`
3. `CandidateReducerV0`
4. `RemoteComputerUseBodyV0`

### 8.6 Tool packets

Required tool packets:

1. `ToolIdentityV0`
2. `ToolCapabilityV0`
3. `ToolPermissionEnvelopeV0`
4. `ToolResultEnvelopeV0`
5. `ToolHealthV0`
6. `ToolRegistryV0`
7. `ToolCapabilitySearchV0`
8. `ToolPackManifestV0`

---

## 9. Mission Executor and Atomic Drop Rail

### 9.1 Mission executor purpose

The Mission Executor turns one approved mission into controlled repo work.

It must support:

1. mission packet validation
2. approval envelope validation
3. file-set bundle validation
4. atomic drop validation
5. dry-run planning
6. scoped apply
7. proof execution
8. evidence import
9. repair packet generation
10. compact result output

### 9.2 One-call mission harness

The preferred write operation is:

```text
run_mission_bundle
```

It should replace many tiny write calls.

### 9.3 Authority Queue

Add:

```text
AuthorityQueueV0
```

Purpose:

1. hold approved mission packets
2. enforce scope
3. prevent unapproved writes
4. support GitHub workflow_dispatch fallback
5. provide queue status to ChatGPT

### 9.4 Output Shelf

Add:

```text
OutputShelfV0
```

Purpose:

1. hold mission output
2. hold generated reports
3. hold artifacts
4. hold PR links
5. hold proof summaries

### 9.5 Atomic Drop Package

Add:

```text
AtomicDropPackageV0
```

Must include:

1. schema version
2. mission id
3. target branch
4. operations
5. file contents or refs
6. intent
7. expected proof
8. rollback plan
9. path budget
10. content hashes

Must reject:

1. absolute paths
2. path traversal
3. `.git`
4. `.env*`
5. `secrets/`
6. private keys
7. case collisions
8. duplicate paths
9. directory/file collisions
10. unauthorized deletes
11. binary-like content unless allowed
12. forbidden root files

---

## 10. GitHub Actions Execution Body

### 10.1 Public CI law

Use public standard GitHub-hosted runners by default.

Default:

```text
ubuntu-latest
```

Targeted:

```text
windows-latest
```

Do not default to:

1. larger runners
2. macOS runners
3. self-hosted runners
4. paid browser cloud
5. GPU runners
6. private heavy CI

### 10.2 Workflows

Required workflows:

```text
.github/workflows/proof-fast.yml
.github/workflows/proof-full.yml
.github/workflows/proof-browser.yml
.github/workflows/proof-windows-targeted.yml
.github/workflows/mission-executor.yml
```

### 10.3 `proof-fast`

Runs:

1. checkout
2. Rust toolchain setup
3. cargo fmt
4. cargo check
5. cargo test
6. schema validation
7. Relay validation
8. Seal validation
9. root file validation
10. forbidden file scan

### 10.4 `proof-full`

Runs:

1. all `proof-fast`
2. clippy
3. all fixtures
4. docs presence check
5. Skill validation
6. capability registry validation
7. packet snapshots

### 10.5 `proof-browser`

Runs:

1. Playwright setup
2. route smoke
3. blank-screen detection
4. console capture
5. network capture
6. screenshot capture
7. DOM sketch
8. artifact upload
9. browser repair card generation

### 10.6 `proof-windows-targeted`

Runs:

1. Rust tests
2. path safety tests
3. Windows separator tests
4. atomic drop path rejection
5. Relay validation
6. Seal validation

### 10.7 `mission-executor`

Runs:

1. workflow_dispatch packet intake
2. dispatch validation
3. authority validation
4. drop validation
5. dry-run apply
6. optional apply
7. proof run
8. summary output
9. artifact upload

---

## 11. Browser Body

### 11.1 Browser body stack

Use:

1. Playwright for deterministic browser proof
2. Stagehand for AI exploration
3. Puppeteer for Chrome/CDP/PDF specialist tasks

### 11.2 Browser proof outputs

Every browser run should emit:

1. `browser-summary.md`
2. `browser-summary.json`
3. `browser-console-errors.json`
4. `browser-network-failures.json`
5. `browser-screenshot-path.txt`
6. `browser-dom-sketch.json`
7. `browser-next-action.txt`
8. `BrowserRepairPacketV0.json`

### 11.3 Browser tests

Browser proof should support:

1. route loads
2. blank-screen detection
3. console errors
4. network failures
5. screenshots
6. DOM/accessibility summaries
7. form flows
8. docs/site smoke
9. app/game first-screen smoke
10. release/demo screenshots

### 11.4 Exploration-to-replay

Stagehand exploration must be converted into deterministic Playwright replay before becoming proof.

---

## 12. MCP/App Control Plane

### 12.1 Control plane law

Do not expose random MCP servers directly.

Expose one controlled gateway:

```text
StealthEye Cloud MCP/App Control Plane
```

### 12.2 Tool registry

Required:

1. closed registry
2. deterministic tool schemas
3. capability classes
4. authority classes
5. permission envelopes
6. health checks
7. blocked tool names
8. normalized results
9. assistant-readable summaries
10. no-action blocked responses

### 12.3 Allowed high-level tool families

1. status
2. capabilities
3. validation
4. mission planning
5. drop validation
6. evidence reading
7. browser observation
8. worker packet creation
9. research packet creation
10. Relay/Seal generation

### 12.4 Blocked raw tool names

Block:

```text
run
exec
shell.run
write
delete
git
github
curl
fetch
browser.action
device.action
release
deploy
secret.read
payment
```

---

## 13. ChatGPT Platform Setup

### 13.1 ChatGPT Project

Use one primary project:

```text
StealthEye Cloud — Build HQ
```

### 13.2 Project instruction

Use:

```text
This project is for StealthEye Cloud, the no-local ChatGPT-native GitHub/browser execution-body agent. Use one active tab until saturated, then generate StealthEye Relay + StealthEye Seal + Active State + Next Action for the next tab. Obey AGENTS.md, STEALTHEYE_DECISIONS.md, STEALTHEYE_ACTIVE.md, STEALTHEYE_RELAY.md/json, and STEALTHEYE_SEAL.json. Do not use CLAUDE.md, Copilot instructions, Cursor rules, or soul.md. Optimize for minimal approvals, maximum output, public free CI proof, browser body proof, and exact same continuation procedure every tab.
```

### 13.3 Project file pack

Keep current:

1. final technical spec
2. build plan
3. AGENTS.md
4. llms.txt
5. llms-full.txt
6. AGENT_REPO_MAP.md
7. STEALTHEYE_ACTIVE.md
8. STEALTHEYE_DECISIONS.md
9. STEALTHEYE_RELAY.md
10. STEALTHEYE_RELAY.json
11. STEALTHEYE_SEAL.json
12. NEXT_ACTION.md
13. OSS assimilation matrix
14. Codex worker lane guide
15. Memory/Relay/Seal guide

### 13.4 Tasks

Use sparingly:

1. weekly Relay/Seal freshness
2. weekly OSS refresh
3. stale mission reminder
4. Codex follow-up reminder
5. public CI status reminder

### 13.5 Skills

StealthEye Cloud Skills:

1. relay generation
2. seal generation
3. tab resume
4. mission executor
5. CI repair
6. browser repair
7. public kernel drop
8. OSS audit
9. Codex worker
10. spec generation

---

## 14. Codex Worker Lane

### 14.1 Codex role

Codex is a premium background/cloud coding worker, not the main brain.

ChatGPT remains planner/reviewer/orchestrator.

### 14.2 Codex files

```text
.stealtheye/workers/codex/queue/
.stealtheye/workers/codex/active/
.stealtheye/workers/codex/results/
.stealtheye/workers/codex/usage/
```

### 14.3 Codex packets

1. `CodexTaskPacketV0`
2. `CodexResultImportV0`
3. `CodexUsageSnapshotV0`
4. `WorkerScorecardV0`

### 14.4 Codex use cases

Use Codex for:

1. hard implementation candidates
2. branch hypothesis racing
3. independent PR review
4. large refactor candidate
5. browser/UI repair branch
6. long-running code work

Do not burn Codex on:

1. routine docs
2. small schema edits
3. easy CI failures
4. formatting-only work
5. simple file creation

---

## 15. Background Capability Reality Layer

### 15.1 What ChatGPT cannot silently do

ChatGPT cannot silently:

1. create new ChatGPT tabs
2. move itself to another tab
3. invoke Deep Research mode
4. invoke Agent Mode
5. spawn Codex without user/tool initiation
6. bypass platform confirmations

### 15.2 What StealthEye Cloud uses instead

1. Relay/Seal for tab switching
2. Tasks for scheduled nudges
3. Codex for user-launched worker tasks
4. GitHub Actions for no-local execution
5. Deep Research packets for user-started heavy research
6. Agent Mode packets for user-started web action
7. Skills for repeatable workflows
8. Apps/MCP where available

### 15.3 Required capability files

```text
STEALTHEYE_CAPABILITIES.md
STEALTHEYE_WORKERS.md
docs/BACKGROUND_CAPABILITY_REALITY.md
docs/MODE_LAUNCH_GUIDE.md
```

---

## 16. OSS Assimilation Pack

### 16.1 Browser/tooling OSS

Adopt/use:

1. Playwright
2. Playwright MCP as reference/optional lane
3. Stagehand
4. Puppeteer
5. GitHub MCP Server as reference/control lane
6. Rust MCP SDK/RMCP
7. cargo-nextest
8. taiki-e/install-action
9. rust-cache
10. sccache as A/B trial
11. dorny/paths-filter
12. actionlint
13. lychee
14. ast-grep
15. tree-sitter via ast-grep first
16. ripgrep
17. fd
18. serde
19. schemars
20. jsonschema
21. insta
22. llms.txt

### 16.2 Conditional OSS

Use when relevant:

1. Hurl
2. Schemathesis
3. hyperfine
4. Gitoxide
5. mdBook

### 16.3 Avoid/defer

Avoid default dependency on:

1. Open Interpreter
2. Dify
3. Flowise
4. BrowserMCP local-profile defaults
5. paid Browserbase cloud
6. local-first tools
7. AGPL/license-risk components
8. full LangChain dependency
9. self-hosted runners
10. paid browser cloud

---

## 17. Open-Source Agent Assimilation Pack

### 17.1 Immediate pattern adoption

1. mini-SWE-agent — minimal high-agency loop
2. SWE-agent — ACI and issue repair lessons
3. Open SWE — async GitHub PR-agent pattern
4. Continue — source-controlled AI checks
5. Aider — repo map and diff-first editing
6. Cline — Plan/Act and checkpoint pattern
7. Roo Code — role/mode templates
8. PocketFlow — tiny workflow abstraction
9. Agentless — localization → repair → validation
10. Moatless/SWE-Search — hard-failure tree search

### 17.2 Named-agent scan upgrades

From OpenClaw:

1. gateway/control-plane lessons
2. live status surface idea
3. template ecosystem lessons
4. cost-avoidance law

From Hermes Agent:

1. self-improving Skills loop
2. memory nudges
3. past-session search
4. scheduled automation concepts
5. multi-channel gateway patterns

From Coasty/open-computer-use:

1. remote sandboxed computer-use body pattern
2. browser/terminal/desktop/planner body taxonomy
3. screenshot/UI observation
4. multi-VM/parallel orchestration as optional lane

From OpenClaw-RL:

1. feedback-to-pattern compiler
2. Skill promotion without training
3. no default RL/model training

From OpenCode/OpenHands/Composio/agent-orchestrator:

1. provider-agnostic worker interface
2. sandbox abstraction
3. tool registry and capability search
4. branch/PR candidate racing
5. review-comment repair loop

---

## 18. Internal Repo Assimilation Pack

### 18.1 Agent Tool Foundry

Adopt patterns:

1. chat-pack/read-pack exports
2. GitHub evidence import
3. PR evidence cards
4. CI evidence cards
5. repair packs
6. one-call mission harness
7. approval-collapse metric
8. operator state readback
9. capability registry
10. atomic drop package
11. hidden primitive tools / visible mission tools
12. final report for exception-only UX

### 18.2 Foundry Project Factory

Adopt patterns:

1. ProjectExecutionPacket
2. authority queue
3. output shelf
4. packet canonicalization
5. packet hashing
6. compact reports
7. packet validation before execution
8. frozen packet before build
9. MCP build server pattern
10. branch + file apply + verify + commit + optional PR sequence

### 18.3 StealthEye CLA

Adopt patterns:

1. closed MCP registry
2. deterministic tool schemas
3. capability/authority classes
4. approval-boundary representation
5. blocked tool registry
6. context pack redaction
7. no generic dangerous tool names
8. policy/tools/health/simulate smoke commands
9. final build-order lock discipline
10. no-watered-down-build doctrine

### 18.4 Glass

Adopt patterns:

1. replay-first proof surface
2. scene → change → evidence → Seal framing
3. bounded claim honesty
4. fixture-pack validation
5. hosted public proof demo pattern
6. screenshot/GIF capture pipeline
7. current truth vs long horizon docs
8. Overview vs Technical explanation layers
9. share-safe artifacts
10. browser media capture as release asset pipeline

### 18.5 Distill-CBL

Adopt patterns:

1. self-describing artifacts
2. embedded recovery context
3. compact integrity witness
4. small-core anti-abstraction law
5. readable artifact as UX
6. no opaque toolchain posture

### 18.6 Murkshot

Adopt patterns:

1. local/no-telemetry browser posture
2. local asset fallback
3. browser canvas smoke target
4. asset loader failure summaries
5. first-screen browser proof
6. offline/browser proof fixtures

### 18.7 Nox Tower

Adopt patterns:

1. deterministic replay checksums
2. data-only schema validation
3. route/fixture metadata
4. replay tape as proof fixture
5. deterministic smoke tests
6. playability repair docs as pattern memory

---

## 19. Self-Improving Skills and Past-Session Search

### 19.1 No-training learning loop

Use:

1. `FeedbackSignalV0`
2. `PatternCandidateV0`
3. `SkillCandidateV0`
4. `SkillPromotionDecisionV0`

Do not use model training, RL, fine-tuning, GPU, or paid training as core.

### 19.2 Skill promotion flow

1. mission succeeds
2. feedback signal created
3. reusable pattern detected
4. Skill candidate generated
5. candidate promoted after repeated success
6. Skill file updated
7. schema validates
8. proof runs

### 19.3 Past-session search

Search:

1. Relay archives
2. Seal archives
3. work memory
4. pattern memory
5. OSS memory
6. Codex results
7. CI failures
8. browser repairs

No vector database required in the first build.

---

## 20. Hypothesis Racing

### 20.1 Purpose

Use bounded branch/PR candidate racing for hard failures.

### 20.2 Packets

1. `HypothesisRaceV0`
2. `CandidateBranchV0`
3. `CandidateReducerV0`

### 20.3 Limits

Only activate when:

1. normal repair fails
2. failure has multiple plausible causes
3. public CI can test cheaply
4. branch count is bounded
5. Codex budget is acceptable if used

Default maximum:

1. three candidates
2. two CI attempts each
3. no paid compute
4. no private data

---

## 21. Remote Computer Use Future Lane

### 21.1 Status

Future optional lane, not default.

### 21.2 Purpose

Support tasks that GitHub Actions/browser CI cannot handle.

### 21.3 Bodies

1. Browser Body — current/near-term
2. Terminal Body — GitHub Actions
3. Desktop Body — future optional sandbox
4. Planner Body — ChatGPT

### 21.4 Constraints

No paid default.

No local requirement.

No spam/ToS-violating automation.

No uncontrolled desktop actions.

---

## 22. Build Drops

Use the separate build plan as execution source:

```text
StealthEye_Cloud_Build_Plan.md
```

Mega drops:

1. S0 — Foundation, Continuity, Packet Spine, and Cheap CI
2. S1 — Mission Executor, Atomic Drop Rail, Authority Queue, and GitHub Evidence
3. S2 — Browser Body, Replay Proof, and Visual Evidence
4. S3 — MCP/App Control Plane, Tool Registry, Skills, Workers, and Background Capability Reality
5. S4 — Self-Improving Skills, Past-Session Search, Hypothesis Racing, and Public Proof Canvas
6. S5 — Full Hardening, Public Release Candidate, and First End-to-End Mission

---

## 23. Completion Criteria

StealthEye Cloud is build-complete when:

1. S0 through S5 are merged
2. all required workflows pass
3. end-to-end mission fixture passes
4. browser proof works
5. Relay/Seal handoff works
6. mission executor works
7. atomic drop rail works
8. Skills validate
9. capability registry validates
10. worker board validates
11. public proof canvas builds
12. docs match implementation
13. no forbidden files exist
14. no private leakage exists
15. final Seal has `seal_type: FINAL`

---

## 24. Immediate Next Action

Build:

```text
S0 — Foundation, Continuity, Packet Spine, and Cheap CI
```

Branch:

```text
build/s0-foundation-continuity-ci
```

Expected first applyable artifact:

```text
APPLY_STEALTHEYE_CLOUD_S0_FOUNDATION.ps1
```

Optional Unix artifact:

```text
apply_stealtheye_cloud_s0_foundation.sh
```

S0 must be a real repo-changing drop, not a prompt-only plan.
