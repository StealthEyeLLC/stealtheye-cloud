# Browser Body

## Purpose

The Browser Body is StealthEye Cloud's no-local observation lane for web, app, docs, demo, and visual proof tasks.

## S2 scope

S2 establishes:

1. Playwright browser proof workflow
2. deterministic inline browser fixture
3. screenshot capture
4. console error capture
5. network failure capture
6. DOM sketch capture
7. browser artifact index
8. browser artifact validation
9. browser proof artifact upload

## Runtime

The browser body runs in GitHub Actions using public standard runners.

## Artifacts

S2 browser proof emits:

1. `.stealtheye/browser/artifacts/browser-summary.json`
2. `.stealtheye/browser/artifacts/browser-summary.md`
3. `.stealtheye/browser/artifacts/browser-console-errors.json`
4. `.stealtheye/browser/artifacts/browser-network-failures.json`
5. `.stealtheye/browser/artifacts/browser-dom-sketch.json`
6. `.stealtheye/browser/artifacts/browser-proof.png`

## Boundary

S2 does not use paid browser cloud, secrets, private data, or local laptop execution.
