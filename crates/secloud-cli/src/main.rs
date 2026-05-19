use std::env;
use std::fs;
use std::path::Path;

use secloud_build_accelerator::{
    has_batch_repair_rule, has_friction_metric, has_future_phase_contract_rule,
    has_human_attention_rule, has_lifecycle_state, has_merge_handoff_rule,
    has_no_silent_downgrade_rule, has_one_drop_step, has_proof_selection_rule,
    has_recovery_rule, has_registration_guard, has_required_doc, has_required_prompt_artifact,
    has_state_consistency_rule, has_tool_fallback_rule, has_velocity_metric,
    is_boundary_action_class, is_build_accelerator_schema, is_build_accelerator_validation_target,
    is_routine_action_class, BUILD_ACCELERATOR_PACKET_SCHEMAS,
    BUILD_ACCELERATOR_VALIDATION_TARGETS,
};
use secloud_control::{classify_tool_name, ALLOWED_TOOL_NAMES, BLOCKED_TOOL_NAMES};
use secloud_e2e::{has_required_step, is_e2e_schema, E2E_PACKET_SCHEMAS, REQUIRED_E2E_STEPS};
use secloud_gateway::{
    has_backpressure_control, has_gateway_boundary, has_transport_requirement, is_gateway_schema,
    GATEWAY_PACKET_SCHEMAS,
};
use secloud_gemini_worker::{
    has_gemini_readiness_check, has_prompt_topology_boundary, is_gemini_worker_schema,
    GEMINI_WORKER_PACKET_SCHEMAS,
};
use secloud_guard::{
    has_isolation_rule, has_production_contract, has_taint_class, is_guard_schema,
    GUARD_PACKET_SCHEMAS,
};
use secloud_hardening::{
    has_required_check, is_hardening_schema, HARDENING_PACKET_SCHEMAS, REQUIRED_HARDENING_CHECKS,
};
use secloud_hypothesis::{is_hypothesis_schema, is_stop_condition, HYPOTHESIS_PACKET_SCHEMAS};
use secloud_knowledge_mirror::{
    has_mirror_check, is_knowledge_mirror_schema, redacts_class, KNOWLEDGE_MIRROR_PACKET_SCHEMAS,
};
use secloud_learning::{has_promotion_rule, is_learning_schema, LEARNING_PACKET_SCHEMAS};
use secloud_mcp_adapters::{
    has_descriptor_integrity_field, has_risk_factor, is_known_type_state, is_mcp_adapter_schema,
    is_non_executing_state, MCP_ADAPTER_PACKET_SCHEMAS,
};
use secloud_mobile_qa::{
    has_game_qa_check, has_mobile_qa_check, is_mobile_qa_schema, MOBILE_QA_PACKET_SCHEMAS,
};
use secloud_notifications::{
    has_notification_check, is_notification_schema, NOTIFICATION_PACKET_SCHEMAS,
};
use secloud_packets::{FORBIDDEN_ROOT_FILES, REQUIRED_PACKET_SCHEMAS, REQUIRED_ROOT_FILES};
use secloud_permission::{has_permission_rule, is_permission_schema, PERMISSION_PACKET_SCHEMAS};
use secloud_proof_viewer::{is_panel_kind, is_viewer_schema, PROOF_VIEWER_PACKET_SCHEMAS};
use secloud_relay::validate_relay_markdown;
use secloud_release::{
    is_release_schema, is_required_artifact, RELEASE_PACKET_SCHEMAS, REQUIRED_RELEASE_ARTIFACTS,
};
use secloud_remediator::{
    has_command_discovery_rule, has_environment_rule, has_failure_taxonomy_class,
    has_localization_signal, has_patch_tournament_rule, has_proof_requirement,
    has_remediation_boundary, has_remediator_module, has_repair_strategy, is_commercial_artifact,
    is_remediator_schema, REMEDIATOR_PACKET_SCHEMAS,
};
use secloud_repo_worker::{
    has_repo_worker_check, is_repo_worker_schema, REPO_WORKER_PACKET_SCHEMAS,
};
use secloud_seal::validate_seal_json_text;
use secloud_search::{is_allowed_corpus, is_search_schema, SEARCH_PACKET_SCHEMAS};
use secloud_workers::{is_real_surface, is_worker_schema, WORKER_PACKET_SCHEMAS};

