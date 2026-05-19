//! Public proof viewer contract validation for StealthEye Cloud S4.

pub const PROOF_VIEWER_PACKET_SCHEMAS: &[&str] = &[
    "ProofCanvasManifestV0",
    "ProofPanelV0",
    "ProofArtifactRefV0",
    "ProofViewerBuildV0",
];

pub const PROOF_PANEL_KINDS: &[&str] = &[
    "mission",
    "ci",
    "browser",
    "seal",
    "relay",
    "worker",
    "decision",
];

pub fn is_viewer_schema(name: &str) -> bool {
    PROOF_VIEWER_PACKET_SCHEMAS.contains(&name)
}

pub fn is_panel_kind(name: &str) -> bool {
    PROOF_PANEL_KINDS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_proof_canvas_manifest_schema() {
        assert!(is_viewer_schema("ProofCanvasManifestV0"));
    }

    #[test]
    fn includes_browser_panel_kind() {
        assert!(is_panel_kind("browser"));
    }
}
