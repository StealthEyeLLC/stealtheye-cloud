//! S6 guard contracts for StealthEye Cloud.
//!
//! This crate models taint, indirect-injection isolation, workflow guardrails,
//! ingest boundaries, database boundary, telemetry redaction, and production
//! adapter readiness. It is declarative readiness only.

pub const GUARD_PACKET_SCHEMAS: &[&str] = &[
    "DataTaintPolicyV0",
    "InjectionIsolationPolicyV0",
    "WorkflowGuardPolicyV0",
    "DocumentIngestPolicyV0",
    "WebIngestPolicyV0",
    "ProductionAdapterContractV0",
    "DatabaseBoundaryV0",
    "TelemetryAdapterContractV0",
    "TelemetryRedactionPolicyV0",
];

pub const REQUIRED_TAINT_CLASSES: &[&str] = &[
    "untrusted_web_content",
    "untrusted_document_content",
    "user_supplied_text",
    "tool_result_data",
];

pub const REQUIRED_ISOLATION_RULES: &[&str] = &[
    "untrusted_content_is_data",
    "tool_result_cannot_be_instruction",
    "workflow_payload_cannot_mutate_plan",
    "ingest_payload_cannot_escalate_authority",
];

pub const REQUIRED_PRODUCTION_CONTRACTS: &[&str] = &[
    "readiness_only",
    "no_deployment_mutation",
    "no_database_mutation",
    "redaction_required",
    "bounded_output",
];

pub fn is_guard_schema(name: &str) -> bool {
    GUARD_PACKET_SCHEMAS.contains(&name)
}

pub fn has_taint_class(class_name: &str) -> bool {
    REQUIRED_TAINT_CLASSES.contains(&class_name)
}

pub fn has_isolation_rule(rule: &str) -> bool {
    REQUIRED_ISOLATION_RULES.contains(&rule)
}

pub fn has_production_contract(contract: &str) -> bool {
    REQUIRED_PRODUCTION_CONTRACTS.contains(&contract)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn untrusted_content_is_never_instruction() {
        assert!(has_isolation_rule("untrusted_content_is_data"));
        assert!(has_isolation_rule("tool_result_cannot_be_instruction"));
    }

    #[test]
    fn production_contract_blocks_mutation_in_s6() {
        assert!(has_production_contract("no_deployment_mutation"));
        assert!(has_production_contract("no_database_mutation"));
    }
}
