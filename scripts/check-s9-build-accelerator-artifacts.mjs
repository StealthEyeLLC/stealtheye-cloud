import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'build-accelerator');
const required = [
  'build-velocity-report.json',
  'confirmation-friction-ledger.json',
  'state-consistency-report.json',
  'no-cleanup-pr-report.json',
  's9-build-accelerator-proof.json',
];

const missing = required.filter((name) => !fs.existsSync(path.join(outDir, name)));
if (missing.length > 0) {
  console.error(`Missing S9 build accelerator artifacts: ${missing.join(', ')}`);
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

console.log(`Validated ${required.length} S9 build accelerator artifacts.`);
