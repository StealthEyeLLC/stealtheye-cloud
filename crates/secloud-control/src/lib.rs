//! Public-safe control-plane registry for StealthEye Cloud S3.

pub const ALLOWED_TOOL_NAMES: &[&str] = &[
    "secloud.status",
    "secloud.capabilities",
    "secloud.validate.relay",
    "secloud.validate.seal",
    "secloud.mission.plan",
    "secloud.mission.status",
    "secloud.bundle.validate",
    "secloud.evidence.run",
    "secloud.evidence.browser",
    "secloud.worker.task",
    "secloud.research.packet",
];

pub const BLOCKED_TOOL_NAMES: &[&str] = &[
    "run",
    "exec",
    "shell.run",
    "write",
    "delete",
    "git",
    "github",
    "curl",
    "fetch",
    "browser.action",
    "device.action",
    "release",
    "deploy",
    "secret.read",
    "payment",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolNameVerdict {
    pub allowed: bool,
    pub blocked: bool,
    pub reason: String,
}

pub fn classify_tool_name(name: &str) -> ToolNameVerdict {
    if BLOCKED_TOOL_NAMES.contains(&name) {
        return ToolNameVerdict {
            allowed: false,
            blocked: true,
            reason: "raw or unsafe tool name is blocked".to_string(),
        };
    }
    if ALLOWED_TOOL_NAMES.contains(&name) {
        return ToolNameVerdict {
            allowed: true,
            blocked: false,
            reason: "public-safe high-level tool name is allowed".to_string(),
        };
    }
    ToolNameVerdict {
        allowed: false,
        blocked: false,
        reason: "tool name is not in the closed registry".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn allows_high_level_status_tool() {
        let verdict = classify_tool_name("secloud.status");
        assert!(verdict.allowed);
        assert!(!verdict.blocked);
    }

    #[test]
    fn blocks_raw_exec_tool() {
        let verdict = classify_tool_name("exec");
        assert!(!verdict.allowed);
        assert!(verdict.blocked);
    }

    #[test]
    fn rejects_unknown_names() {
        let verdict = classify_tool_name("unknown.tool");
        assert!(!verdict.allowed);
        assert!(!verdict.blocked);
    }
}