const BASE_VALIDATION_TARGETS: &[&str] = &[
    "relay",
    "seal",
    "active",
    "decisions",
    "schemas",
    "root",
    "skills",
    "capabilities",
    "workers",
    "control",
    "learning",
    "search",
    "hypothesis",
    "proof-viewer",
    "hardening",
    "release",
    "e2e",
    "gateway",
    "gateway-transport",
    "mcp-adapters",
    "adapter-type-state",
    "adapter-integrity",
    "adapter-catalog",
    "gemini-worker",
    "normalization",
    "prompt-topology",
    "data-tainting",
    "injection-isolation",
    "backpressure",
    "external-auth",
    "workflow-security",
    "knowledge-mirror",
    "semantic-snapshot",
    "notifications",
    "git-worker",
    "mobile-qa",
    "game-qa",
    "document-ingest",
    "web-ingest",
    "production-adapters",
    "database-boundary",
    "telemetry-adapters",
    "telemetry-redaction",
    "remediator",
    "remediation-intake",
    "remediation-permissions",
    "remediation-reality-map",
    "remediation-command-discovery",
    "remediation-environment",
    "remediation-reproduction",
    "remediation-failure-taxonomy",
    "remediation-localization",
    "remediation-repair-strategy",
    "remediation-patch-tournament",
    "remediation-proof-plan",
    "remediation-report",
    "remediation-commercial",
];

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.as_slice() {
        [cmd] if cmd == "doctor" => doctor(),
        [cmd] if cmd == "status" => status(),
        [cmd, target] if cmd == "validate" => validate_target(target),
        _ => {
            print_help();
            Err("unknown command".to_string())
        }
    };

    match result {
        Ok(message) => println!("PASS: {message}"),
        Err(error) => {
            eprintln!("FAIL: {error}");
            std::process::exit(1);
        }
    }
}

fn validate_target(target: &str) -> Result<String, String> {
    match target {
        "relay" => validate_relay(),
        "seal" => validate_seal(),
        "active" => validate_file_contains("STEALTHEYE_ACTIVE.md", &["Current mission", "Next exact action"]),
        "decisions" => validate_file_contains("STEALTHEYE_DECISIONS.md", &["Frozen decisions", "Forbidden files"]),
        "schemas" => validate_schemas(),
        "root" => validate_root_files(),
        "skills" => validate_skills(),
        "capabilities" => validate_capabilities(),
        "workers" => validate_workers(),
        "control" => validate_control_registry(),
        "learning" => validate_learning(),
        "search" => validate_search(),
        "hypothesis" => validate_hypothesis(),
        "proof-viewer" => validate_proof_viewer(),
        "hardening" => validate_hardening(),
        "release" => validate_release(),
        "e2e" => validate_e2e(),
        "gateway" => validate_gateway(),
        "gateway-transport" => validate_gateway_transport(),
        "mcp-adapters" => validate_mcp_adapters(),
        "adapter-type-state" => validate_adapter_type_state(),
        "adapter-integrity" => validate_adapter_integrity(),
        "adapter-catalog" => validate_adapter_catalog(),
        "gemini-worker" => validate_gemini_worker(),
        "normalization" => validate_normalization(),
        "prompt-topology" => validate_prompt_topology(),
        "data-tainting" => validate_data_tainting(),
        "injection-isolation" => validate_injection_isolation(),
        "backpressure" => validate_backpressure(),
        "external-auth" => validate_external_auth(),
        "workflow-security" => validate_workflow_security(),
        "knowledge-mirror" => validate_knowledge_mirror(),
        "semantic-snapshot" => validate_semantic_snapshot(),
        "notifications" => validate_notifications(),
        "git-worker" => validate_git_worker(),
        "mobile-qa" => validate_mobile_qa(),
        "game-qa" => validate_game_qa(),
        "document-ingest" => validate_document_ingest(),
        "web-ingest" => validate_web_ingest(),
        "production-adapters" => validate_production_adapters(),
        "database-boundary" => validate_database_boundary(),
        "telemetry-adapters" => validate_telemetry_adapters(),
        "telemetry-redaction" => validate_telemetry_redaction(),
        "remediator" => validate_remediator(),
        "remediation-intake" => validate_remediation_intake(),
        "remediation-permissions" => validate_remediation_permissions(),
        "remediation-reality-map" => validate_remediation_reality_map(),
        "remediation-command-discovery" => validate_remediation_command_discovery(),
        "remediation-environment" => validate_remediation_environment(),
        "remediation-reproduction" => validate_remediation_reproduction(),
        "remediation-failure-taxonomy" => validate_remediation_failure_taxonomy(),
        "remediation-localization" => validate_remediation_localization(),
        "remediation-repair-strategy" => validate_remediation_repair_strategy(),
        "remediation-patch-tournament" => validate_remediation_patch_tournament(),
        "remediation-proof-plan" => validate_remediation_proof_plan(),
        "remediation-report" => validate_remediation_report(),
        "remediation-commercial" => validate_remediation_commercial(),
        target if is_build_accelerator_validation_target(target) => validate_build_accelerator_target(target),
        _ => {
            print_help();
            Err(format!("unknown validation target: {target}"))
        }
    }
}

fn print_help() {
    println!("secloud commands:");
    println!("  secloud doctor");
    println!("  secloud status");
    for target in BASE_VALIDATION_TARGETS {
        println!("  secloud validate {target}");
    }
    for target in BUILD_ACCELERATOR_VALIDATION_TARGETS {
        println!("  secloud validate {target}");
    }
}

