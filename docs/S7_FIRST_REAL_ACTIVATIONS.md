# S7 — First Real Activations

## Objective

S7 activates selected working lanes after S6 proves the zero-trust enforcement substrate.

S7 is not a readiness phase. Every S7 lane must perform a real action and produce proof artifacts.

## Branch

```text
build/s7-first-real-activations
```

## Activated S7 package

S7 activates three public-safe lanes:

1. Mobile Browser Game Preview and Playtest Activation
2. Notification Lane Activation
3. Knowledge Mirror Export Activation

These lanes are intentionally bounded:

1. no browser-cookie/session-token automation
2. no secrets committed or printed
3. no paid compute
4. no production deployment
5. no database mutation
6. no live external sync for the mirror bundle

## S7A — Mobile Browser Game Preview and Playtest Activation

Real capability:

1. materializes a static mobile playtest surface at `public/previews/s7-mobile-playtest/index.html`
2. runs Playwright with an iPhone-class viewport
3. performs tap replay
4. performs swipe replay
5. captures screenshot artifact
6. captures console and network failures
7. emits a mobile QA artifact index
8. emits a playtest link artifact pointing to the static preview path

Required proof:

```text
proof-mobile
npm run proof:mobile
npm run proof:mobile:summary
s7-mobile-proof-artifacts artifact upload
```

## S7B — Notification Lane Activation

Real capability:

1. emits a notification activation artifact on every proof run
2. performs a real dispatch only when both of these are configured:
   - `STEALTHEYE_NOTIFICATION_WEBHOOK_URL`
   - `STEALTHEYE_NOTIFICATION_REAL_DISPATCH=true`
3. falls back to dry-run proof when no webhook secret is configured
4. runs sensitive-content redaction before dispatch or artifact emission
5. never logs or exports the webhook secret value

Required proof:

```text
npm run proof:notification
notification dry-run green by default
real dispatch green only when explicit secret and enable flag are configured
notification content redaction green
s7-notification-artifacts artifact upload
```

## S7C — Knowledge Mirror Export Activation

Real capability:

1. exports public-safe docs and reports into `.stealtheye/mirror/export/s7-knowledge-mirror/bundle`
2. generates a manifest
3. generates a semantic snapshot from document headings and byte lengths
4. runs a redaction scan against credential-shaped content
5. uploads the mirror bundle as a GitHub Actions artifact

External Drive, NotebookLM, or consumer browser upload is a later optional activation and must not use browser-session token hacks.

Required proof:

```text
npm run proof:mirror
npm run proof:activations:summary
s7-knowledge-mirror-artifacts artifact upload
s7-activation-proof-artifacts artifact upload
```

## S7 packet contracts

S7 adds these packet contracts:

```text
MobilePlaytestActivationV0
NotificationActivationRunV0
KnowledgeMirrorExportV0
S7ActivationProofV0
```

The `secloud-activations` crate validates the activation contract inventory, required lanes, and boundary checks.

## Required workflows

```text
.github/workflows/proof-mobile.yml
.github/workflows/proof-activations.yml
```

## Active versus later lanes

Active in S7:

1. Mobile Browser Game Preview and Playtest Activation
2. Notification Lane Activation
3. Knowledge Mirror Export Activation

Not activated in this S7 drop:

1. Gemini Worker Lane Activation
2. Document/Web Ingest Activation
3. Vercel Preview Adapter Activation
4. Supabase Read-Only Schema Inspect Activation
5. Telemetry Intake Activation
6. Git Worker Activation
7. Android Emulator QA Activation

## Acceptance

S7 passes when:

1. mobile preview/playtest static surface exists
2. mobile tap/swipe replay succeeds
3. mobile QA artifacts exist
4. notification dry-run succeeds without a secret
5. real notification dispatch is gated behind explicit secret plus enable flag
6. notification content redaction passes
7. knowledge mirror bundle is generated
8. semantic snapshot is generated
9. mirror redaction passes
10. all activation contract tests pass
11. `proof-mobile` is green
12. `proof-activations` is green
13. existing Rust/browser/e2e/gateway proof gates remain green
