import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'build-accelerator');
fs.mkdirSync(outDir, { recursive: true });

const requiredFiles = [
  'crates/secloud-build-accelerator/src/lib.rs',
  'crates/secloud-build-accelerator/Cargo.toml',
  '.github/workflows/proof-build-accelerator.yml',
  'docs/ONE_DROP_BUILD_MODE.md',
  'docs/MISSION_APPROVAL_ENVELOPE.md',
  'docs/BATCH_REPAIR_POLICY.md',
  'docs/MERGE_AWARE_HANDOFF.md',
  'docs/PHASE_TEMPLATE_SYSTEM.md',
  'docs/PROMPTS/NEXT_TAB_PROMPT.md',
  'docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md',
  'docs/S9_FINAL_REPORT.md',
];

const schemaNames = [
  'OneDropPlanV0',
  'MissionApprovalEnvelopeV0',
  'ApprovalCompressionPolicyV0',
  'NoMidpointAskPolicyV0',
  'RepoMutationBatchV0',
  'BatchRepairPlanV0',
  'MergeAwareNextActionV0',
  'StateConsistencyReportV0',
  'NoSilentDowngradePolicyV0',
  'FuturePhaseDefaultContractV0',
  'BuildVelocityReportV0',
  'S9BuildAcceleratorProofV0',
];

const requiredMarkers = [
  ['docs/ONE_DROP_BUILD_MODE.md', 'one mission approval'],
  ['docs/MISSION_APPROVAL_ENVELOPE.md', 'Stop only for true boundaries'],
  ['docs/BATCH_REPAIR_POLICY.md', 'inspect all failures before patching'],
  ['docs/MERGE_AWARE_HANDOFF.md', 'no-cleanup-PR'],
  ['docs/PHASE_TEMPLATE_SYSTEM.md', 'Future phase default contract'],
  ['docs/S9_FINAL_REPORT.md', 'No validator weakening'],
  ['docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md', 'one coherent repo mutation/drop'],
];

function readText(rel) {
  return fs.readFileSync(path.join(root, rel), 'utf8');
}

function exists(rel) {
  return fs.existsSync(path.join(root, rel));
}

const missingFiles = requiredFiles.filter((rel) => !exists(rel));
const missingSchemas = schemaNames.filter((name) => !exists(`schemas/${name}.schema.json`));
const missingMarkers = requiredMarkers.filter(([rel, marker]) => !readText(rel).includes(marker));

const docs = [
  'README.md',
  'docs/StealthEye_Cloud_Build_Plan.md',
  'STEALTHEYE_ACTIVE.md',
  'STEALTHEYE_RELAY.md',
  'STEALTHEYE_RELAY.json',
  'STEALTHEYE_SEAL.json',
  'NEXT_ACTION.md',
];
const docsText = Object.fromEntries(docs.map((rel) => [rel, readText(rel)]));

const forbiddenFiles = ['CLAUDE.md', '.github/copilot-instructions.md', '.cursorrules', 'soul.md', 'MEMORY.md', 'rules.md'];
const forbiddenPresent = forbiddenFiles.filter((rel) => exists(rel));

const stalePatterns = [
  'S9 setup PR',
  'S9 is selected as the next mission',
  'Open and prove the S9 setup PR',
  'setup branch active',
  'implementation branch is active',
  'Pending creation from the S9 implementation branch',
  'pending_pr_proof_wave',
  'Open the S9 implementation PR',
];
const staleFindings = [];
for (const [rel, text] of Object.entries(docsText)) {
  for (const pattern of stalePatterns) {
    if (text.includes(pattern)) staleFindings.push({ file: rel, pattern });
  }
}

