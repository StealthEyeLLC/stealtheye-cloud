use std::env;
use std::fs;
use std::path::Path;

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
use secloud_repair_readiness::{
    has_repair_boundary, has_repair_module, is_repair_readiness_schema,
    REPAIR_READINESS_PACKET_SCHEMAS,
};
use secloud_repo_worker::{
    has_repo_worker_check, is_repo_worker_schema, REPO_WORKER_PACKET_SCHEMAS,
};
use secloud_seal::validate_seal_json_text;
use secloud_search::{is_allowed_corpus, is_search_schema, SEARCH_PACKET_SCHEMAS};
use secloud_workers::{is_real_surface, is_worker_schema, WORKER_PACKET_SCHEMAS};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.as_slice() {
        [cmd] if cmd == "doctor" => doctor(),
        [cmd] if cmd == "status" => status(),
        [cmd, target] if cmd == "validate" && target == "relay" => validate_relay(),
        [cmd, target] if cmd == "validate" && target == "seal" => validate_seal(),
        [cmd, target] if cmd == "validate" && target == "active" => validate_file_contains(
            "STEALTHEYE_ACTIVE.md",
            &["Current mission", "Next exact action"],
        ),
        [cmd, target] if cmd == "validate" && target == "decisions" => validate_file_contains(
            "STEALTHEYE_DECISIONS.md",
            &["Frozen decisions", "Forbidden files"],
        ),
        [cmd, target] if cmd == "validate" && target == "schemas" => validate_schemas(),
        [cmd, target] if cmd == "validate" && target == "root" => validate_root_files(),
        [cmd, target] if cmd == "validate" && target == "skills" => validate_skills(),
        [cmd, target] if cmd == "validate" && target == "capabilities" => validate_capabilities(),
        [cmd, target] if cmd == "validate" && target == "workers" => validate_workers(),
        [cmd, target] if cmd == "validate" && target == "control" => validate_control_registry(),
        [cmd, target] if cmd == "validate" && target == "learning" => validate_learning(),
        [cmd, target] if cmd == "validate" && target == "search" => validate_search(),
        [cmd, target] if cmd == "validate" && target == "hypothesis" => validate_hypothesis(),
        [cmd, target] if cmd == "validate" && target == "proof-viewer" => validate_proof_viewer(),
        [cmd, target] if cmd == "validate" && target == "hardening" => validate_hardening(),
        [cmd, target] if cmd == "validate" && target == "release" => validate_release(),
        [cmd, target] if cmd == "validate" && target == "e2e" => validate_e2e(),
        [cmd, target] if cmd == "validate" && target == "gateway" => validate_gateway(),
        [cmd, target] if cmd == "validate" && target == "gateway-transport" => {
            validate_gateway_transport()
        }
        [cmd, target] if cmd == "validate" && target == "mcp-adapters" => validate_mcp_adapters(),
        [cmd, target] if cmd == "validate" && target == "adapter-type-state" => {
            validate_adapter_type_state()
        }
        [cmd, target] if cmd == "validate" && target == "adapter-integrity" => {
            validate_adapter_integrity()
        }
        [cmd, target] if cmd == "validate" && target == "adapter-catalog" => {
            validate_adapter_catalog()
        }
        [cmd, target] if cmd == "validate" && target == "gemini-worker" => validate_gemini_worker(),
        [cmd, target] if cmd == "validate" && target == "normalization" => validate_normalization(),
        [cmd, target] if cmd == "validate" && target == "prompt-topology" => {
            validate_prompt_topology()
        }
        [cmd, target] if cmd == "validate" && target == "data-tainting" => validate_data_tainting(),
        [cmd, target] if cmd == "validate" && target == "injection-isolation" => {
            validate_injection_isolation()
        }
        [cmd, target] if cmd == "validate" && target == "backpressure" => validate_backpressure(),
        [cmd, target] if cmd == "validate" && target == "external-auth" => validate_external_auth(),
        [cmd, target] if cmd == "validate" && target == "workflow-security" => {
            validate_workflow_security()
        }
        [cmd, target] if cmd == "validate" && target == "knowledge-mirror" => {
            validate_knowledge_mirror()
        }
        [cmd, target] if cmd == "validate" && target == "semantic-snapshot" => {
            validate_semantic_snapshot()
        }
        [cmd, target] if cmd == "validate" && target == "notifications" => validate_notifications(),
        [cmd, target] if cmd == "validate" && target == "git-worker" => validate_git_worker(),
        [cmd, target] if cmd == "validate" && target == "mobile-qa" => validate_mobile_qa(),
        [cmd, target] if cmd == "validate" && target == "game-qa" => validate_game_qa(),
        [cmd, target] if cmd == "validate" && target == "document-ingest" => validate_document_ingest(),
        [cmd, target] if cmd == "validate" && target == "web-ingest" => validate_web_ingest(),
        [cmd, target] if cmd == "validate" && target == "production-adapters" => {
            validate_production_adapters()
        }
        [cmd, target] if cmd == "validate" && target == "database-boundary" => {
            validate_database_boundary()
        }
        [cmd, target] if cmd == "validate" && target == "telemetry-adapters" => {
            validate_telemetry_adapters()
        }
        [cmd, target] if cmd == "validate" && target == "telemetry-redaction" => {
            validate_telemetry_redaction()
        }
        [cmd, target] if cmd == "validate" && target == "remediator" => validate_remediator(),
        [cmd, target] if cmd == "validate" && target == "remediation-intake" => {
            validate_remediation_intake()
        }
        [cmd, target] if cmd == "validate" && target == "remediation-permissions" => {
            validate_remediation_permissions()
        }
        [cmd, target] if cmd == "validate" && target == "remediation-reproduction" => {
            validate_remediation_reproduction()
        }
        [cmd, target] if cmd == "validate" && target == "remediation-failure-taxonomy" => {
            validate_remediation_failure_taxonomy()
        }
        [cmd, target] if cmd == "validate" && target == "remediation-proof-plan" => {
            validate_remediation_proof_plan()
        }
        [cmd, target] if cmd == "validate" && target == "remediation-report" => {
            validate_remediation_report()
        }
        _ => {
            print_help();
            Err("unknown command".to_string())
        }
    };

    match result {
        Ok(message) => {
            println!("PASS: {message}");
        }
        Err(error) => {
            eprintln!("FAIL: {error}");
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("secloud commands:");
    println!("  secloud doctor");
    println!("  secloud status");
    println!("  secloud validate relay");
    println!("  secloud validate seal");
    println!("  secloud validate active");
    println!("  secloud validate decisions");
    println!("  secloud validate schemas");
    println!("  secloud validate root");
    println!("  secloud validate skills");
    println!("  secloud validate capabilities");
    println!("  secloud validate workers");
    println!("  secloud validate control");
    println!("  secloud validate learning");
    println!("  secloud validate search");
    println!("  secloud validate hypothesis");
    println!("  secloud validate proof-viewer");
    println!("  secloud validate hardening");
    println!("  secloud validate release");
    println!("  secloud validate e2e");
    println!("  secloud validate gateway");
    println!("  secloud validate gateway-transport");
    println!("  secloud validate mcp-adapters");
    println!("  secloud validate adapter-type-state");
    println!("  secloud validate adapter-integrity");
    println!("  secloud validate adapter-catalog");
    println!("  secloud validate gemini-worker");
    println!("  secloud validate normalization");
    println!("  secloud validate prompt-topology");
    println!("  secloud validate data-tainting");
    println!("  secloud validate injection-isolation");
    println!("  secloud validate backpressure");
    println!("  secloud validate external-auth");
    println!("  secloud validate workflow-security");
    println!("  secloud validate knowledge-mirror");
    println!("  secloud validate semantic-snapshot");
    println!("  secloud validate notifications");
    println!("  secloud validate git-worker");
    println!("  secloud validate mobile-qa");
    println!("  secloud validate game-qa");
    println!("  secloud validate document-ingest");
    println!("  secloud validate web-ingest");
    println!("  secloud validate production-adapters");
    println!("  secloud validate database-boundary");
    println!("  secloud validate telemetry-adapters");
    println!("  secloud validate telemetry-redaction");
    println!("  secloud validate remediator");
    println!("  secloud validate remediation-intake");
    println!("  secloud validate remediation-permissions");
    println!("  secloud validate remediation-reproduction");
    println!("  secloud validate remediation-failure-taxonomy");
    println!("  secloud validate remediation-proof-plan");
    println!("  secloud validate remediation-report");
}

fn doctor() -> Result<String, String> {
    validate_root_files()?;
    validate_schemas()?;
    validate_relay()?;
    validate_seal()?;
    validate_skills()?;
    validate_capabilities()?;
    validate_workers()?;
    validate_control_registry()?;
    validate_learning()?;
    validate_search()?;
    validate_hypothesis()?;
    validate_proof_viewer()?;
    validate_hardening()?;
    validate_release()?;
    validate_e2e()?;
    validate_gateway()?;
    validate_gateway_transport()?;
    validate_mcp_adapters()?;
    validate_adapter_type_state()?;
    validate_adapter_integrity()?;
    validate_adapter_catalog()?;
    validate_gemini_worker()?;
    validate_normalization()?;
    validate_prompt_topology()?;
    validate_data_tainting()?;
    validate_injection_isolation()?;
    validate_backpressure()?;
    validate_external_auth()?;
    validate_workflow_security()?;
    validate_knowledge_mirror()?;
    validate_semantic_snapshot()?;
    validate_notifications()?;
    validate_git_worker()?;
    validate_mobile_qa()?;
    validate_game_qa()?;
    validate_document_ingest()?;
    validate_web_ingest()?;
    validate_production_adapters()?;
    validate_database_boundary()?;
    validate_telemetry_adapters()?;
    validate_telemetry_redaction()?;
    validate_remediator()?;
    validate_remediation_intake()?;
    validate_remediation_permissions()?;
    validate_remediation_reproduction()?;
    validate_remediation_failure_taxonomy()?;
    validate_remediation_proof_plan()?;
    validate_remediation_report()?;
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

fn validate_relay() -> Result<String, String> {
    let content = fs::read_to_string("STEALTHEYE_RELAY.md")
        .map_err(|err| format!("failed to read STEALTHEYE_RELAY.md: {err}"))?;
    let result = validate_relay_markdown(&content);
    if result.valid {
        Ok("relay is valid".to_string())
    } else {
        Err(result.errors.join("; "))
    }
}

fn validate_seal() -> Result<String, String> {
    let content = fs::read_to_string("STEALTHEYE_SEAL.json")
        .map_err(|err| format!("failed to read STEALTHEYE_SEAL.json: {err}"))?;
    let result = validate_seal_json_text(&content);
    if result.valid {
        Ok("seal is valid".to_string())
    } else {
        Err(result.errors.join("; "))
    }
}

fn validate_file_contains(path: &str, needles: &[&str]) -> Result<String, String> {
    let content =
        fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
    let mut missing = Vec::new();
    for needle in needles {
        if !content.contains(needle) {
            missing.push(*needle);
        }
    }
    if missing.is_empty() {
        Ok(format!("{path} contains required markers"))
    } else {
        Err(format!("{path} missing markers: {}", missing.join(", ")))
    }
}

fn validate_schemas() -> Result<String, String> {
    let root = Path::new("schemas");
    if !root.exists() {
        return Err("schemas directory missing".to_string());
    }

    let mut missing = Vec::new();
    for schema in REQUIRED_PACKET_SCHEMAS {
        let path = root.join(format!("{schema}.schema.json"));
        if !path.exists() {
            missing.push(path.display().to_string());
        }
    }

    if missing.is_empty() {
        Ok("required schemas are present".to_string())
    } else {
        Err(format!("missing schemas: {}", missing.join(", ")))
    }
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

fn require(label: &str, ok: bool) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(format!("missing contract: {label}"))
    }
}

fn require_all(label: &str, values: &[&str], predicate: impl Fn(&str) -> bool) -> Result<(), String> {
    for value in values {
        require(&format!("{label}:{value}"), predicate(value))?;
    }
    Ok(())
}

fn validate_root_files() -> Result<String, String> {
    let mut missing = Vec::new();
    for file in REQUIRED_ROOT_FILES {
        if !Path::new(file).exists() {
            missing.push(*file);
        }
    }

    let mut forbidden_present = Vec::new();
    for file in FORBIDDEN_ROOT_FILES {
        if Path::new(file).exists() {
            forbidden_present.push(*file);
        }
    }

    if !missing.is_empty() {
        return Err(format!("missing root files: {}", missing.join(", ")));
    }
    if !forbidden_present.is_empty() {
        return Err(format!(
            "forbidden files present: {}",
            forbidden_present.join(", ")
        ));
    }

    Ok("root files are valid".to_string())
}

fn validate_skills() -> Result<String, String> {
    let required_skills = [
        "relay-generation",
        "seal-generation",
        "tab-resume",
        "mission-executor",
        "ci-repair",
        "browser-repair",
        "public-kernel-drop",
        "oss-audit",
        "codex-worker",
        "spec-generation",
    ];
    let mut missing = Vec::new();
    for skill in required_skills {
        let path = format!(".stealtheye/skills/{skill}/SKILL.md");
        if !Path::new(&path).exists() {
            missing.push(path);
        }
    }
    if missing.is_empty() {
        Ok("required skills are present".to_string())
    } else {
        Err(format!("missing skills: {}", missing.join(", ")))
    }
}

fn validate_capabilities() -> Result<String, String> {
    validate_file_contains(
        "STEALTHEYE_CAPABILITIES.md",
        &[
            "Allowed high-level tool names",
            "Blocked raw tool names",
            "secloud.status",
            "secret.read",
        ],
    )
}

fn validate_workers() -> Result<String, String> {
    validate_file_contains(
        "STEALTHEYE_WORKERS.md",
        &[
            "Real surfaces",
            "Worker packets",
            "CodexTaskPacketV0",
            "FeatureAvailabilityCheckV0",
        ],
    )?;
    if !is_real_surface("github.actions") {
        return Err("worker registry must include github.actions".to_string());
    }
    if !is_worker_schema("FeatureAvailabilityCheckV0") {
        return Err("worker schema inventory must include FeatureAvailabilityCheckV0".to_string());
    }
    validate_schema_files(WORKER_PACKET_SCHEMAS)?;
    Ok("workers are valid".to_string())
}

fn validate_control_registry() -> Result<String, String> {
    for name in ALLOWED_TOOL_NAMES {
        let verdict = classify_tool_name(name);
        if !verdict.allowed || verdict.blocked {
            return Err(format!("allowed tool rejected: {name}"));
        }
    }
    for name in BLOCKED_TOOL_NAMES {
        let verdict = classify_tool_name(name);
        if verdict.allowed || !verdict.blocked {
            return Err(format!("blocked tool was not blocked: {name}"));
        }
    }
    Ok("control registry is valid".to_string())
}

fn validate_learning() -> Result<String, String> {
    validate_schema_files(LEARNING_PACKET_SCHEMAS)?;
    if !is_learning_schema("SkillCandidateV0") {
        return Err("learning registry must include SkillCandidateV0".to_string());
    }
    if !has_promotion_rule("human_authority_preserved") {
        return Err("learning promotion rules must preserve human authority".to_string());
    }
    Ok("learning contracts are valid".to_string())
}

fn validate_search() -> Result<String, String> {
    validate_schema_files(SEARCH_PACKET_SCHEMAS)?;
    if !is_search_schema("PastSessionSearchV0") {
        return Err("search registry must include PastSessionSearchV0".to_string());
    }
    if !is_allowed_corpus("relay") || !is_allowed_corpus("seal") {
        return Err("search corpus must include relay and seal".to_string());
    }
    Ok("search contracts are valid".to_string())
}

fn validate_hypothesis() -> Result<String, String> {
    validate_schema_files(HYPOTHESIS_PACKET_SCHEMAS)?;
    if !is_hypothesis_schema("HypothesisRaceV0") {
        return Err("hypothesis registry must include HypothesisRaceV0".to_string());
    }
    if !is_stop_condition("authority_boundary") {
        return Err("hypothesis stop conditions must include authority_boundary".to_string());
    }
    Ok("hypothesis contracts are valid".to_string())
}

fn validate_proof_viewer() -> Result<String, String> {
    validate_schema_files(PROOF_VIEWER_PACKET_SCHEMAS)?;
    if !is_viewer_schema("ProofCanvasManifestV0") {
        return Err("proof viewer registry must include ProofCanvasManifestV0".to_string());
    }
    if !is_panel_kind("browser") {
        return Err("proof viewer panels must include browser".to_string());
    }
    Ok("proof viewer contracts are valid".to_string())
}

fn validate_hardening() -> Result<String, String> {
    validate_schema_files(HARDENING_PACKET_SCHEMAS)?;
    if !is_hardening_schema("ReleaseReadinessV0") {
        return Err("hardening registry must include ReleaseReadinessV0".to_string());
    }
    for check in REQUIRED_HARDENING_CHECKS {
        if !has_required_check(check) {
            return Err(format!("missing hardening check: {check}"));
        }
    }
    Ok("hardening contracts are valid".to_string())
}

fn validate_release() -> Result<String, String> {
    validate_schema_files(RELEASE_PACKET_SCHEMAS)?;
    if !is_release_schema("ReleaseCandidateV0") {
        return Err("release registry must include ReleaseCandidateV0".to_string());
    }
    for artifact in REQUIRED_RELEASE_ARTIFACTS {
        if !is_required_artifact(artifact) {
            return Err(format!("release registry missing artifact: {artifact}"));
        }
        if !Path::new(artifact).exists() {
            return Err(format!("release artifact missing from repo: {artifact}"));
        }
    }
    Ok("release contracts are valid".to_string())
}

fn validate_e2e() -> Result<String, String> {
    validate_schema_files(E2E_PACKET_SCHEMAS)?;
    if !is_e2e_schema("EndToEndMissionV0") {
        return Err("e2e registry must include EndToEndMissionV0".to_string());
    }
    for step in REQUIRED_E2E_STEPS {
        if !has_required_step(step) {
            return Err(format!("missing e2e step: {step}"));
        }
    }
    Ok("e2e contracts are valid".to_string())
}

fn validate_gateway() -> Result<String, String> {
    validate_schema_files(GATEWAY_PACKET_SCHEMAS)?;
    require("GatewayPolicyV0", is_gateway_schema("GatewayPolicyV0"))?;
    require_all(
        "gateway-boundary",
        &[
            "transport_declared",
            "session_scoped",
            "origin_checked",
            "authority_declared",
            "no_live_external_activation",
        ],
        has_gateway_boundary,
    )?;
    Ok("gateway contracts are valid".to_string())
}

fn validate_gateway_transport() -> Result<String, String> {
    validate_schema_files(GATEWAY_PACKET_SCHEMAS)?;
    require(
        "GatewayTransportPolicyV0",
        is_gateway_schema("GatewayTransportPolicyV0"),
    )?;
    require_all(
        "gateway-transport",
        &[
            "declared_protocol",
            "bounded_payload",
            "explicit_origin",
            "deterministic_error_surface",
        ],
        has_transport_requirement,
    )?;
    Ok("gateway transport contracts are valid".to_string())
}

fn validate_backpressure() -> Result<String, String> {
    validate_schema_files(GATEWAY_PACKET_SCHEMAS)?;
    require("BackpressurePolicyV0", is_gateway_schema("BackpressurePolicyV0"))?;
    require_all(
        "backpressure-control",
        &["retry_budget", "rate_budget", "token_budget", "loop_cutoff"],
        has_backpressure_control,
    )?;
    Ok("backpressure contracts are valid".to_string())
}

fn validate_mcp_adapters() -> Result<String, String> {
    validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?;
    require(
        "McpAdapterRegistryV0",
        is_mcp_adapter_schema("McpAdapterRegistryV0"),
    )?;
    Ok("MCP adapter contracts are valid".to_string())
}

fn validate_adapter_type_state() -> Result<String, String> {
    validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?;
    require("AdapterTypeStateV0", is_mcp_adapter_schema("AdapterTypeStateV0"))?;
    require_all(
        "adapter-state",
        &["contract_only", "quarantined", "rejected"],
        is_known_type_state,
    )?;
    require_all(
        "adapter-non-executing-state",
        &["contract_only", "quarantined", "rejected"],
        is_non_executing_state,
    )?;
    Ok("adapter type-state contracts are valid".to_string())
}

fn validate_adapter_integrity() -> Result<String, String> {
    validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?;
    require(
        "AdapterDescriptorIntegrityV0",
        is_mcp_adapter_schema("AdapterDescriptorIntegrityV0"),
    )?;
    require_all(
        "adapter-integrity-field",
        &[
            "capability_digest",
            "schema_digest",
            "declared_permissions",
            "source_digest",
        ],
        has_descriptor_integrity_field,
    )?;
    Ok("adapter integrity contracts are valid".to_string())
}

fn validate_adapter_catalog() -> Result<String, String> {
    validate_schema_files(MCP_ADAPTER_PACKET_SCHEMAS)?;
    require(
        "AdapterCandidateCatalogV0",
        is_mcp_adapter_schema("AdapterCandidateCatalogV0"),
    )?;
    require_all(
        "adapter-risk-factor",
        &["requires_secret", "untrusted_descriptor", "mutable_remote_capability"],
        has_risk_factor,
    )?;
    Ok("adapter catalog contracts are valid".to_string())
}

fn validate_gemini_worker() -> Result<String, String> {
    validate_schema_files(GEMINI_WORKER_PACKET_SCHEMAS)?;
    require(
        "GeminiWorkerReadinessV0",
        is_gemini_worker_schema("GeminiWorkerReadinessV0"),
    )?;
    require(
        "no_live_provider_call",
        has_gemini_readiness_check("no_live_provider_call"),
    )?;
    Ok("Gemini worker readiness contracts are valid".to_string())
}

fn validate_normalization() -> Result<String, String> {
    validate_schema_files(GEMINI_WORKER_PACKET_SCHEMAS)?;
    require(
        "SemanticNormalizationV0",
        is_gemini_worker_schema("SemanticNormalizationV0"),
    )?;
    require(
        "semantic_normalization_declared",
        has_gemini_readiness_check("semantic_normalization_declared"),
    )?;
    Ok("normalization contracts are valid".to_string())
}

fn validate_prompt_topology() -> Result<String, String> {
    validate_schema_files(GEMINI_WORKER_PACKET_SCHEMAS)?;
    require(
        "ProviderPromptTopologyV0",
        is_gemini_worker_schema("ProviderPromptTopologyV0"),
    )?;
    require_all(
        "prompt-topology-boundary",
        &[
            "provider_specific_prompt_not_shared",
            "untrusted_content_cannot_be_instruction",
        ],
        has_prompt_topology_boundary,
    )?;
    Ok("prompt topology contracts are valid".to_string())
}

fn validate_data_tainting() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require("DataTaintPolicyV0", is_guard_schema("DataTaintPolicyV0"))?;
    require_all(
        "taint-class",
        &[
            "untrusted_web_content",
            "untrusted_document_content",
            "tool_result_data",
        ],
        has_taint_class,
    )?;
    Ok("data tainting contracts are valid".to_string())
}

