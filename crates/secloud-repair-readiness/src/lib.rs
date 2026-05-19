//! S6 repair/remediation readiness contracts for StealthEye Cloud.
//!
//! This crate models Remediator readiness without executing repairs. S6 proves
//! intake, permission, reproduction, taxonomy, proof-plan, and report contracts;
//! S8 is the later activation phase.

pub const REPAIR_READINESS_PACKET_SCHEMAS: &[&str] = &[
    "RemediatorReadinessV0",
    "RemediationIntakeV0",
    "RemediationPermissionsV0",
    "RemediationReproductionV0",
    "RemediationFailureTaxonomyV0",
    "RemediationProofPlanV0",
    "RemediationReportV0",
];

pub const REQUIRED_REPAIR_MODULES: &[&str] = &[
    "intake",
    "permissions",
    "reproduction",
    "failure_taxonomy",
    "proof_plan",
    "report",
];

pub const REQUIRED_REPAIR_BOUNDARIES: &[&str] = &[
    "no_patch_without_reproduction_plan",
    "no_unbounded_command_execution",
    "no_secret_required_for_readiness",
    "diagnosis_is_not_completed_repair",
    "activation_deferred_to_s8",
];

pub fn is_repair_readiness_schema(name: &str) -> bool {
    REPAIR_READINESS_PACKET_SCHEMAS.contains(&name)
}

pub fn has_repair_module(module: &str) -> bool {
    REQUIRED_REPAIR_MODULES.contains(&module)
}

pub fn has_repair_boundary(boundary: &str) -> bool {
    REQUIRED_REPAIR_BOUNDARIES.contains(&boundary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repair_readiness_requires_reproduction_plan() {
        assert!(has_repair_module("reproduction"));
        assert!(has_repair_boundary("no_patch_without_reproduction_plan"));
    }

    #[test]
    fn repair_activation_is_deferred_to_s8() {
        assert!(has_repair_boundary("activation_deferred_to_s8"));
    }
}
