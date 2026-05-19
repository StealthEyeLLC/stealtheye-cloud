//! Packet inventory for StealthEye Cloud.
//!
//! The public schema inventory is intentionally dependency-light. It gives CI
//! a fast way to prove that the repo exposes every packet contract required by
//! the current build spine.

pub const REQUIRED_PACKET_SCHEMAS: &[&str] = &[
    "StealthEyeRelayV0",
    "StealthEyeSealV0",
    "MissionPacketV0",
    "MissionApprovalV0",
    "ActionPacketV0",
    "ObservationPacketV0",
    "FailurePacketV0",
    "RepairPacketV0",
    "ContinuationPacketV0",
    "ResultPacketV0",
    "BoundaryPacketV0",
    "BrowserObservationPacketV0",
    "CodexTaskPacketV0",
    "CapabilityRegistryV0",
    "ToolRegistryV0",
    "MissionExecutorDispatchV0",
    "AuthorityQueueV0",
    "OutputShelfV0",
    "AtomicDropPackageV0",
    "FileSetBundleV0",
    "DropPlanV0",
    "DropValidationV0",
    "DropApplyReportV0",
    "GitHubEvidencePackV0",
    "PrEvidenceCardV0",
    "FailureCardV0",
    "OperatorStateV0",
    "StealthEyeAutonomyStatusV0",
    "BrowserRunRequestV0",
    "BrowserArtifactIndexV0",
    "BrowserRepairPacketV0",
    "BrowserReplayPackV0",
    "BrowserRouteSmokeV0",
    "BrowserConsoleFailureV0",
    "BrowserNetworkFailureV0",
    "BrowserScreenshotRefV0",
    "BrowserDomSketchV0",
    "VisualEvidenceCardV0",
    "ExplorationToReplayCandidateV0",
    "ToolIdentityV0",
    "ToolCapabilityV0",
    "ToolPermissionEnvelopeV0",
    "ToolResultEnvelopeV0",
    "ToolHealthV0",
    "ToolCapabilitySearchV0",
    "ToolPackManifestV0",
    "StealthEyeCapabilitiesV0",
    "StealthEyeWorkersV0",
    "CodexResultImportV0",
    "CodexUsageSnapshotV0",
    "DeepResearchTaskPacketV0",
    "ResearchResultImportV0",
    "AgentModeTaskPacketV0",
    "FeatureAvailabilityCheckV0",
    "FeedbackSignalV0",
    "PatternCandidateV0",
    "SkillCandidateV0",
    "SkillPromotionDecisionV0",
    "SkillTemplateIndexV0",
    "TemplateToSkillCompilerV0",
    "PastSessionSearchV0",
    "SearchCorpusManifestV0",
    "SearchResultCardV0",
    "SearchImportDecisionV0",
    "HypothesisRaceV0",
    "CandidateBranchV0",
    "CandidateReducerV0",
    "RaceDecisionV0",
    "ProofCanvasManifestV0",
    "ProofPanelV0",
    "ProofArtifactRefV0",
    "ProofViewerBuildV0",
];

pub const FORBIDDEN_ROOT_FILES: &[&str] = &[
    "CLAUDE.md",
    ".github/copilot-instructions.md",
    ".cursorrules",
    "soul.md",
    "MEMORY.md",
    "rules.md",
];

pub const REQUIRED_ROOT_FILES: &[&str] = &[
    "AGENTS.md",
    "llms.txt",
    "llms-full.txt",
    "AGENT_REPO_MAP.md",
    "STEALTHEYE_ACTIVE.md",
    "STEALTHEYE_DECISIONS.md",
    "STEALTHEYE_RELAY.md",
    "STEALTHEYE_RELAY.json",
    "STEALTHEYE_SEAL.json",
    "NEXT_ACTION.md",
];

pub fn is_required_schema(name: &str) -> bool {
    REQUIRED_PACKET_SCHEMAS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_schema_inventory_contains_relay_and_seal() {
        assert!(is_required_schema("StealthEyeRelayV0"));
        assert!(is_required_schema("StealthEyeSealV0"));
    }

    #[test]
    fn required_schema_inventory_contains_s3_control_and_worker_contracts() {
        assert!(is_required_schema("ToolPermissionEnvelopeV0"));
        assert!(is_required_schema("StealthEyeWorkersV0"));
        assert!(is_required_schema("FeatureAvailabilityCheckV0"));
    }

    #[test]
    fn required_schema_inventory_contains_s4_learning_search_race_and_canvas_contracts() {
        assert!(is_required_schema("SkillCandidateV0"));
        assert!(is_required_schema("PastSessionSearchV0"));
        assert!(is_required_schema("HypothesisRaceV0"));
        assert!(is_required_schema("ProofCanvasManifestV0"));
    }

    #[test]
    fn forbidden_file_inventory_blocks_tool_adapters() {
        assert!(FORBIDDEN_ROOT_FILES.contains(&"CLAUDE.md"));
        assert!(FORBIDDEN_ROOT_FILES.contains(&".cursorrules"));
        assert!(FORBIDDEN_ROOT_FILES.contains(&"soul.md"));
    }
}
