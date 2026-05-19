//! S6 gateway readiness contracts for StealthEye Cloud.
//!
//! This crate is contract-only. It models transport, session, origin,
//! gateway, and backpressure policy readiness. It does not open sockets,
//! call providers, or activate external services.

pub const GATEWAY_PACKET_SCHEMAS: &[&str] = &[
    "GatewayPolicyV0",
    "GatewayTransportPolicyV0",
    "GatewaySessionPolicyV0",
    "GatewayOriginPolicyV0",
    "BackpressurePolicyV0",
];

pub const REQUIRED_GATEWAY_BOUNDARIES: &[&str] = &[
    "transport_declared",
    "session_scoped",
    "origin_checked",
    "authority_declared",
    "no_live_external_activation",
];

pub const REQUIRED_TRANSPORT_REQUIREMENTS: &[&str] = &[
    "declared_protocol",
    "bounded_payload",
    "explicit_origin",
    "deterministic_error_surface",
];

pub const REQUIRED_BACKPRESSURE_CONTROLS: &[&str] = &[
    "retry_budget",
    "rate_budget",
    "token_budget",
    "loop_cutoff",
    "degraded_mode",
];

pub fn is_gateway_schema(name: &str) -> bool {
    GATEWAY_PACKET_SCHEMAS.contains(&name)
}

pub fn has_gateway_boundary(name: &str) -> bool {
    REQUIRED_GATEWAY_BOUNDARIES.contains(&name)
}

pub fn has_transport_requirement(name: &str) -> bool {
    REQUIRED_TRANSPORT_REQUIREMENTS.contains(&name)
}

pub fn has_backpressure_control(name: &str) -> bool {
    REQUIRED_BACKPRESSURE_CONTROLS.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gateway_inventory_contains_policy_schema() {
        assert!(is_gateway_schema("GatewayPolicyV0"));
    }

    #[test]
    fn gateway_requires_no_live_activation() {
        assert!(has_gateway_boundary("no_live_external_activation"));
    }

    #[test]
    fn backpressure_stops_retry_storms() {
        assert!(has_backpressure_control("retry_budget"));
        assert!(has_backpressure_control("loop_cutoff"));
    }
}
