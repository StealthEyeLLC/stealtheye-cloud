# Visual Evidence

## Purpose

Visual Evidence is the public-safe artifact layer for browser proof.

S2 establishes screenshot-backed evidence without requiring a local laptop or paid browser cloud.

## S2 outputs

The browser proof lane emits:

1. browser summary JSON
2. browser summary Markdown
3. console error JSON
4. network failure JSON
5. DOM sketch JSON
6. screenshot PNG

## Schema

S2 introduces `VisualEvidenceCardV0` for referencing visual artifacts in future proof summaries.

## Boundary

Visual evidence is derived proof. StealthEye Seal remains the checkpoint artifact. Screenshots and browser artifacts are inputs to proof, not separate receipt families.
