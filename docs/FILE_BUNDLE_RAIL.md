# File Bundle Rail

## Purpose

The File Bundle Rail validates repo-relative file-set changes before they become executable work.

S1 establishes public-safe validation rules for bundle intent and path shape.

## Executable crate

```text
crates/secloud-file-bundle
```

## S1 validation posture

Reject unsafe path shapes, private material targets, forbidden root files, and non-repo-relative paths.

## Future expansion

S2+ can add richer file operations, hashes, rollback journals, sandbox applies, and workflow dispatch integration.
