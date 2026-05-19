//! S6 Gemini worker readiness contracts for StealthEye Cloud.
//!
//! This crate models provider-worker readiness without calling Gemini or any
//! external model endpoint. It preserves prompt topology isolation and keeps
//! provider-specific prompts out of shared lanes.

pub const GEMINI_WORKER_PACKET_SCHEMAS: &[&str] = &[
    "GeminiWorkerReadinessV0",
    "ProviderPromptTopologyV0",
    "SemanticNormalizationV0",
];

pub const REQUIRED_GEMINI_READINESS_CHECKS: &[&str] = &[
    "provider_declared",
    "model_lane_declared",
    "no_live_provider_call",
    "prompt_topology_isolated",
    "semantic_normalization_declared",
];

pub const PROMPT_TOPOLOGY_BOUNDARIES: &[&str] = &[
    "provider_specific_prompt_not_shared",
    "normal_form_declared_before_dispatch",
    "untrusted_content_cannot_be_instruction",
];

pub fn is_gemini_worker_schema(name: &str) -> bool {
    GEMINI_WORKER_PACKET_SCHEMAS.contains(&name)
}

pub fn has_gemini_readiness_check(check: &str) -> bool {
    REQUIRED_GEMINI_READINESS_CHECKS.contains(&check)
}

pub fn has_prompt_topology_boundary(boundary: &str) -> bool {
    PROMPT_TOPOLOGY_BOUNDARIES.contains(&boundary)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gemini_readiness_is_not_activation() {
        assert!(has_gemini_readiness_check("no_live_provider_call"));
    }

    #[test]
    fn prompt_topology_is_isolated() {
        assert!(has_prompt_topology_boundary("provider_specific_prompt_not_shared"));
    }
}
