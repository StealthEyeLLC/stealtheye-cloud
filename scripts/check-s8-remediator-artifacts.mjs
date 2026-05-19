import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'remediator');

function readJson(relativePath) {
  const file = path.join(outDir, relativePath);
  if (!existsSync(file)) {
    throw new Error(`missing S8 remediator artifact: ${relativePath}`);
  }
  return JSON.parse(readFileSync(file, 'utf8'));
}

function assert(condition, message) {
  if (!condition) {
    throw new Error(message);
  }
}

const report = readJson('remediation-report.json');
const diagnosisOnly = readJson('diagnosis-only-report.json');
const receipt = readJson('execution-receipt.json');
const commercial = readJson('commercial.json');
const reproduction = readJson('reproduction.json');
const proofPlan = readJson('proof-plan.json');
const patch = readJson('bounded-patch.json');

assert(report.schema_version === 'RemediationReportV0', 'report schema mismatch');
assert(report.status === 'remediated', 'report must claim remediated only for proven flow');
assert(report.failure_reproduced === true, 'remediation report missing reproduced failure');
assert(report.bounded_patch_applied === true, 'remediation report missing bounded patch');
assert(report.proof_gates_passed === true, 'remediation report missing green proof gates');
assert(reproduction.reproduced === true, 'reproduction artifact must prove failure reproduction');
assert(proofPlan.requires_reproduction === true, 'proof plan must require reproduction');
assert(proofPlan.requires_bounded_patch === true, 'proof plan must require bounded patch');
assert(proofPlan.requires_green_proof === true, 'proof plan must require green proof');
assert(patch.changedLines === 1, 'bounded patch must remain one changed line in proof fixture');
assert(diagnosisOnly.status === 'diagnosis_only', 'diagnosis-only artifact missing');
assert(diagnosisOnly.failure_reproduced === false, 'diagnosis-only artifact cannot reproduce failure');
assert(!diagnosisOnly.claim.toLowerCase().includes('remediated'), 'diagnosis-only claim must not claim remediation');
assert(receipt.status === 'remediated', 'execution receipt status mismatch');
assert(Array.isArray(receipt.artifacts) && receipt.artifacts.length >= 3, 'execution receipt missing artifacts');
assert(commercial.billing_activated === false, 'commercial artifact must not activate billing');

console.log('S8 Remediator artifacts valid.');