const s9MergeSha = 'a5540d1fe77a0752a6a32b086a66b7b4bbec33ec';
const consistencyChecks = [
  { name: 'readme records s0-s9 green', passed: docsText['README.md'].includes('S0–S9 merged green') },
  { name: 'readme records no s10 implementation', passed: docsText['README.md'].includes('No S10 implementation has started') },
  { name: 'build plan records s9 crate', passed: docsText['docs/StealthEye_Cloud_Build_Plan.md'].includes('crates/secloud-build-accelerator') },
  { name: 'active records post-s9 cleanup', passed: docsText['STEALTHEYE_ACTIVE.md'].includes('Post-S9 truth cleanup') },
  { name: 'relay records s9 merge sha', passed: docsText['STEALTHEYE_RELAY.md'].includes(s9MergeSha) },
  { name: 'relay json records s9 merge sha', passed: docsText['STEALTHEYE_RELAY.json'].includes(s9MergeSha) },
  { name: 'seal records post-s9 cleanup', passed: docsText['STEALTHEYE_SEAL.json'].includes('post-s9-truth-cleanup') },
  { name: 'next action is define or choose s10', passed: docsText['NEXT_ACTION.md'].includes('Define or choose S10') },
  { name: 'next action remains merge-aware', passed: docsText['NEXT_ACTION.md'].includes('merge-aware') },
];

const buildVelocityReport = {
  schema_version: 'BuildVelocityReportV0',
  id: 's9-build-velocity-report',
  status: 'passed',
  metrics: {
    mission_approvals: 1,
    repo_mutation_batches_target: 1,
    pull_requests_target: 1,
    proof_waves_target: 1,
    batch_repairs_required: true,
    cleanup_prs_target: 0,
  },
  invariants: [
    'fast mode compresses routine confirmations only',
    'validators are not weakened',
    'proof gates are not skipped',
    'merge still requires green CI',
  ],
};

const confirmationFrictionLedger = {
  schema_version: 'ConfirmationFrictionEventV0',
  id: 's9-confirmation-friction-ledger',
  status: 'passed',
  events: [
    { class: 'routine_confirmation_avoided', action: 'safe file creation/update after mission approval' },
    { class: 'routine_confirmation_avoided', action: 'PR creation after coherent branch drop' },
    { class: 'true_boundary_preserved', action: 'standard public-safe boundaries remain hard stops' },
  ],
  invariants: ['no midpoint ask for routine action', 'true boundaries remain hard stops'],
};

const stateConsistencyReport = {
  schema_version: 'StateConsistencyReportV0',
  id: 's9-state-consistency-report',
  status: staleFindings.length === 0 && consistencyChecks.every((check) => check.passed) ? 'passed' : 'failed',
  consistency_checks: consistencyChecks,
  stale_findings: staleFindings,
  invariants: ['Active/Relay/Seal/Next Action align on post-S9 truth', 'no stale setup or active-PR language remains'],
};

const noCleanupPrReport = {
  schema_version: 'PostMergeTruthTemplateV0',
  id: 's9-no-cleanup-pr-report',
  status: staleFindings.length === 0 ? 'passed' : 'failed',
  stale_patterns: staleFindings,
  merge_aware_next_action: 'After the post-S9 truth cleanup merges green, define or choose S10 from current main.',
  invariants: ['no stale setup language', 'post-merge next action is explicit'],
};

const proofSummary = {
  schema_version: 'S9BuildAcceleratorProofV0',
  id: 's9-build-accelerator-proof',
  status: missingFiles.length === 0 && missingSchemas.length === 0 && missingMarkers.length === 0 && forbiddenPresent.length === 0 && stateConsistencyReport.status === 'passed' ? 'passed' : 'failed',
  missing_files: missingFiles,
  missing_schemas: missingSchemas,
  missing_markers: missingMarkers,
  forbidden_present: forbiddenPresent,
  artifacts: [
    '.stealtheye/build-accelerator/build-velocity-report.json',
    '.stealtheye/build-accelerator/confirmation-friction-ledger.json',
    '.stealtheye/build-accelerator/state-consistency-report.json',
    '.stealtheye/build-accelerator/no-cleanup-pr-report.json',
  ],
  invariants: ['No validator weakening', 'No proof weakening', 'No boundary relaxation'],
};

const artifacts = {
  'build-velocity-report.json': buildVelocityReport,
  'confirmation-friction-ledger.json': confirmationFrictionLedger,
  'state-consistency-report.json': stateConsistencyReport,
  'no-cleanup-pr-report.json': noCleanupPrReport,
  's9-build-accelerator-proof.json': proofSummary,
};

for (const [name, value] of Object.entries(artifacts)) {
  fs.writeFileSync(path.join(outDir, name), `${JSON.stringify(value, null, 2)}\n`);
}

if (proofSummary.status !== 'passed') {
  console.error(JSON.stringify(proofSummary, null, 2));
  process.exit(1);
}

console.log(JSON.stringify(proofSummary, null, 2));
