//! S5 release candidate validation for StealthEye Cloud.

pub const RELEASE_PACKET_SCHEMAS: &[&str] = &[
    "ReleaseCandidateV0",
    "ReleaseEvidenceIndexV0",
    "PublicProofSummaryV0",
    "ReleaseBlockerV0",
];

pub const REQUIRED_RELEASE_ARTIFACTS: &[&str] = &[
    "docs/S5_FINAL_REPORT.md",
    "docs/RELEASE_CANDIDATE.md",
    "docs/FIRST_E2E_MISSION.md",
    "public/proof/index.html",
];

pub fn is_release_schema(name: &str) -> bool {
    RELEASE_PACKET_SCHEMAS.contains(&name)
}

pub fn is_required_artifact(path: &str) -> bool {
    REQUIRED_RELEASE_ARTIFACTS.contains(&path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_release_candidate_schema() {
        assert!(is_release_schema("ReleaseCandidateV0"));
    }

    #[test]
    fn requires_final_report_artifact() {
        assert!(is_required_artifact("docs/S5_FINAL_REPORT.md"));
    }
}
