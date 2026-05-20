import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'assistant-optimizer');
const required = [
  'assistant-operating-profile.json',
  'mission-intake-packet.json',
  'context-load-plan.json',
  'repo-truth-first-report.json',
  'tool-use-plan.json',
  'tool-fallback-report.json',
  'low-attention-mode-report.json',
  'handoff-quality-report.json',
  'prompt-compression-report.json',
  'scope-drift-guard-report.json',
  'proof-awareness-report.json',
  'repair-triage-plan.json',
  'capability-reality-map.json',
  'read-only-verification-report.json',
  'assistant-self-audit.json',
  'build-cockpit-card.json',
  'assistant-trace-digest.json',
  's10-assistant-optimizer-proof.json'
];

const missing = required.filter((name) => !fs.existsSync(path.join(outDir, name)));
if (missing.length > 0) {
  console.error(`Missing S10 assistant optimizer artifacts: ${missing.join(', ')}`);
  process.exit(1);
}

for (const name of required) {
  const artifact = JSON.parse(fs.readFileSync(path.join(outDir, name), 'utf8'));
  if (artifact.status !== 'passed') {
    console.error(`${name} did not pass`);
    console.error(JSON.stringify(artifact, null, 2));
    process.exit(1);
  }
  if (!Array.isArray(artifact.invariants) || artifact.invariants.length === 0) {
    console.error(`${name} is missing invariants`);
    process.exit(1);
  }
}

console.log(`Validated ${required.length} S10 assistant optimizer artifacts.`);
