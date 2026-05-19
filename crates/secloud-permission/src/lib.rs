//! S6 permission boundary contracts for StealthEye Cloud.
//!
//! This crate models external authority-policy readiness without reading
//! credentials, storing secrets, or rehydrating browser sessions.

pub const PERMISSION_PACKET_SCHEMAS: &[&str] = &[
    "ExternalAuthPolicyV0",
    "CredentialBoundaryV0",
    "SessionMaterialBanV0",
];

pub const REQUIRED_PERMISSION_RULES: &[&str] = &[
    "no_browser_cookie_extraction",
    "no_consumer_session_rehydration",
    "no_two_factor_bypass",
    "no_billing_bypass_framing",
    "secret_material_never_required_for_readiness",
];

pub fn is_permission_schema(name: &str) -> bool {
    PERMISSION_PACKET_SCHEMAS.contains(&name)
}

pub fn has_permission_rule(rule: &str) -> bool {
    REQUIRED_PERMISSION_RULES.contains(&rule)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permission_rules_block_session_rehydration() {
        assert!(has_permission_rule("no_browser_cookie_extraction"));
        assert!(has_permission_rule("no_consumer_session_rehydration"));
    }

    #[test]
    fn permission_rules_do_not_need_secret_material_for_readiness() {
        assert!(has_permission_rule("secret_material_never_required_for_readiness"));
    }
}