fn validate_injection_isolation() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require(
        "InjectionIsolationPolicyV0",
        is_guard_schema("InjectionIsolationPolicyV0"),
    )?;
    require_all(
        "injection-isolation",
        &[
            "untrusted_content_is_data",
            "tool_result_cannot_be_instruction",
            "ingest_payload_cannot_escalate_authority",
        ],
        has_isolation_rule,
    )?;
    Ok("injection isolation contracts are valid".to_string())
}

fn validate_external_auth() -> Result<String, String> {
    validate_schema_files(PERMISSION_PACKET_SCHEMAS)?;
    require("ExternalAuthPolicyV0", is_permission_schema("ExternalAuthPolicyV0"))?;
    require_all(
        "permission-rule",
        &[
            "no_browser_cookie_extraction",
            "no_consumer_session_rehydration",
            "no_two_factor_bypass",
            "no_billing_bypass_framing",
        ],
        has_permission_rule,
    )?;
    Ok("external auth boundary contracts are valid".to_string())
}

fn validate_workflow_security() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require("WorkflowGuardPolicyV0", is_guard_schema("WorkflowGuardPolicyV0"))?;
    require(
        "workflow_payload_cannot_mutate_plan",
        has_isolation_rule("workflow_payload_cannot_mutate_plan"),
    )?;
    Ok("workflow guard contracts are valid".to_string())
}

