//! Packet inventory for StealthEye Cloud S0.
//!
//! S0 keeps packet definitions dependency-light. Later drops may add full
//! serde-backed structs, but the public names and validation posture are
//! established here.

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
    fn forbidden_file_inventory_blocks_tool_adapters() {
        assert!(FORBIDDEN_ROOT_FILES.contains(&"CLAUDE.md"));
        assert!(FORBIDDEN_ROOT_FILES.contains(&".cursorrules"));
        assert!(FORBIDDEN_ROOT_FILES.contains(&"soul.md"));
    }
}
