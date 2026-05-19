//! S6 repository worker readiness contracts for StealthEye Cloud.
//!
//! This crate models repository worker readiness without mutating repository
//! state. S6 proves contracts only; activation belongs to later phases.

pub const REPO_WORKER_PACKET_SCHEMAS: &[&str] = &[
    "GitWorkerReadinessV0",
    "RepoWorkerPermissionEnvelopeV0",
];

pub const REQUIRED_REPO_WORKER_CHECKS: &[&str] = &[
    "read_only_default",
    "branch_scope_declared",
    "mutation_requires_later_activation",
    "evidence_pack_required",
    "no_unreviewed_push",
];

pub fn is_repo_worker_schema(name: &str) -> bool {
    REPO_WORKER_PACKET_SCHEMAS.contains(&name)
}

pub fn has_repo_worker_check(check: &str) -> bool {
    REQUIRED_REPO_WORKER_CHECKS.contains(&check)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repo_worker_is_read_only_by_default_in_s6() {
        assert!(has_repo_worker_check("read_only_default"));
    }

    #[test]
    fn repo_worker_mutation_is_not_s6_activation() {
        assert!(has_repo_worker_check("mutation_requires_later_activation"));
    }
}