fn validate_knowledge_mirror() -> Result<String, String> {
    validate_schema_files(KNOWLEDGE_MIRROR_PACKET_SCHEMAS)?;
    require(
        "KnowledgeMirrorReadinessV0",
        is_knowledge_mirror_schema("KnowledgeMirrorReadinessV0"),
    )?;
    require_all(
        "knowledge-mirror-check",
        &[
            "source_inventory_declared",
            "redaction_policy_declared",
            "private_data_excluded",
            "export_is_not_live_sync",
        ],
        has_mirror_check,
    )?;
    require("redacts_private_user_data", redacts_class("private_user_data"))?;
    Ok("knowledge mirror contracts are valid".to_string())
}

fn validate_semantic_snapshot() -> Result<String, String> {
    validate_schema_files(KNOWLEDGE_MIRROR_PACKET_SCHEMAS)?;
    require(
        "SemanticSnapshotV0",
        is_knowledge_mirror_schema("SemanticSnapshotV0"),
    )?;
    require(
        "semantic_snapshot_declared",
        has_mirror_check("semantic_snapshot_declared"),
    )?;
    Ok("semantic snapshot contracts are valid".to_string())
}

fn validate_notifications() -> Result<String, String> {
    validate_schema_files(NOTIFICATION_PACKET_SCHEMAS)?;
    require(
        "NotificationReadinessV0",
        is_notification_schema("NotificationReadinessV0"),
    )?;
    require_all(
        "notification-check",
        &["dry_run_available", "no_secret_required_for_validation"],
        has_notification_check,
    )?;
    Ok("notification readiness contracts are valid".to_string())
}