fn doctor() -> Result<String, String> {
    for target in BASE_VALIDATION_TARGETS {
        validate_target(target)?;
    }
    for target in BUILD_ACCELERATOR_VALIDATION_TARGETS {
        validate_target(target)?;
    }
    Ok("doctor checks passed".to_string())
}

fn status() -> Result<String, String> {
    let active = fs::read_to_string("STEALTHEYE_ACTIVE.md")
        .map_err(|err| format!("failed to read STEALTHEYE_ACTIVE.md: {err}"))?;
    let next = fs::read_to_string("NEXT_ACTION.md")
        .map_err(|err| format!("failed to read NEXT_ACTION.md: {err}"))?;
    println!("# Active\n{active}\n# Next\n{next}");
    Ok("status emitted".to_string())
}

fn validate_schema_files(schema_names: &[&str]) -> Result<(), String> {
    for schema in schema_names {
        let path = Path::new("schemas").join(format!("{schema}.schema.json"));
        if !path.exists() {
            return Err(format!("missing schema: {}", path.display()));
        }
    }
    Ok(())
}

fn validate_file_contains(path: &str, needles: &[&str]) -> Result<String, String> {
    let content = fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    let missing: Vec<&str> = needles.iter().copied().filter(|needle| !content.contains(needle)).collect();
    if missing.is_empty() { Ok(format!("{path} contains required markers")) } else { Err(format!("{path} missing markers: {}", missing.join(", "))) }
}

fn require(label: &str, ok: bool) -> Result<(), String> {
    if ok { Ok(()) } else { Err(format!("missing contract: {label}")) }
}

fn require_all(label: &str, values: &[&str], predicate: impl Fn(&str) -> bool) -> Result<(), String> {
    for value in values { require(&format!("{label}:{value}"), predicate(value))?; }
    Ok(())
}

fn validate_relay() -> Result<String, String> {
    let content = fs::read_to_string("STEALTHEYE_RELAY.md").map_err(|err| format!("failed to read STEALTHEYE_RELAY.md: {err}"))?;
    let result = validate_relay_markdown(&content);
    if result.valid { Ok("relay is valid".to_string()) } else { Err(result.errors.join("; ")) }
}

fn validate_seal() -> Result<String, String> {
    let content = fs::read_to_string("STEALTHEYE_SEAL.json").map_err(|err| format!("failed to read STEALTHEYE_SEAL.json: {err}"))?;
    let result = validate_seal_json_text(&content);
    if result.valid { Ok("seal is valid".to_string()) } else { Err(result.errors.join("; ")) }
}

fn validate_schemas() -> Result<String, String> {
    if !Path::new("schemas").exists() { return Err("schemas directory missing".to_string()); }
    validate_schema_files(REQUIRED_PACKET_SCHEMAS)?;
    validate_schema_files(BUILD_ACCELERATOR_PACKET_SCHEMAS)?;
    Ok("required schemas are present".to_string())
}

fn validate_root_files() -> Result<String, String> {
    for file in REQUIRED_ROOT_FILES { if !Path::new(file).exists() { return Err(format!("missing root file: {file}")); } }
    for file in FORBIDDEN_ROOT_FILES { if Path::new(file).exists() { return Err(format!("forbidden file present: {file}")); } }
    Ok("root files are valid".to_string())
}

fn validate_skills() -> Result<String, String> {
    let required_skills = ["relay-generation", "seal-generation", "tab-resume", "mission-executor", "ci-repair", "browser-repair", "public-kernel-drop", "oss-audit", "codex-worker", "spec-generation"];
    for skill in required_skills { let path = format!(".stealtheye/skills/{skill}/SKILL.md"); if !Path::new(&path).exists() { return Err(format!("missing skill: {path}")); } }
    Ok("required skills are present".to_string())
}

fn validate_capabilities() -> Result<String, String> { validate_file_contains("STEALTHEYE_CAPABILITIES.md", &["Allowed high-level tool names", "Blocked raw tool names", "secloud.status", "secret.read"]) }

fn validate_workers() -> Result<String, String> {
    validate_file_contains("STEALTHEYE_WORKERS.md", &["Real surfaces", "Worker packets", "CodexTaskPacketV0", "FeatureAvailabilityCheckV0"])?;
    require("github.actions", is_real_surface("github.actions"))?;
    require("FeatureAvailabilityCheckV0", is_worker_schema("FeatureAvailabilityCheckV0"))?;
    validate_schema_files(WORKER_PACKET_SCHEMAS)?;
    Ok("workers are valid".to_string())
}

fn validate_control_registry() -> Result<String, String> {
    for name in ALLOWED_TOOL_NAMES { let verdict = classify_tool_name(name); if !verdict.allowed || verdict.blocked { return Err(format!("allowed tool rejected: {name}")); } }
    for name in BLOCKED_TOOL_NAMES { let verdict = classify_tool_name(name); if verdict.allowed || !verdict.blocked { return Err(format!("blocked tool was not blocked: {name}")); } }
    Ok("control registry is valid".to_string())
}

