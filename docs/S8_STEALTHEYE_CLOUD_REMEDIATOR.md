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

## Crate

```text
crates/secloud-remediator
```

## Required modules

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
quote
```

## Core rule

A repo is not remediated until the failing behavior is reproduced, a bounded patch is applied, and proof gates pass.

If failure is not reproduced, Remediator may produce diagnosis but must not claim remediation.

## Required capabilities

1. intake classifier
2. permission envelope
3. repo reality map
4. command discovery
5. environment synthesis
6. failure reproduction
7. failure taxonomy
8. localization engine
9. repair strategy selection
10. patch tournament
11. proof plan
12. CI remediation mode
13. browser/mobile/game remediation mode
14. Rust remediation mode
15. TypeScript/PWA remediation mode
16. dependency remediation mode
17. public/private safety split
18. remediation report
19. commercial quote/risk pack without activating billing

## Required workflow

```text
.github/workflows/proof-remediator.yml
```

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

## Acceptance

S8 passes when Remediator performs a real remediation flow with reproduced failure, bounded patch, green proof, and a remediation report, or explicitly emits diagnosis-only status when reproduction is impossible.