fn validate_git_worker() -> Result<String, String> {
    validate_schema_files(REPO_WORKER_PACKET_SCHEMAS)?;
    require(
        "GitWorkerReadinessV0",
        is_repo_worker_schema("GitWorkerReadinessV0"),
    )?;
    require_all(
        "repo-worker-check",
        &["read_only_default", "mutation_requires_later_activation"],
        has_repo_worker_check,
    )?;
    Ok("git worker readiness contracts are valid".to_string())
}

fn validate_mobile_qa() -> Result<String, String> {
    validate_schema_files(MOBILE_QA_PACKET_SCHEMAS)?;
    require("MobileQaReadinessV0", is_mobile_qa_schema("MobileQaReadinessV0"))?;
    require_all(
        "mobile-qa-check",
        &["viewport_matrix_declared", "activation_deferred_to_s7"],
        has_mobile_qa_check,
    )?;
    Ok("mobile QA readiness contracts are valid".to_string())
}

fn validate_game_qa() -> Result<String, String> {
    validate_schema_files(MOBILE_QA_PACKET_SCHEMAS)?;
    require("GameQaReadinessV0", is_mobile_qa_schema("GameQaReadinessV0"))?;
    require_all(
        "game-qa-check",
        &["startup_smoke_declared", "screenshot_artifact_declared"],
        has_game_qa_check,
    )?;
    Ok("game QA readiness contracts are valid".to_string())
}

