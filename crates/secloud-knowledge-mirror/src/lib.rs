//! S6 knowledge mirror readiness contracts for StealthEye Cloud.
//!
//! This crate models export, redaction, and semantic snapshot readiness without
//! reading private data, calling external services, or producing a live mirror.

pub const KNOWLEDGE_MIRROR_PACKET_SCHEMAS: &[&str] = &[
    "KnowledgeMirrorReadinessV0",
    "KnowledgeMirrorRedactionV0",
    "SemanticSnapshotV0",
];

pub const REQUIRED_MIRROR_CHECKS: &[&str] = &[
    "source_inventory_declared",
    "redaction_policy_declared",
    "semantic_snapshot_declared",
    "private_data_excluded",
    "export_is_not_live_sync",
];

pub const REDACTION_CLASSES: &[&str] = &[
    "secret",
    "private_user_data",
    "client_data",
    "credential_material",
    "session_material",
];

pub fn is_knowledge_mirror_schema(name: &str) -> bool {
    KNOWLEDGE_MIRROR_PACKET_SCHEMAS.contains(&name)
}

pub fn has_mirror_check(check: &str) -> bool {
    REQUIRED_MIRROR_CHECKS.contains(&check)
}

pub fn redacts_class(class_name: &str) -> bool {
    REDACTION_CLASSES.contains(&class_name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mirror_readiness_excludes_private_data() {
        assert!(has_mirror_check("private_data_excluded"));
        assert!(redacts_class("private_user_data"));
    }

    #[test]
    fn mirror_is_not_live_sync() {
        assert!(has_mirror_check("export_is_not_live_sync"));
    }
}
