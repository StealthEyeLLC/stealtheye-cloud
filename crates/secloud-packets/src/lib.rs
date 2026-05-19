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
    "HardeningCheckV0",
    "ReleaseReadinessV0",
    "PublicSafetyReviewV0",
    "CostGuardSnapshotV0",
    "ProofGateMatrixV0",
    "ReleaseCandidateV0",
    "ReleaseEvidenceIndexV0",
    "PublicProofSummaryV0",
    "ReleaseBlockerV0",
    "EndToEndMissionV0",
    "EndToEndStepV0",
    "EndToEndProofSummaryV0",
    "MissionCompletionGateV0",
    "GatewayPolicyV0",
    "GatewayTransportPolicyV0",
    "GatewaySessionPolicyV0",
    "GatewayOriginPolicyV0",
    "BackpressurePolicyV0",
    "McpAdapterRegistryV0",
    "AdapterTypeStateV0",
    "AdapterDescriptorIntegrityV0",
    "AdapterCandidateCatalogV0",
    "AdapterRiskScoreV0",
    "GeminiWorkerReadinessV0",
    "ModelTopologyBoundaryV0",
    "SemanticNormalizationV0",
    "KnowledgeMirrorReadinessV0",
    "KnowledgeMirrorRedactionV0",
    "SemanticSnapshotV0",
    "NotificationReadinessV0",
    "NotificationDispatchPolicyV0",
    "ExternalAuthPolicyV0",
    "CredentialBoundaryV0",
    "SessionMaterialBanV0",
    "DataTaintPolicyV0",
    "InjectionIsolationPolicyV0",
    "WorkflowGuardPolicyV0",
    "DocumentIngestPolicyV0",
    "WebIngestPolicyV0",
    "ProductionAdapterContractV0",
    "DatabaseBoundaryV0",
    "TelemetryAdapterContractV0",
    "TelemetryRedactionPolicyV0",
    "GitWorkerReadinessV0",
    "RepoWorkerPermissionEnvelopeV0",
    "MobileQaReadinessV0",
    "GameQaReadinessV0",
    "MobilePreviewPolicyV0",
    "RemediatorReadinessV0",
    "RemediationIntakeV0",
    "RemediationPermissionsV0",
    "RemediationRealityMapV0",
    "RemediationCommandDiscoveryV0",
    "RemediationEnvironmentV0",
    "RemediationReproductionV0",
    "RemediationFailureTaxonomyV0",
    "RemediationLocalizationV0",
    "RemediationRepairStrategyV0",
    "RemediationPatchTournamentV0",
    "RemediationProofPlanV0",
    "RemediationReportV0",
    "RemediationCommercialV0",
    "RemediatorExecutionReceiptV0",
    "MobilePlaytestActivationV0",
    "NotificationActivationRunV0",
    "KnowledgeMirrorExportV0",
    "S7ActivationProofV0",
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
    fn required_schema_inventory_contains_s5_release_and_e2e_contracts() {
        assert!(is_required_schema("ReleaseReadinessV0"));
        assert!(is_required_schema("ReleaseCandidateV0"));
        assert!(is_required_schema("EndToEndMissionV0"));
        assert!(is_required_schema("MissionCompletionGateV0"));
    }

    #[test]
    fn required_schema_inventory_contains_s6_gateway_contracts() {
        assert!(is_required_schema("GatewayPolicyV0"));
        assert!(is_required_schema("McpAdapterRegistryV0"));
        assert!(is_required_schema("ModelTopologyBoundaryV0"));
        assert!(is_required_schema("ExternalAuthPolicyV0"));
        assert!(is_required_schema("GitWorkerReadinessV0"));
        assert!(is_required_schema("RemediatorReadinessV0"));
    }

    #[test]
    fn required_schema_inventory_contains_s7_activation_contracts() {
        assert!(is_required_schema("MobilePlaytestActivationV0"));
        assert!(is_required_schema("NotificationActivationRunV0"));
        assert!(is_required_schema("KnowledgeMirrorExportV0"));
        assert!(is_required_schema("S7ActivationProofV0"));
    }

    #[test]
    fn required_schema_inventory_contains_s8_remediator_contracts() {
        assert!(is_required_schema("RemediationRealityMapV0"));
        assert!(is_required_schema("RemediationCommandDiscoveryV0"));
        assert!(is_required_schema("RemediationEnvironmentV0"));
        assert!(is_required_schema("RemediationLocalizationV0"));
        assert!(is_required_schema("RemediationRepairStrategyV0"));
        assert!(is_required_schema("RemediationPatchTournamentV0"));
        assert!(is_required_schema("RemediationCommercialV0"));
        assert!(is_required_schema("RemediatorExecutionReceiptV0"));
    }

    #[test]
    fn forbidden_file_inventory_blocks_tool_adapters() {
        assert!(FORBIDDEN_ROOT_FILES.contains(&"CLAUDE.md"));
        assert!(FORBIDDEN_ROOT_FILES.contains(&".cursorrules"));
        assert!(FORBIDDEN_ROOT_FILES.contains(&"soul.md"));
    }
}
