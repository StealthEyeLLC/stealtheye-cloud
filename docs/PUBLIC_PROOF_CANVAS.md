# Public Proof Canvas

## Purpose

The Public Proof Canvas is a lightweight public proof viewer surface for StealthEye Cloud evidence.

## S4 scope

S4 establishes representation and validation contracts for:

1. `ProofCanvasManifestV0`
2. `ProofPanelV0`
3. `ProofArtifactRefV0`
4. `ProofViewerBuildV0`

## Executable crate

```text
crates/secloud-proof-viewer
```

## Panel kinds

1. mission
2. CI
3. browser
4. Seal
5. Relay
6. worker
7. decision

## Boundary

The proof canvas is presentation. It does not replace GitHub Actions, browser artifacts, Relay, or Seal. It summarizes and links public-safe evidence.
