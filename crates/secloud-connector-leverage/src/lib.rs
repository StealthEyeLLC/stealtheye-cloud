//! S12 connector leverage contracts for StealthEye Cloud.
//!
//! This crate defines high-frequency connector surfaces that make proof, mission,
//! repo, and safety actions explicit. Connector output remains data, not
//! instruction authority. Capability manifests must record both what the connector
//! can do and what it cannot do.

pub const CONNECTOR_LEVERAGE_PACKET_SCHEMAS: &[&str] = &[
    "StealthEyeConnectorLeverageLayerV0",
    "ConnectorProofSurfaceV0",
    "ConnectorMissionSurfaceV0",
    "ConnectorRepoSurfaceV0",
    "ConnectorBoundaryClassifierV0",
    "StealthEyeCommandInboxV0",
    "IssueCommentCommandDispatchV0",
    "CommandParserV0",
    "CommandAuthorizationCheckV0",
    "CommandResultOutboxV0",
    "CommandResultV0",
    "CommandHistoryV0",
    "CommandProofReceiptV0",
    "WorkflowDispatchBusV0",
    "RepositoryDispatchBusV0",
    "DispatchRunCorrelationV0",
    "DispatchResultReceiptV0",
    "ToolDescriptorSnapshotV0",
    "ToolDescriptorCommitPinV0",
    "ToolDescriptorHashV0",
    "McpToolManifestV0",
    "ToolOutputTaintPolicyV0",
    "PromptToScriptFirewallV0",
    "LeastPrivilegeJobPermissionV0",
];

pub const PROOF_SURFACE_ACTIONS: &[&str] = &[
    "dispatch_workflow",
    "dispatch_workflow_and_wait",
    "prove_main_head",
    "run_required_proof_set",
    "list_workflows",
    "list_workflow_runs_filtered",
    "correlate_workflow_dispatch",
    "rerun_and_wait",
    "detect_stale_green",
    "required_check_resolver",
];

pub const MISSION_SURFACE_ACTIONS: &[&str] = &[
    "start_mission_executor",
    "get_mission_status",
    "get_mission_result",
    "resume_mission",
    "get_approval_count_report",
    "fetch_mission_journal",
    "fetch_mission_result",
];

pub const REPO_SURFACE_ACTIONS: &[&str] = &[
    "fetch_context_pack",
    "fetch_changed_surface",
    "apply_file_batch",
    "create_or_reuse_branch",
    "create_or_reuse_pr",
    "update_handoff_bundle",
    "get_pr_check_summary",
    "download_failed_logs_bundle",
    "triage_failed_workflows",
];

pub const SAFETY_SURFACE_ACTIONS: &[&str] = &[
    "get_connector_capabilities",
    "get_repo_actions_permissions",
    "get_branch_rules_and_merge_blockers",
    "classify_github_boundary",
    "permission_upgrade_request",
];

pub const COMMANDS: &[&str] = &[
    "prove-main",
    "run-proof",
    "run-proof-set",
    "start-mission",
    "status",
];

pub const UNTRUSTED_SOURCES: &[&str] = &[
    "issue_body",
    "pr_body",
    "comment",
    "workflow_log",
    "external_tool_output",
    "mcp_output",
    "browser_observation",
];

pub const FORBIDDEN_SINKS: &[&str] = &[
    "model_output_to_shell",
    "raw_token_passthrough",
    "untrusted_text_as_instruction",
    "omnibus_permission_scope",
    "unbounded_mcp_tool_authority",
    "ssrf_metadata_or_redirect",
    "secret_elicitation",
    "browser_cookie_session_token_automation",
    "github_permission_bypass",
];

fn has(values: &[&str], value: &str) -> bool {
    values.contains(&value)
}

pub fn is_connector_schema(name: &str) -> bool {
    has(CONNECTOR_LEVERAGE_PACKET_SCHEMAS, name)
}

pub fn is_proof_surface_action(action: &str) -> bool {
    has(PROOF_SURFACE_ACTIONS, action)
}

pub fn is_mission_surface_action(action: &str) -> bool {
    has(MISSION_SURFACE_ACTIONS, action)
}

pub fn is_repo_surface_action(action: &str) -> bool {
    has(REPO_SURFACE_ACTIONS, action)
}

pub fn is_safety_surface_action(action: &str) -> bool {
    has(SAFETY_SURFACE_ACTIONS, action)
}

pub fn is_closed_command(command: &str) -> bool {
    has(COMMANDS, command)
}

pub fn is_untrusted_source(source: &str) -> bool {
    has(UNTRUSTED_SOURCES, source)
}

pub fn is_forbidden_sink(sink: &str) -> bool {
    has(FORBIDDEN_SINKS, sink)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectorCommandVerdict {
    AcceptedClosedCommand,
    RejectedUnknownCommand,
    RejectedUnsafeSink,
}

pub fn classify_command(command: &str, sink: &str) -> ConnectorCommandVerdict {
    if is_forbidden_sink(sink) {
        return ConnectorCommandVerdict::RejectedUnsafeSink;
    }
    if !is_closed_command(command) {
        return ConnectorCommandVerdict::RejectedUnknownCommand;
    }
    ConnectorCommandVerdict::AcceptedClosedCommand
}

#[cfg(test)]
mod tests {
    use super::*;

    const MANIFEST_SCHEMA: &str =
        include_str!("../../../schemas/ConnectorCapabilityManifestV0.schema.json");
    const DISPATCH_SCHEMA: &str =
        include_str!("../../../schemas/IssueCommentCommandDispatchV0.schema.json");
    const FIREWALL_SCHEMA: &str =
        include_str!("../../../schemas/PromptToScriptFirewallV0.schema.json");

    #[test]
    fn connector_schema_files_are_materialized() {
        assert!(MANIFEST_SCHEMA.contains("ConnectorCapabilityManifestV0"));
        assert!(DISPATCH_SCHEMA.contains("IssueCommentCommandDispatchV0"));
        assert!(FIREWALL_SCHEMA.contains("PromptToScriptFirewallV0"));
    }

    #[test]
    fn high_frequency_surfaces_are_closed() {
        assert!(is_proof_surface_action("prove_main_head"));
        assert!(is_mission_surface_action("resume_mission"));
        assert!(is_repo_surface_action("create_or_reuse_pr"));
        assert!(is_safety_surface_action("classify_github_boundary"));
        assert!(!is_proof_surface_action("arbitrary_shell"));
    }

    #[test]
    fn command_parser_rejects_unknown_or_unsafe_sinks() {
        assert_eq!(
            classify_command("prove-main", "dispatch_workflow"),
            ConnectorCommandVerdict::AcceptedClosedCommand
        );
        assert_eq!(
            classify_command("curl-secret", "dispatch_workflow"),
            ConnectorCommandVerdict::RejectedUnknownCommand
        );
        assert_eq!(
            classify_command("prove-main", "model_output_to_shell"),
            ConnectorCommandVerdict::RejectedUnsafeSink
        );
    }

    #[test]
    fn untrusted_sources_remain_data_only() {
        assert!(is_untrusted_source("comment"));
        assert!(is_untrusted_source("workflow_log"));
        assert!(is_forbidden_sink("untrusted_text_as_instruction"));
        assert!(is_forbidden_sink("github_permission_bypass"));
    }
}
