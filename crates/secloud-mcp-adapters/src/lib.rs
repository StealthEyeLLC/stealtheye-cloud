//! S6 MCP adapter readiness contracts for StealthEye Cloud.
//!
//! This crate describes adapter registry, candidate catalog, lifecycle, and
//! descriptor integrity rules. It is contract-only and cannot execute tools.

pub const MCP_ADAPTER_PACKET_SCHEMAS: &[&str] = &[
    "McpAdapterRegistryV0",
    "AdapterTypeStateV0",
    "AdapterDescriptorIntegrityV0",
    "AdapterCandidateCatalogV0",
    "AdapterRiskScoreV0",
];

pub const ADAPTER_TYPE_STATES: &[&str] = &[
    "candidate",
    "contract_only",
    "quarantined",
    "ready_for_activation_review",
    "rejected",
];

pub const NON_EXECUTING_STATES: &[&str] =
    &["candidate", "contract_only", "quarantined", "rejected"];

pub const DESCRIPTOR_INTEGRITY_FIELDS: &[&str] = &[
    "name",
    "version",
    "capability_digest",
    "schema_digest",
    "declared_permissions",
    "source_digest",
];

pub const RISK_FACTORS: &[&str] = &[
    "writes_files",
    "calls_network",
    "requires_secret",
    "changes_repository",
    "untrusted_descriptor",
    "mutable_remote_capability",
];

pub fn is_mcp_adapter_schema(name: &str) -> bool {
    MCP_ADAPTER_PACKET_SCHEMAS.contains(&name)
}

pub fn is_known_type_state(state: &str) -> bool {
    ADAPTER_TYPE_STATES.contains(&state)
}

pub fn is_non_executing_state(state: &str) -> bool {
    NON_EXECUTING_STATES.contains(&state)
}

pub fn has_descriptor_integrity_field(field: &str) -> bool {
    DESCRIPTOR_INTEGRITY_FIELDS.contains(&field)
}

pub fn has_risk_factor(factor: &str) -> bool {
    RISK_FACTORS.contains(&factor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lifecycle_blocks_contract_only_execution() {
        assert!(is_non_executing_state("contract_only"));
        assert!(is_non_executing_state("quarantined"));
    }

    #[test]
    fn descriptor_integrity_includes_digests() {
        assert!(has_descriptor_integrity_field("capability_digest"));
        assert!(has_descriptor_integrity_field("schema_digest"));
    }

    #[test]
    fn risk_catalog_marks_mutable_remote_capability() {
        assert!(has_risk_factor("mutable_remote_capability"));
    }
}
