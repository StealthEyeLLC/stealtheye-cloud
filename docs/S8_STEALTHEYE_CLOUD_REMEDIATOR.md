# S8 — StealthEye Cloud Remediator

## Objective

S8 activates Remediator Mode as a flagship proof-driven repo remediation system.

Tagline:

```text
Broken repo in. Reproduced failure, bounded patch, green proof, remediation report out.
```

## Branch

```text
build/s8-remediator-mode
```

## Active crate

```text
crates/secloud-remediator
```

## Core rule

A repo is not remediated until all three facts are true:

1. the failing behavior is reproduced;
2. a bounded patch is applied;
3. proof gates pass after the patch.

If failure is not reproduced, Remediator may produce diagnosis, but it must not claim remediation.

## Implemented S8 modules

S8 implements active public-safe contracts and validators for:

```text
intake
permissions
reality_map
command_discovery
environment
reproduction
failure_taxonomy
localization
repair_strategy
patch_tournament
proof_plan
report
quote_risk
```

## Real proof body

S8 adds a deterministic public-safe proof script:

```text
scripts/s8-remediator-proof.mjs
scripts/check-s8-remediator-artifacts.mjs
```

The proof script creates a synthetic broken repo under `.stealtheye/remediator/workspaces/synthetic-broken-repo`, reproduces a failing `node --test` command, applies a bounded one-line patch, reruns proof green, emits a remediation report, emits a commercial quote/risk artifact with billing disabled, and emits a diagnosis-only artifact proving unreproduced failures cannot be called remediated.

## Workflow

```text
.github/workflows/proof-remediator.yml
```

The workflow runs Rust format/check/test, Remediator crate tests, Clippy for the Remediator crate, every S8 validator, the Remediator proof script, artifact validation, and artifact upload.

## Required validators

```text
secloud validate remediator
secloud validate remediation-intake
secloud validate remediation-permissions
secloud validate remediation-reality-map
secloud validate remediation-command-discovery
secloud validate remediation-environment
secloud validate remediation-reproduction
secloud validate remediation-failure-taxonomy
secloud validate remediation-localization
secloud validate remediation-repair-strategy
secloud validate remediation-patch-tournament
secloud validate remediation-proof-plan
secloud validate remediation-report
secloud validate remediation-commercial
```

## Required packet schemas

```text
RemediatorReadinessV0
RemediationIntakeV0
RemediationPermissionsV0
RemediationRealityMapV0
RemediationCommandDiscoveryV0
RemediationEnvironmentV0
RemediationReproductionV0
RemediationFailureTaxonomyV0
RemediationLocalizationV0
RemediationRepairStrategyV0
RemediationPatchTournamentV0
RemediationProofPlanV0
RemediationReportV0
RemediationCommercialV0
RemediatorExecutionReceiptV0
```

## Boundaries preserved

S8 does not activate secrets, paid compute, production deployment, database mutation, browser-cookie/session-token automation, or billing.

Commercial quote/risk output is an artifact only. It cannot activate billing.

## Acceptance

S8 passes when:

1. Remediator active contracts compile;
2. all S8 packet schemas exist;
3. every S8 validator passes;
4. the proof-remediator workflow reproduces the synthetic failure;
5. the proof-remediator workflow applies the bounded patch;
6. the proof-remediator workflow reruns proof green;
7. remediation and diagnosis-only reports are emitted;
8. commercial quote/risk artifact exists with `billing_activated: false`;
9. CI is green before merge.
