import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'mission-executor');
const required = [
  'github-capability-preflight.json',
  'mission-lease.json',
  'mission-executor-request.json',
  'mission-plan.json',
  'batch-repo-mutation.json',
  'pr-controller-report.json',
  'proof-controller-report.json',
  'proof-repair-loop-report.json',
  'merge-when-green-report.json',
  'post-merge-proof-freshness-report.json',
  'boundary-stop-report.json',
  'mission-journal.jsonl',
  'approval-count-report.json',
  'mission-executor-proof.json'
];

const missing = required.filter((name) => !fs.existsSync(path.join(outDir, name)));
if (missing.length > 0) {
  console.error(`Missing S11 mission executor artifacts: ${missing.join(', ')}`);
  process.exit(1);
}

const proof = JSON.parse(fs.readFileSync(path.join(outDir, 'mission-executor-proof.json'), 'utf8'));
if (proof.status !== 'passed') {
  console.error('S11 mission executor proof did not pass');
  console.error(JSON.stringify(proof, null, 2));
  process.exit(1);
}
if (proof.approval_count.initial_mission_approval !== 1 || proof.approval_count.routine_midpoint_approvals !== 0) {
  console.error('ApprovalCountReportV0 failed one-accept metric');
  process.exit(1);
}
const freshness = JSON.parse(fs.readFileSync(path.join(outDir, 'post-merge-proof-freshness-report.json'), 'utf8'));
if (freshness.direct_main_mutation_requires_workflow_dispatch !== true || freshness.unverified_truth_commit_allowed !== false) {
  console.error('PostMergeProofFreshnessGateV0 failed S11 freshness invariant');
  process.exit(1);
}
const journal = fs.readFileSync(path.join(outDir, 'mission-journal.jsonl'), 'utf8').trim().split('\n').filter(Boolean);
if (journal.length < 3) {
  console.error('MissionJournalV0 JSONL is missing required receipts');
  process.exit(1);
}
console.log(`Validated ${required.length} S11 mission executor artifacts.`);
