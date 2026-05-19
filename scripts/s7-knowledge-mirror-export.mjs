import { copyFileSync, existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import { dirname, join } from 'node:path';

const outRoot = '.stealtheye/mirror/export/s7-knowledge-mirror';
const bundleRoot = join(outRoot, 'bundle');
mkdirSync(bundleRoot, { recursive: true });

const sources = [
  'README.md',
  'AGENTS.md',
  'STEALTHEYE_DECISIONS.md',
  'docs/StealthEye_Cloud_Build_Plan.md',
  'docs/S6_ZERO_TRUST_CROSS_CLOUD_GATEWAY.md',
  'docs/S7_FIRST_REAL_ACTIVATIONS.md',
  'docs/HANDOFF_AND_CONTINUATION.md'
];

const credentialShapes = [
  { name: 'github_classic_token', pattern: /ghp_[A-Za-z0-9_]{20,}/g },
  { name: 'github_fine_grained_token', pattern: /github_pat_[A-Za-z0-9_]{20,}/g },
  { name: 'slack_token', pattern: /xox[baprs]-[A-Za-z0-9-]{10,}/g },
  { name: 'openai_style_secret', pattern: /sk-[A-Za-z0-9]{20,}/g },
  { name: 'aws_access_key_id', pattern: /AKIA[0-9A-Z]{16}/g },
  { name: 'private_key_block', pattern: /-----BEGIN [A-Z ]*PRIVATE KEY-----/g }
];

const copied = [];
const redactionFindings = [];
const semanticCards = [];

for (const source of sources) {
  if (!existsSync(source)) continue;
  const content = readFileSync(source, 'utf8');
  for (const shape of credentialShapes) {
    const matches = content.match(shape.pattern) || [];
    for (const _match of matches) {
      redactionFindings.push({ source, kind: shape.name, action: 'blocked' });
    }
  }
  const destination = join(bundleRoot, source);
  mkdirSync(dirname(destination), { recursive: true });
  copyFileSync(source, destination);
  copied.push({ source, destination });
  const headings = content
    .split('\n')
    .filter((line) => line.startsWith('#'))
    .slice(0, 12)
    .map((line) => line.replace(/^#+\s*/, '').trim());
  semanticCards.push({ source, headings, byte_length: Buffer.byteLength(content, 'utf8') });
}

if (redactionFindings.length > 0) {
  mkdirSync(outRoot, { recursive: true });
  writeFileSync(join(outRoot, 'redaction-report.json'), JSON.stringify({
    schema_version: 'KnowledgeMirrorRedactionV0',
    status: 'failure',
    findings: redactionFindings
  }, null, 2));
  console.error('Knowledge mirror redaction failed');
  process.exit(1);
}

const manifest = {
  schema_version: 'KnowledgeMirrorExportV0',
  mission_id: 's7-first-real-activations',
  lane: 'knowledge_mirror_export',
  bundle_path: bundleRoot,
  manifest_path: join(outRoot, 'manifest.json'),
  semantic_snapshot_path: join(outRoot, 'semantic-snapshot.json'),
  redaction_report_path: join(outRoot, 'redaction-report.json'),
  source_count: copied.length,
  sources: copied,
  external_sync: false,
  status: 'success'
};

const semanticSnapshot = {
  schema_version: 'SemanticSnapshotV0',
  mission_id: 's7-first-real-activations',
  generated_from: copied.map((item) => item.source),
  cards: semanticCards,
  status: 'success'
};

const redactionReport = {
  schema_version: 'KnowledgeMirrorRedactionV0',
  mission_id: 's7-first-real-activations',
  scanned_sources: copied.map((item) => item.source),
  findings: [],
  redaction_status: 'passed',
  private_data_excluded: true,
  status: 'success'
};

writeFileSync(join(outRoot, 'manifest.json'), JSON.stringify(manifest, null, 2));
writeFileSync(join(outRoot, 'semantic-snapshot.json'), JSON.stringify(semanticSnapshot, null, 2));
writeFileSync(join(outRoot, 'redaction-report.json'), JSON.stringify(redactionReport, null, 2));
writeFileSync(join(outRoot, 'README.md'), '# S7 Knowledge Mirror Export\n\nStatus: success\n\nThis bundle is public-safe, static, and not a live external sync.\n');

if (copied.length < 3) {
  console.error('Knowledge mirror export copied too few sources');
  process.exit(1);
}

console.log(`PASS: S7 knowledge mirror exported ${copied.length} public-safe sources`);
