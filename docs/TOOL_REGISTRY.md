# Tool Registry

## Purpose

The Tool Registry is the closed list of public-safe high-level actions available to StealthEye Cloud.

## S3 implementation

S3 implements registry logic in:

```text
crates/secloud-control
```

## Rules

1. Only named public-safe high-level tools are allowed.
2. Raw primitive tools are blocked.
3. Unknown tool names are not allowed.
4. Tool results must return summaries, not hidden authority.
5. Future adapters must normalize through this registry.

## Schemas

1. `ToolIdentityV0`
2. `ToolCapabilityV0`
3. `ToolPermissionEnvelopeV0`
4. `ToolResultEnvelopeV0`
5. `ToolHealthV0`
6. `ToolRegistryV0`
7. `ToolCapabilitySearchV0`
8. `ToolPackManifestV0`