fn validate_learning() -> Result<String, String> { validate_schema_files(LEARNING_PACKET_SCHEMAS)?; require("SkillCandidateV0", is_learning_schema("SkillCandidateV0"))?; require("human_authority_preserved", has_promotion_rule("human_authority_preserved"))?; Ok("learning contracts are valid".to_string()) }
fn validate_search() -> Result<String, String> { validate_schema_files(SEARCH_PACKET_SCHEMAS)?; require("PastSessionSearchV0", is_search_schema("PastSessionSearchV0"))?; require("relay corpus", is_allowed_corpus("relay"))?; require("seal corpus", is_allowed_corpus("seal"))?; Ok("search contracts are valid".to_string()) }
fn validate_hypothesis() -> Result<String, String> { validate_schema_files(HYPOTHESIS_PACKET_SCHEMAS)?; require("HypothesisRaceV0", is_hypothesis_schema("HypothesisRaceV0"))?; require("authority_boundary", is_stop_condition("authority_boundary"))?; Ok("hypothesis contracts are valid".to_string()) }
fn validate_proof_viewer() -> Result<String, String> { validate_schema_files(PROOF_VIEWER_PACKET_SCHEMAS)?; require("ProofCanvasManifestV0", is_viewer_schema("ProofCanvasManifestV0"))?; require("browser panel", is_panel_kind("browser"))?; Ok("proof viewer contracts are valid".to_string()) }
fn validate_hardening() -> Result<String, String> { validate_schema_files(HARDENING_PACKET_SCHEMAS)?; require("ReleaseReadinessV0", is_hardening_schema("ReleaseReadinessV0"))?; for check in REQUIRED_HARDENING_CHECKS { require(check, has_required_check(check))?; } Ok("hardening contracts are valid".to_string()) }
fn validate_release() -> Result<String, String> { validate_schema_files(RELEASE_PACKET_SCHEMAS)?; require("ReleaseCandidateV0", is_release_schema("ReleaseCandidateV0"))?; for artifact in REQUIRED_RELEASE_ARTIFACTS { require(artifact, is_required_artifact(artifact))?; if !Path::new(artifact).exists() { return Err(format!("release artifact missing from repo: {artifact}")); } } Ok("release contracts are valid".to_string()) }
fn validate_e2e() -> Result<String, String> { validate_schema_files(E2E_PACKET_SCHEMAS)?; require("EndToEndMissionV0", is_e2e_schema("EndToEndMissionV0"))?; for step in REQUIRED_E2E_STEPS { require(step, has_required_step(step))?; } Ok("e2e contracts are valid".to_string()) }

fn validate_gateway() -> Result<String, String> { validate_schema_files(GATEWAY_PACKET_SCHEMAS)?; require("GatewayPolicyV0", is_gateway_schema("GatewayPolicyV0"))?; require_all("gateway-boundary", &["transport_declared", "session_scoped", "origin_checked", "authority_declared", "no_live_external_activation"], has_gateway_boundary)?; Ok("gateway contracts are valid".to_string()) }
fn validate_gateway_transport() -> Result<String, String> { validate_schema_files(GATEWAY_PACKET_SCHEMAS)?; require("GatewayTransportPolicyV0", is_gateway_schema("GatewayTransportPolicyV0"))?; require_all("gateway-transport", &["declared_protocol", "bounded_payload", "explicit_origin", "deterministic_error_surface"], has_transport_requirement)?; Ok("gateway transport contracts are valid".to_string()) }
fn validate_backpressure() -> Result<String, String> { validate_schema_files(GATEWAY_PACKET_SCHEMAS)?; require("BackpressurePolicyV0", is_gateway_schema("BackpressurePolicyV0"))?; require_all("backpressure-control", &["retry_budget", "rate_budget", "token_budget", "loop_cutoff"], has_backpressure_control)?; Ok("backpressure contracts are valid".to_string()) }
fn validate_mcp_adapters() -> Result<String, String> { validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?; require("McpAdapterRegistryV0", is_mcp_adapter_schema("McpAdapterRegistryV0"))?; Ok("MCP adapter contracts are valid".to_string()) }
fn validate_adapter_type_state() -> Result<String, String> { validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?; require("AdapterTypeStateV0", is_mcp_adapter_schema("AdapterTypeStateV0"))?; require_all("adapter-state", &["contract_only", "quarantined", "rejected"], is_known_type_state)?; require_all("adapter-non-executing-state", &["contract_only", "quarantined", "rejected"], is_non_executing_state)?; Ok("adapter type-state contracts are valid".to_string()) }
fn validate_adapter_integrity() -> Result<String, String> { validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?; require("AdapterDescriptorIntegrityV0", is_mcp_adapter_schema("AdapterDescriptorIntegrityV0"))?; require_all("adapter-integrity-field", &["capability_digest", "schema_digest", "declared_permissions", "source_digest"], has_descriptor_integrity_field)?; Ok("adapter integrity contracts are valid".to_string()) }
fn validate_adapter_catalog() -> Result<String, String> { validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?; require("AdapterCandidateCatalogV0", is_mcp_adapter_schema("AdapterCandidateCatalogV0"))?; require_all("adapter-risk-factor", &["requires_secret", "untrusted_descriptor", "mutable_remote_capability"], has_risk_factor)?; Ok("adapter catalog contracts are valid".to_string()) }