fn validate_document_ingest() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require("DocumentIngestPolicyV0", is_guard_schema("DocumentIngestPolicyV0"))?;
    require(
        "untrusted_document_content",
        has_taint_class("untrusted_document_content"),
    )?;
    require(
        "ingest_payload_cannot_escalate_authority",
        has_isolation_rule("ingest_payload_cannot_escalate_authority"),
    )?;
    Ok("document ingest contracts are valid".to_string())
}

fn validate_web_ingest() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require("WebIngestPolicyV0", is_guard_schema("WebIngestPolicyV0"))?;
    require("untrusted_web_content", has_taint_class("untrusted_web_content"))?;
    require(
        "untrusted_content_is_data",
        has_isolation_rule("untrusted_content_is_data"),
    )?;
    Ok("web ingest contracts are valid".to_string())
}

fn validate_production_adapters() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require(
        "ProductionAdapterContractV0",
        is_guard_schema("ProductionAdapterContractV0"),
    )?;
    require_all(
        "production-contract",
        &["readiness_only", "no_deployment_mutation", "no_database_mutation"],
        has_production_contract,
    )?;
    Ok("production adapter contracts are valid".to_string())
}

fn validate_database_boundary() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require("DatabaseBoundaryV0", is_guard_schema("DatabaseBoundaryV0"))?;
    require("no_database_mutation", has_production_contract("no_database_mutation"))?;
    Ok("database boundary contracts are valid".to_string())
}

