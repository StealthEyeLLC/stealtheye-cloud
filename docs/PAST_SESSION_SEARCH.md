# Past-Session Search

## Purpose

Past-Session Search lets StealthEye Cloud recover execution-relevant context from Relay, Seal, Active, Decisions, Skills, worker results, browser artifacts, CI proof, and docs.

## S4 scope

S4 establishes representation and validation contracts for:

1. `PastSessionSearchV0`
2. `SearchCorpusManifestV0`
3. `SearchResultCardV0`
4. `SearchImportDecisionV0`

## Executable crate

```text
crates/secloud-search
```

## Search corpora

1. relay
2. seal
3. active
4. decisions
5. skills
6. worker-results
7. browser-artifacts
8. ci-proof
9. docs

## Boundary

Search is a retrieval and import-decision layer. It does not make retrieved context authoritative by itself. Imported results must be evaluated against current repo state, current proof, and latest Seal.