fn validate_gemini_worker() -> Result<String, String> { validate_schema_files(GEMINI_WORKER_PACKET_SCHEMAS)?; require("GeminiWorkerReadinessV0", is_gemini_worker_schema("GeminiWorkerReadinessV0"))?; require("no_live_provider_call", has_gemini_readiness_check("no_live_provider_call"))?; Ok("Gemini worker readiness contracts are valid".to_string()) }
fn validate_normalization() -> Result<String, String> { validate_schema_files(GEMINI_WORKER_PACKET_SCHEMAS)?; require("SemanticNormalizationV0", is_gemini_worker_schema("SemanticNormalizationV0"))?; require("semantic_normalization_declared", has_gemini_readiness_check("semantic_normalization_declared"))?; Ok("normalization contracts are valid".to_string()) }
fn validate_prompt_topology() -> Result<String, String> { validate_schema_files(GEMINI_WORKER_PACKET_SCHEMAS)?; require("ModelTopologyBoundaryV0", is_gemini_worker_schema("ModelTopologyBoundaryV0"))?; require_all("prompt-topology-boundary", &["provider_specific_prompt_not_shared", "untrusted_content_cannot_be_instruction"], has_prompt_topology_boundary)?; Ok("prompt topology contracts are valid".to_string()) }

fn validate_data_tainting() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("DataTaintPolicyV0", is_guard_schema("DataTaintPolicyV0"))?; require_all("taint-class", &["untrusted_web_content", "untrusted_document_content", "tool_result_data"], has_taint_class)?; Ok("data tainting contracts are valid".to_string()) }
fn validate_injection_isolation() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("InjectionIsolationPolicyV0", is_guard_schema("InjectionIsolationPolicyV0"))?; require_all("injection-isolation", &["untrusted_content_is_data", "tool_result_cannot_be_instruction", "ingest_payload_cannot_escalate_authority"], has_isolation_rule)?; Ok("injection isolation contracts are valid".to_string()) }
fn validate_workflow_security() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("WorkflowGuardPolicyV0", is_guard_schema("WorkflowGuardPolicyV0"))?; require("workflow_payload_cannot_mutate_plan", has_isolation_rule("workflow_payload_cannot_mutate_plan"))?; Ok("workflow guard contracts are valid".to_string()) }
fn validate_document_ingest() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("DocumentIngestPolicyV0", is_guard_schema("DocumentIngestPolicyV0"))?; require("untrusted_document_content", has_taint_class("untrusted_document_content"))?; require("ingest_payload_cannot_escalate_authority", has_isolation_rule("ingest_payload_cannot_escalate_authority"))?; Ok("document ingest contracts are valid".to_string()) }
fn validate_web_ingest() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("WebIngestPolicyV0", is_guard_schema("WebIngestPolicyV0"))?; require("untrusted_web_content", has_taint_class("untrusted_web_content"))?; require("untrusted_content_is_data", has_isolation_rule("untrusted_content_is_data"))?; Ok("web ingest contracts are valid".to_string()) }
fn validate_production_adapters() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("ProductionAdapterContractV0", is_guard_schema("ProductionAdapterContractV0"))?; require_all("production-contract", &["readiness_only", "no_deployment_mutation", "no_database_mutation"], has_production_contract)?; Ok("production adapter contracts are valid".to_string()) }
fn validate_database_boundary() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("DatabaseBoundaryV0", is_guard_schema("DatabaseBoundaryV0"))?; require("no_database_mutation", has_production_contract("no_database_mutation"))?; Ok("database boundary contracts are valid".to_string()) }
fn validate_telemetry_adapters() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("TelemetryAdapterContractV0", is_guard_schema("TelemetryAdapterContractV0"))?; require("bounded_output", has_production_contract("bounded_output"))?; Ok("telemetry adapter contracts are valid".to_string()) }
fn validate_telemetry_redaction() -> Result<String, String> { validate_schema_files(GUARD_PACKET_SCHEMAS)?; require("TelemetryRedactionPolicyV0", is_guard_schema("TelemetryRedactionPolicyV0"))?; require("redaction_required", has_production_contract("redaction_required"))?; Ok("telemetry redaction contracts are valid".to_string()) }

