# StealthEye Capabilities

## Purpose

This file lists the public-safe capability surface for StealthEye Cloud.

## S3 capability classes

1. status and state reading
2. Relay validation
3. Seal validation
4. mission planning/status
5. file bundle validation
6. run evidence summaries
7. browser evidence summaries
8. worker task packet creation
9. research task packet creation
10. feature availability checks

## Allowed high-level tool names

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

## Rule

Expose mission-level public-safe actions. Do not expose raw primitive tools directly.
