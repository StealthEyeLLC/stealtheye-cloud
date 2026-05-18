use std::env;
use std::fs;
use std::path::Path;

use secloud_packets::{FORBIDDEN_ROOT_FILES, REQUIRED_PACKET_SCHEMAS, REQUIRED_ROOT_FILES};
use secloud_relay::validate_relay_markdown;
use secloud_seal::validate_seal_json_text;

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
}

fn doctor() -> Result<String, String> {
    validate_root_files()?;
    validate_schemas()?;
    validate_relay()?;
    validate_seal()?;
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
    let content = fs::read_to_string(path).map_err(|err| format!("failed to read {path}: {err}"))?;
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