fn validate_external_auth() -> Result<String, String> { validate_schema_files(PERMISSION_PACKET_SCHEMAS)?; require("ExternalAuthPolicyV0", is_permission_schema("ExternalAuthPolicyV0"))?; require_all("permission-rule", &["no_browser_cookie_extraction", "no_consumer_session_rehydration", "no_two_factor_bypass", "no_billing_bypass_framing"], has_permission_rule)?; Ok("external auth boundary contracts are valid".to_string()) }
fn validate_knowledge_mirror() -> Result<String, String> { validate_schema_files(KNOWLEDGE_MIRROR_PACKET_SCHEMAS)?; require("KnowledgeMirrorReadinessV0", is_knowledge_mirror_schema("KnowledgeMirrorReadinessV0"))?; require_all("knowledge-mirror-check", &["source_inventory_declared", "redaction_policy_declared", "private_data_excluded", "export_is_not_live_sync"], has_mirror_check)?; require("redacts_private_user_data", redacts_class("private_user_data"))?; Ok("knowledge mirror contracts are valid".to_string()) }
fn validate_semantic_snapshot() -> Result<String, String> { validate_schema_files(KNOWLEDGE_MIRROR_PACKET_SCHEMAS)?; require("SemanticSnapshotV0", is_knowledge_mirror_schema("SemanticSnapshotV0"))?; require("semantic_snapshot_declared", has_mirror_check("semantic_snapshot_declared"))?; Ok("semantic snapshot contracts are valid".to_string()) }
fn validate_notifications() -> Result<String, String> { validate_schema_files(NOTIFICATION_PACKET_SCHEMAS)?; require("NotificationReadinessV0", is_notification_schema("NotificationReadinessV0"))?; require_all("notification-check", &["dry_run_available", "no_secret_required_for_validation"], has_notification_check)?; Ok("notification readiness contracts are valid".to_string()) }
fn validate_git_worker() -> Result<String, String> { validate_schema_files(REPO_WORKER_PACKET_SCHEMAS)?; require("GitWorkerReadinessV0", is_repo_worker_schema("GitWorkerReadinessV0"))?; require_all("repo-worker-check", &["read_only_default", "mutation_requires_later_activation"], has_repo_worker_check)?; Ok("git worker readiness contracts are valid".to_string()) }
fn validate_mobile_qa() -> Result<String, String> { validate_schema_files(MOBILE_QA_PACKET_SCHEMAS)?; require("MobileQaReadinessV0", is_mobile_qa_schema("MobileQaReadinessV0"))?; require_all("mobile-qa-check", &["viewport_matrix_declared", "activation_deferred_to_s7"], has_mobile_qa_check)?; Ok("mobile QA readiness contracts are valid".to_string()) }
fn validate_game_qa() -> Result<String, String> { validate_schema_files(MOBILE_QA_PACKET_SCHEMAS)?; require("GameQaReadinessV0", is_mobile_qa_schema("GameQaReadinessV0"))?; require_all("game-qa-check", &["startup_smoke_declared", "screenshot_artifact_declared"], has_game_qa_check)?; Ok("game QA readiness contracts are valid".to_string()) }

