//! Worker and background capability registry for StealthEye Cloud S3.

pub const WORKER_PACKET_SCHEMAS: &[&str] = &[
    "CodexTaskPacketV0",
    "CodexResultImportV0",
    "CodexUsageSnapshotV0",
    "DeepResearchTaskPacketV0",
    "ResearchResultImportV0",
    "AgentModeTaskPacketV0",
    "FeatureAvailabilityCheckV0",
];

pub const REAL_BACKGROUND_SURFACES: &[&str] = &[
    "chatgpt.active_tab",
    "chatgpt.skills",
    "chatgpt.tasks",
    "codex.user_launched_worker",
    "github.actions",
    "deep_research.user_started",
    "agent_mode.user_started",
    "apps.connectors.when_available",
    "relay.seal.handoff",
];

pub const HIDDEN_AUTONOMY_CLAIMS_BLOCKED: &[&str] = &[
    "secretly_create_tabs",
    "silently_start_deep_research",
    "silently_start_agent_mode",
    "spawn_paid_compute_without_approval",
];

pub fn is_real_surface(surface: &str) -> bool {
    REAL_BACKGROUND_SURFACES.contains(&surface)
}

pub fn is_worker_schema(schema: &str) -> bool {
    WORKER_PACKET_SCHEMAS.contains(&schema)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_github_actions_surface() {
        assert!(is_real_surface("github.actions"));
    }

    #[test]
    fn includes_codex_task_packet() {
        assert!(is_worker_schema("CodexTaskPacketV0"));
    }

    #[test]
    fn blocks_hidden_autonomy_claims() {
        assert!(HIDDEN_AUTONOMY_CLAIMS_BLOCKED.contains(&"silently_start_deep_research"));
    }
}
