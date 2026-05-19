//! S5 hardening validation for StealthEye Cloud.

pub const HARDENING_PACKET_SCHEMAS: &[&str] = &[
    "HardeningCheckV0",
    "ReleaseReadinessV0",
    "PublicSafetyReviewV0",
    "CostGuardSnapshotV0",
    "ProofGateMatrixV0",
];

pub const REQUIRED_HARDENING_CHECKS: &[&str] = &[
    "no_forbidden_files",
    "schema_inventory_complete",
    "proof_fast_green",
    "proof_full_green",
    "proof_browser_green",
    "windows_targeted_green",
    "no_local_required",
    "no_paid_compute_required",
];

pub fn is_hardening_schema(name: &str) -> bool {
    HARDENING_PACKET_SCHEMAS.contains(&name)
}

pub fn has_required_check(name: &str) -> bool {
    REQUIRED_HARDENING_CHECKS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn includes_release_readiness_schema() {
        assert!(is_hardening_schema("ReleaseReadinessV0"));
    }

    #[test]
    fn includes_no_paid_compute_check() {
        assert!(has_required_check("no_paid_compute_required"));
    }
}
