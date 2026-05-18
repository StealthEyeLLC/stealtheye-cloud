import { existsSync, readFileSync } from 'node:fs';

const required = [
  '.stealtheye/browser/artifacts/browser-summary.json',
  '.stealtheye/browser/artifacts/browser-summary.md',
  '.stealtheye/browser/artifacts/browser-console-errors.json',
  '.stealtheye/browser/artifacts/browser-network-failures.json',
  '.stealtheye/browser/artifacts/browser-dom-sketch.json',
  '.stealtheye/browser/artifacts/browser-proof.png'
];

const missing = required.filter((path) => !existsSync(path));
if (missing.length > 0) {
  console.error(`Missing browser artifacts: ${missing.join(', ')}`);
  process.exit(1);
}

const summary = JSON.parse(readFileSync('.stealtheye/browser/artifacts/browser-summary.json', 'utf8'));
if (summary.schema_version !== 'BrowserArtifactIndexV0') {
  console.error('browser summary schema_version must be BrowserArtifactIndexV0');
  process.exit(1);
}
if (summary.status !== 'success') {
  console.error(`browser summary status must be success, got ${summary.status}`);
  process.exit(1);
}
if (!summary.dom_sketch || summary.dom_sketch.marker !== 'browser-proof') {
  console.error('browser DOM sketch marker is invalid');
  process.exit(1);
}

console.log('PASS: browser artifacts validated');
