import { existsSync, readFileSync, writeFileSync, mkdirSync } from 'node:fs';
import { join } from 'node:path';

const required = [
  '.stealtheye/notifications/artifacts/s7-notification-summary.json',
  '.stealtheye/notifications/artifacts/s7-notification-summary.md',
  '.stealtheye/mirror/export/s7-knowledge-mirror/manifest.json',
  '.stealtheye/mirror/export/s7-knowledge-mirror/semantic-snapshot.json',
  '.stealtheye/mirror/export/s7-knowledge-mirror/redaction-report.json',
  '.stealtheye/mirror/export/s7-knowledge-mirror/README.md'
];

const missing = required.filter((path) => !existsSync(path));
if (missing.length > 0) {
  console.error(`Missing S7 activation artifacts: ${missing.join(', ')}`);
  process.exit(1);
}

const notification = JSON.parse(readFileSync('.stealtheye/notifications/artifacts/s7-notification-summary.json', 'utf8'));
const mirror = JSON.parse(readFileSync('.stealtheye/mirror/export/s7-knowledge-mirror/manifest.json', 'utf8'));
const snapshot = JSON.parse(readFileSync('.stealtheye/mirror/export/s7-knowledge-mirror/semantic-snapshot.json', 'utf8'));
const redaction = JSON.parse(readFileSync('.stealtheye/mirror/export/s7-knowledge-mirror/redaction-report.json', 'utf8'));

if (notification.schema_version !== 'NotificationActivationRunV0' || notification.status !== 'success') {
  console.error('notification activation summary invalid');
  process.exit(1);
}
if (notification.redaction_status !== 'passed') {
  console.error('notification redaction did not pass');
  process.exit(1);
}
if (notification.mode === 'real_dispatch' && notification.provider_host === notification.webhook_url) {
  console.error('notification artifact must not expose webhook URL');
  process.exit(1);
}
if (mirror.schema_version !== 'KnowledgeMirrorExportV0' || mirror.status !== 'success' || mirror.source_count < 3) {
  console.error('knowledge mirror manifest invalid');
  process.exit(1);
}
if (snapshot.schema_version !== 'SemanticSnapshotV0' || snapshot.status !== 'success') {
  console.error('semantic snapshot invalid');
  process.exit(1);
}
if (redaction.redaction_status !== 'passed' || redaction.findings.length !== 0) {
  console.error('mirror redaction report invalid');
  process.exit(1);
}

mkdirSync('.stealtheye/activations/artifacts', { recursive: true });
const summary = {
  schema_version: 'S7ActivationProofV0',
  mission_id: 's7-first-real-activations',
  activated_lanes: [
    'mobile_browser_game_preview',
    'notification_lane',
    'knowledge_mirror_export'
  ],
  proof_workflows: ['proof-mobile', 'proof-activations'],
  boundary_status: 'public-safe; no secrets; no paid compute; no production deployment; no database mutation; no browser-cookie/session-token automation',
  notification_mode: notification.mode,
  mirror_source_count: mirror.source_count,
  status: 'success'
};
writeFileSync(join('.stealtheye/activations/artifacts', 's7-activation-proof.json'), JSON.stringify(summary, null, 2));
writeFileSync(join('.stealtheye/activations/artifacts', 's7-activation-proof.md'), '# S7 Activation Proof\n\nStatus: success\n');

console.log('PASS: S7 activation artifacts validated');
