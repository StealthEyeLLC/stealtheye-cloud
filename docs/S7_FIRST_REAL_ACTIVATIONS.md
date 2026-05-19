# S7 — First Real Activations

## Objective

S7 activates selected working lanes after S6 proves the zero-trust enforcement substrate.

S7 is not a readiness phase. Every S7 lane must perform a real action and produce proof artifacts.

## Branch

```text
build/s7-first-real-activations
```

## Recommended first activation package

1. Mobile Browser Game Preview and Playtest Activation
2. Notification Lane Activation
3. Knowledge Mirror Export Activation

## S7A — Mobile Browser Game Preview and Playtest Activation

Real capability:

1. build or update a web/PWA/HTML5 game
2. create or expose a preview/playtest URL
3. run mobile viewport proof
4. run tap/swipe/game-input replay
5. capture screenshots
6. capture console/network failures
7. emit mobile QA report
8. give the user a phone playtest link

Required proof:

```text
proof-mobile
proof-browser
preview/playtest link artifact
mobile QA artifact index
```

## S7B — Notification Lane Activation

Real capability:

1. send a real notification when CI passes, fails, phase completes, or a user decision is needed
2. support dry-run proof when no webhook secret is configured
3. prevent sensitive content from being sent

Required proof:

```text
notification dry-run green
real dispatch green only when explicit secret is configured
notification content redaction green
```

## S7C — Knowledge Mirror Export Activation

Real capability:

1. export public-safe docs/reports/proof summaries into a mirror bundle
2. generate a semantic snapshot
3. run redaction check
4. upload the bundle as a GitHub Actions artifact

External Drive/Notebook upload is a later optional activation and must not use browser-session token hacks.

Required proof:

```text
mirror bundle artifact exists
semantic snapshot artifact exists
redaction pass green
```

## Later S7 tracks

1. Gemini Worker Lane Activation
2. Document/Web Ingest Activation
3. Vercel Preview Adapter Activation
4. Supabase Read-Only Schema Inspect Activation
5. Telemetry Intake Activation
6. Git Worker Activation
7. Android Emulator QA Activation

## Acceptance

S7 passes when activated lanes perform real work, docs state exactly which lanes are active, and all related proof workflows are green.
