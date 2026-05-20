import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'mission-gauntlet');
const commandDir = path.join(root, '.stealtheye', 'command-outbox');

const required = [
  'gauntlet-run-plan.json',
  'gauntlet-result.json',
  'approval-count-report.json',
  'github-capability-closure.json',
  'connector-capability-manifest.json',
  'current-main-head-proof.json',
  'required-check-resolver-report.json',
  'path-filter-pending-guard.json',
  'mission-control-plane.json',
  'mission-resource-mirror.json',
  'workflow-injection-guard-report.json',
  'prompt-to-script-firewall-report.json',
  'mission-replay-report.json',
  'synthetic-failure-report.json',
  'resume-torture-report.json',
  'boundary-quality-report.json',
  's12-proof.json'
];

const missing = required.filter((name) => !fs.existsSync(path.join(outDir, name)));
if (missing.length > 0) {
  console.error(`Missing S12 mission gauntlet artifacts: ${missing.join(', ')}`);
  process.exit(1);
}
for (const name of ['latest.json', 'history.jsonl']) {
  if (!fs.existsSync(path.join(commandDir, name))) {
    console.error(`Missing S12 command outbox artifact: ${name}`);
    process.exit(1);
  }
}

const proof = JSON.parse(fs.readFileSync(path.join(outDir, 's12-proof.json'), 'utf8'));
if (proof.status !== 'passed') {
  console.error('S12 mission gauntlet proof did not pass');
  console.error(JSON.stringify(proof, null, 2));
  process.exit(1);
}
const result = JSON.parse(fs.readFileSync(path.join(outDir, 'gauntlet-result.json'), 'utf8'));
if (result.bounded_missions < 5) {
  console.error('S12 gauntlet did not record at least 5 bounded missions');
  process.exit(1);
}
if (result.routine_midpoint_approvals !== 0) {
  console.error('S12 gauntlet recorded routine midpoint approvals');
  process.exit(1);
}
if (!result.synthetic_failure_repaired || !result.branch_pr_reuse_proven || !result.fresh_main_head_gate) {
  console.error('S12 gauntlet missing required repair/reuse/freshness proof flags');
  process.exit(1);
}
const approval = JSON.parse(fs.readFileSync(path.join(outDir, 'approval-count-report.json'), 'utf8'));
if (approval.initial_mission_approval !== 1 || approval.routine_midpoint_approvals !== 0) {
  console.error('S12 ApprovalCountReportV0 failed one-accept metric');
  process.exit(1);
}
const capability = JSON.parse(fs.readFileSync(path.join(outDir, 'github-capability-closure.json'), 'utf8'));
if (!capability.can.includes('read_workflow_runs') || !capability.cannot.includes('native_workflow_dispatch_from_current_connector')) {
  console.error('S12 GitHubCapabilityClosureV0 missing connector capability evidence');
  process.exit(1);
}
const manifest = JSON.parse(fs.readFileSync(path.join(outDir, 'connector-capability-manifest.json'), 'utf8'));
if (manifest.native_workflow_dispatch_exposed !== false || manifest.issue_comment_bridge_required !== true) {
  console.error('S12 connector manifest failed dispatch gap evidence');
  process.exit(1);
}
const firewall = JSON.parse(fs.readFileSync(path.join(outDir, 'prompt-to-script-firewall-report.json'), 'utf8'));
if (firewall.model_output_shell_sink_allowed !== false || firewall.schema_only_command_plan !== true) {
  console.error('S12 prompt-to-script firewall failed');
  process.exit(1);
}
const resume = JSON.parse(fs.readFileSync(path.join(outDir, 'resume-torture-report.json'), 'utf8'));
if (!resume.duplicate_branch_block || !resume.duplicate_pr_block || !resume.duplicate_commit_block || !resume.duplicate_merge_block) {
  console.error('S12 resume torture report failed duplicate-action protection');
  process.exit(1);
}
const latest = JSON.parse(fs.readFileSync(path.join(commandDir, 'latest.json'), 'utf8'));
if (latest.schema_version !== 'CommandResultOutboxV0' || !Array.isArray(latest.closed_command_set)) {
  console.error('S12 command outbox latest.json is invalid');
  process.exit(1);
}
const history = fs.readFileSync(path.join(commandDir, 'history.jsonl'), 'utf8').trim().split('\n').filter(Boolean);
if (history.length < 1) {
  console.error('S12 command outbox history is empty');
  process.exit(1);
}
console.log(`Validated ${required.length} S12 mission gauntlet artifacts plus command outbox.`);