fn validate_telemetry_adapters() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require(
        "TelemetryAdapterContractV0",
        is_guard_schema("TelemetryAdapterContractV0"),
    )?;
    require("bounded_output", has_production_contract("bounded_output"))?;
    Ok("telemetry adapter contracts are valid".to_string())
}

fn validate_telemetry_redaction() -> Result<String, String> {
    validate_schema_files(GUARD_PACKET_SCHEMAS)?;
    require(
        "TelemetryRedactionPolicyV0",
        is_guard_schema("TelemetryRedactionPolicyV0"),
    )?;
    require("redaction_required", has_production_contract("redaction_required"))?;
    Ok("telemetry redaction contracts are valid".to_string())
}

fn validate_remediator() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediatorReadinessV0",
        is_repair_readiness_schema("RemediatorReadinessV0"),
    )?;
    require_all(
        "repair-module",
        &[
            "intake",
            "permissions",
            "reproduction",
            "failure_taxonomy",
            "proof_plan",
            "report",
        ],
        has_repair_module,
    )?;
    require(
        "activation_deferred_to_s8",
        has_repair_boundary("activation_deferred_to_s8"),
    )?;
    Ok("Remediator readiness contracts are valid".to_string())
}

fn validate_remediation_intake() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediationIntakeV0",
        is_repair_readiness_schema("RemediationIntakeV0"),
    )?;
    require("intake", has_repair_module("intake"))?;
    Ok("remediation intake contracts are valid".to_string())
}

