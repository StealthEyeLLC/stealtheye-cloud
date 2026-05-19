use std::env;
use std::fs;
use std::path::Path;

use secloud_control::{classify_tool_name, ALLOWED_TOOL_NAMES, BLOCKED_TOOL_NAMES};
use secloud_hypothesis::{is_hypothesis_schema, is_stop_condition, HYPOTHESIS_PACKET_SCHEMAS};
use secloud_learning::{has_promotion_rule, is_learning_schema, LEARNING_PACKET_SCHEMAS};
use secloud_packets::{FORBIDDEN_ROOT_FILES, REQUIRED_PACKET_SCHEMAS, REQUIRED_ROOT_FILES};
use secloud_proof_viewer::{is_panel_kind, is_viewer_schema, PROOF_VIEWER_PACKET_SCHEMAS};
use secloud_relay::validate_relay_markdown;
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
