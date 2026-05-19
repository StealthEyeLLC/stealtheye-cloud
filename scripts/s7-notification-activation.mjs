import { mkdirSync, writeFileSync } from 'node:fs';
import { join } from 'node:path';

const artifactDir = '.stealtheye/notifications/artifacts';
mkdirSync(artifactDir, { recursive: true });

const event = process.env.STEALTHEYE_NOTIFICATION_EVENT || 's7_activation_ci_probe';
const webhookUrl = process.env.STEALTHEYE_NOTIFICATION_WEBHOOK_URL || '';
const realDispatchEnabled = process.env.STEALTHEYE_NOTIFICATION_REAL_DISPATCH === 'true';

const publicSafeMessage = {
  product: 'StealthEye Cloud',
  mission_id: 's7-first-real-activations',
  event,
  status: 'ci_probe',
  decision_boundary: false,
  public_safe: true
};

const serialized = JSON.stringify(publicSafeMessage);
const forbiddenPatterns = [
  /ghp_[A-Za-z0-9_]{20,}/,
  /github_pat_[A-Za-z0-9_]{20,}/,
  /xox[baprs]-[A-Za-z0-9-]{10,}/,
  /sk-[A-Za-z0-9]{20,}/,
  /AKIA[0-9A-Z]{16}/,
  /-----BEGIN [A-Z ]*PRIVATE KEY-----/
];

const redactionFailures = forbiddenPatterns
  .map((pattern) => pattern.test(serialized) ? pattern.source : null)
  .filter(Boolean);

if (redactionFailures.length > 0) {
  const failure = {
    schema_version: 'NotificationActivationRunV0',
    lane: 'notification_lane',
    mode: 'dry_run',
    event,
    dispatch_status: 'blocked_by_redaction',
    redaction_status: 'failed',
    secret_policy: 'secret values are never logged or exported',
    redaction_failures: redactionFailures,
    status: 'failure'
  };
  writeFileSync(join(artifactDir, 's7-notification-summary.json'), JSON.stringify(failure, null, 2));
  process.exit(1);
}

let mode = 'dry_run';
let dispatchStatus = 'dry_run_no_secret_or_enable_flag';
let httpStatus = null;
let providerHost = null;

if (webhookUrl && realDispatchEnabled) {
  mode = 'real_dispatch';
  const url = new URL(webhookUrl);
  providerHost = url.host;
  const response = await fetch(webhookUrl, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: serialized
  });
  httpStatus = response.status;
  dispatchStatus = response.ok ? 'real_dispatch_success' : 'real_dispatch_failed';
  if (!response.ok) {
    const failure = {
      schema_version: 'NotificationActivationRunV0',
      lane: 'notification_lane',
      mode,
      event,
      dispatch_status: dispatchStatus,
      http_status: httpStatus,
      provider_host: providerHost,
      redaction_status: 'passed',
      secret_policy: 'webhook secret was used but not logged or exported',
      status: 'failure'
    };
    writeFileSync(join(artifactDir, 's7-notification-summary.json'), JSON.stringify(failure, null, 2));
    process.exit(1);
  }
}

const summary = {
  schema_version: 'NotificationActivationRunV0',
  mission_id: 's7-first-real-activations',
  lane: 'notification_lane',
  mode,
  event,
  dispatch_status: dispatchStatus,
  http_status: httpStatus,
  provider_host: providerHost,
  redaction_status: 'passed',
  secret_policy: mode === 'real_dispatch'
    ? 'real dispatch requires STEALTHEYE_NOTIFICATION_WEBHOOK_URL and STEALTHEYE_NOTIFICATION_REAL_DISPATCH=true; secret value omitted from artifacts'
    : 'dry-run proof requires no secret and sends no external request',
  payload_preview: publicSafeMessage,
  status: 'success'
};

writeFileSync(join(artifactDir, 's7-notification-summary.json'), JSON.stringify(summary, null, 2));
writeFileSync(join(artifactDir, 's7-notification-summary.md'), `# S7 Notification Activation\n\nStatus: success\nMode: ${mode}\nDispatch: ${dispatchStatus}\n`);
console.log(`PASS: S7 notification activation ${mode}`);
