# MCP/App Control Plane

## Purpose

The StealthEye Cloud Control Plane exposes public-safe, high-level mission actions instead of raw primitive tools.

## S3 scope

S3 establishes:

1. closed public-safe tool registry
2. blocked raw tool names
3. tool identity schema
4. tool capability schema
5. permission envelope schema
6. result envelope schema
7. tool health schema
8. tool capability search schema
9. tool pack manifest schema
10. executable Rust registry tests

## Executable crate

```text
crates/secloud-control
```

## Allowed high-level tools

```text
secloud.status
secloud.capabilities
secloud.validate.relay
secloud.validate.seal
secloud.mission.plan
secloud.mission.status
secloud.bundle.validate
secloud.evidence.run
secloud.evidence.browser
secloud.worker.task
secloud.research.packet
```

## Blocked raw tool names

```text
run
exec
shell.run
write
delete
git
github
curl
fetch
browser.action
device.action
release
deploy
secret.read
payment
```

## Boundary

S3 does not expose uncontrolled MCP servers or raw write/execute/browser actions. It defines the governed public-safe surface that future adapters must obey.