fn validate_remediator() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediatorReadinessV0", is_remediator_schema("RemediatorReadinessV0"))?; require_all("remediator-module", &["intake", "permissions", "reality_map", "command_discovery", "environment", "reproduction", "failure_taxonomy", "localization", "repair_strategy", "patch_tournament", "proof_plan", "report", "quote_risk"], has_remediator_module)?; require_all("remediator-boundary", &["no_claim_without_reproduction", "no_patch_without_reproduced_failure", "bounded_patch_only", "proof_gates_required", "diagnosis_only_when_unreproduced", "no_browser_cookie_session_automation", "no_secrets", "no_paid_compute", "no_production_deployment", "no_database_mutation", "commercial_quote_does_not_activate_billing"], has_remediation_boundary)?; Ok("active Remediator Mode contracts are valid".to_string()) }
fn validate_remediation_intake() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationIntakeV0", is_remediator_schema("RemediationIntakeV0"))?; require("intake", has_remediator_module("intake"))?; Ok("remediation intake contracts are valid".to_string()) }
fn validate_remediation_permissions() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationPermissionsV0", is_remediator_schema("RemediationPermissionsV0"))?; require("permissions", has_remediator_module("permissions"))?; require("no_secrets", has_remediation_boundary("no_secrets"))?; Ok("remediation permission contracts are valid".to_string()) }
fn validate_remediation_reality_map() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationRealityMapV0", is_remediator_schema("RemediationRealityMapV0"))?; require("reality_map", has_remediator_module("reality_map"))?; Ok("remediation reality map contracts are valid".to_string()) }
fn validate_remediation_command_discovery() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationCommandDiscoveryV0", is_remediator_schema("RemediationCommandDiscoveryV0"))?; require("command_discovery", has_remediator_module("command_discovery"))?; require_all("command-discovery-rule", &["inspect_declared_scripts", "prefer_existing_verify_entrypoint", "bounded_command_set", "no_secret_commands"], has_command_discovery_rule)?; Ok("remediation command discovery contracts are valid".to_string()) }
fn validate_remediation_environment() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationEnvironmentV0", is_remediator_schema("RemediationEnvironmentV0"))?; require("environment", has_remediator_module("environment"))?; require_all("environment-rule", &["detect_language_runtime", "synthesize_minimal_environment", "avoid_paid_compute", "record_platform_assumptions"], has_environment_rule)?; Ok("remediation environment contracts are valid".to_string()) }
fn validate_remediation_reproduction() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationReproductionV0", is_remediator_schema("RemediationReproductionV0"))?; require("reproduction", has_remediator_module("reproduction"))?; require("failing_behavior_reproduced", has_proof_requirement("failing_behavior_reproduced"))?; require("no_claim_without_reproduction", has_remediation_boundary("no_claim_without_reproduction"))?; Ok("remediation reproduction contracts are valid".to_string()) }
fn validate_remediation_failure_taxonomy() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationFailureTaxonomyV0", is_remediator_schema("RemediationFailureTaxonomyV0"))?; require("failure_taxonomy", has_remediator_module("failure_taxonomy"))?; require_all("failure-taxonomy-class", &["test_failure", "compile_failure", "lint_failure", "browser_failure", "runtime_failure", "dependency_failure", "unknown_unreproduced"], has_failure_taxonomy_class)?; Ok("remediation failure taxonomy contracts are valid".to_string()) }
fn validate_remediation_localization() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationLocalizationV0", is_remediator_schema("RemediationLocalizationV0"))?; require("localization", has_remediator_module("localization"))?; require_all("localization-signal", &["failing_command", "stderr_excerpt", "changed_file_hint", "minimal_reproduction_path"], has_localization_signal)?; Ok("remediation localization contracts are valid".to_string()) }
fn validate_remediation_repair_strategy() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationRepairStrategyV0", is_remediator_schema("RemediationRepairStrategyV0"))?; require("repair_strategy", has_remediator_module("repair_strategy"))?; require_all("repair-strategy", &["minimal_patch", "config_correction", "test_aligned_fix", "dependency_pin_or_update", "diagnosis_only"], has_repair_strategy)?; Ok("remediation repair strategy contracts are valid".to_string()) }
fn validate_remediation_patch_tournament() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationPatchTournamentV0", is_remediator_schema("RemediationPatchTournamentV0"))?; require("patch_tournament", has_remediator_module("patch_tournament"))?; require_all("patch-tournament-rule", &["candidate_patch_bounded", "candidate_patch_reversible", "selects_first_green_candidate", "rejects_unproven_candidate"], has_patch_tournament_rule)?; Ok("remediation patch tournament contracts are valid".to_string()) }
fn validate_remediation_proof_plan() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationProofPlanV0", is_remediator_schema("RemediationProofPlanV0"))?; require("proof_plan", has_remediator_module("proof_plan"))?; require_all("proof-requirement", &["failing_behavior_reproduced", "bounded_patch_applied", "proof_gates_passed", "remediation_report_emitted"], has_proof_requirement)?; Ok("remediation proof plan contracts are valid".to_string()) }
fn validate_remediation_report() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationReportV0", is_remediator_schema("RemediationReportV0"))?; require("report", has_remediator_module("report"))?; require("diagnosis_only_when_unreproduced", has_remediation_boundary("diagnosis_only_when_unreproduced"))?; Ok("remediation report contracts are valid".to_string()) }
fn validate_remediation_commercial() -> Result<String, String> { validate_schema_files(REMEDIATOR_PACKET_SCHEMAS)?; require("RemediationCommercialV0", is_remediator_schema("RemediationCommercialV0"))?; require("quote_risk", has_remediator_module("quote_risk"))?; require_all("commercial-artifact", &["quote_range", "risk_band", "scope_assumptions", "billing_not_activated"], is_commercial_artifact)?; require("commercial_quote_does_not_activate_billing", has_remediation_boundary("commercial_quote_does_not_activate_billing"))?; Ok("remediation commercial contracts are valid".to_string()) }

