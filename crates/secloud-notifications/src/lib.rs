//! S6 notification readiness contracts for StealthEye Cloud.
//!
//! This crate models notification lane readiness without dispatching messages,
//! touching secrets, or contacting external providers.

pub const NOTIFICATION_PACKET_SCHEMAS: &[&str] =
    &["NotificationReadinessV0", "NotificationDispatchPolicyV0"];

pub const REQUIRED_NOTIFICATION_CHECKS: &[&str] = &[
    "provider_declared",
    "dry_run_available",
    "no_secret_required_for_validation",
    "delivery_scope_declared",
    "failure_surface_declared",
];

pub fn is_notification_schema(name: &str) -> bool {
    NOTIFICATION_PACKET_SCHEMAS.contains(&name)
}

pub fn has_notification_check(check: &str) -> bool {
    REQUIRED_NOTIFICATION_CHECKS.contains(&check)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn notification_readiness_requires_dry_run() {
        assert!(has_notification_check("dry_run_available"));
    }

    #[test]
    fn notification_readiness_does_not_require_secret_for_validation() {
        assert!(has_notification_check("no_secret_required_for_validation"));
    }
}