fn validate_remediation_permissions() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediationPermissionsV0",
        is_repair_readiness_schema("RemediationPermissionsV0"),
    )?;
    require("permissions", has_repair_module("permissions"))?;
    require(
        "no_secret_required_for_readiness",
        has_repair_boundary("no_secret_required_for_readiness"),
    )?;
    Ok("remediation permission contracts are valid".to_string())
}

fn validate_remediation_reproduction() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediationReproductionV0",
        is_repair_readiness_schema("RemediationReproductionV0"),
    )?;
    require("reproduction", has_repair_module("reproduction"))?;
    require(
        "no_patch_without_reproduction_plan",
        has_repair_boundary("no_patch_without_reproduction_plan"),
    )?;
    Ok("remediation reproduction contracts are valid".to_string())
}

fn validate_remediation_failure_taxonomy() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediationFailureTaxonomyV0",
        is_repair_readiness_schema("RemediationFailureTaxonomyV0"),
    )?;
    require("failure_taxonomy", has_repair_module("failure_taxonomy"))?;
    Ok("remediation failure taxonomy contracts are valid".to_string())
}

fn validate_remediation_proof_plan() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediationProofPlanV0",
        is_repair_readiness_schema("RemediationProofPlanV0"),
    )?;
    require("proof_plan", has_repair_module("proof_plan"))?;
    Ok("remediation proof plan contracts are valid".to_string())
}

fn validate_remediation_report() -> Result<String, String> {
    validate_schema_files(REPAIR_READINESS_PACKET_SCHEMAS)?;
    require(
        "RemediationReportV0",
        is_repair_readiness_schema("RemediationReportV0"),
    )?;
    require("report", has_repair_module("report"))?;
    require(
        "diagnosis_is_not_completed_repair",
        has_repair_boundary("diagnosis_is_not_completed_repair"),
    )?;
    Ok("remediation report contracts are valid".to_string())
}