fn validate_build_accelerator_target(target: &str) -> Result<String, String> {
    validate_schema_files(BUILD_ACCELERATOR_PACKET_SCHEMAS)?;
    require(target, is_build_accelerator_validation_target(target))?;
    match target {
        "one-drop" => require_all("one-drop-step", &["mission_approval", "coherent_repo_mutation", "one_pr", "one_proof_wave", "batched_repairs", "merge_when_green"], has_one_drop_step)?,
        "mission-approval" | "approval-compression" | "no-midpoint-ask" => {
            require_all("routine-action", &["safe_file_update", "pr_creation", "ci_inspection", "exact_failure_repair", "merge_when_green"], is_routine_action_class)?;
            require_all("boundary-action", &["secrets", "paid_compute", "database_mutation", "browser_cookie_session_automation", "unresolved_high_impact_ambiguity"], is_boundary_action_class)?;
        }
        "tool-call-bundling" => require_all("tool-fallback-rule", &["prefer_git_tree_batch", "fall_back_to_contents_api", "stop_at_true_boundary"], has_tool_fallback_rule)?,
        "repo-mutation-batch" => require_all("one-drop-step", &["preflight_state_snapshot", "coherent_repo_mutation"], has_one_drop_step)?,
        "batch-repair" => require_all("batch-repair-rule", &["inspect_all_failures_before_patch", "group_related_repairs", "patch_exact_failures_only", "rerun_after_batch"], has_batch_repair_rule)?,
        "merge-aware-handoff" | "no-cleanup-pr" => require_all("merge-handoff-rule", &["pre_merge_truth_template", "post_merge_truth_template", "merge_sha_resolution_field", "no_stale_pending_language", "next_action_survives_merge"], has_merge_handoff_rule)?,
        "state-consistency" | "obsolete-text" | "doc-conflicts" => require_all("state-consistency-rule", &["readme_build_plan_match", "active_relay_seal_match", "next_action_matches_phase", "no_obsolete_phase_reference", "no_duplicate_conflicting_doc_truth"], has_state_consistency_rule)?,
        "phase-truth" | "branch-lifecycle" | "pr-lifecycle" => require_all("lifecycle-state", &["planned", "branch_active", "pr_open", "ci_running", "repairing", "green", "merged", "blocked"], has_lifecycle_state)?,
        "proof-selector" | "required-checks" | "workflow-path-filter" | "ci-wave-counter" => require_all("proof-selection-rule", &["touched_surface_declared", "required_checks_manifested", "path_filters_simulated", "unexpected_skip_detected", "ci_wave_counted"], has_proof_selection_rule)?,
        "repair-cause-memory" => require_all("batch-repair-rule", &["inspect_all_failures_before_patch", "patch_exact_failures_only"], has_batch_repair_rule)?,
        "validator-registration" | "schema-inventory" | "workspace-registration" => require_all("registration-guard", &["validator_registered", "schema_inventory_registered", "workspace_member_registered", "cli_dependency_registered", "workflow_registered"], has_registration_guard)?,
        "merge-readiness-red-team" | "no-silent-downgrade" => require_all("no-silent-downgrade-rule", &["required_validator_not_removed", "required_schema_not_removed", "required_workflow_not_removed", "proof_gate_not_weakened", "safety_boundary_not_relaxed"], has_no_silent_downgrade_rule)?,
        "build-velocity" => require_all("velocity-metric", &["mission_approvals", "repo_mutation_batches", "pull_requests", "ci_waves", "repair_commits", "cleanup_prs_avoided"], has_velocity_metric)?,
        "confirmation-friction" | "human-availability" => { require_all("friction-metric", &["routine_confirmations_avoided", "true_boundaries_detected", "tool_calls_batched", "human_attention_events"], has_friction_metric)?; require_all("human-attention-rule", &["mission_approval_reuse", "no_midpoint_ask_for_routine_action", "friction_event_recorded", "low_attention_workday_supported"], has_human_attention_rule)?; }
        "tool-fallback" => require_all("tool-fallback-rule", &["prefer_git_tree_batch", "fall_back_to_contents_api", "fall_back_to_pr_patch", "stop_at_true_boundary"], has_tool_fallback_rule)?,
        "existing-work-reuse" | "partial-completion-recovery" => require_all("recovery-rule", &["existing_branch_reuse_checked", "existing_pr_reuse_checked", "partial_drop_recovery_plan", "saturation_handoff_prompt"], has_recovery_rule)?,
        "capability-activation-ledger" | "future-phase-contract" => require_all("future-phase-contract-rule", &["one_drop_default", "mission_approval_default", "proof_plan_required", "state_update_required", "final_report_required", "next_tab_prompt_required"], has_future_phase_contract_rule)?,
        "next-tab-prompt" => { for path in ["docs/PROMPTS/NEXT_TAB_PROMPT.md", "docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md"] { require(path, has_required_prompt_artifact(path) && Path::new(path).exists())?; } }
        _ => {}
    }
    for path in ["docs/ONE_DROP_BUILD_MODE.md", "docs/MISSION_APPROVAL_ENVELOPE.md", "docs/BATCH_REPAIR_POLICY.md", "docs/MERGE_AWARE_HANDOFF.md", "docs/PHASE_TEMPLATE_SYSTEM.md", "docs/S9_FINAL_REPORT.md"] {
        require(path, has_required_doc(path) && Path::new(path).exists())?;
    }
    Ok(format!("S9 build accelerator target {target} is valid"))
}
