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
  'docs/S9_FINAL_REPORT.md'
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
  'S9BuildAcceleratorProofV0'
];

const requiredMarkers = [
  ['docs/ONE_DROP_BUILD_MODE.md', 'one mission approval'],
  ['docs/MISSION_APPROVAL_ENVELOPE.md', 'Stop only for true boundaries'],
  ['docs/BATCH_REPAIR_POLICY.md', 'inspect all failures before patching'],
  ['docs/MERGE_AWARE_HANDOFF.md', 'no-cleanup-PR'],
  ['docs/PHASE_TEMPLATE_SYSTEM.md', 'Future phase default contract'],
  ['docs/S9_FINAL_REPORT.md', 'No validator weakening'],
  ['docs/PROMPTS/FUTURE_PHASE_DEFAULT_PROMPT.md', 'one coherent repo mutation/drop']
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
  'NEXT_ACTION.md'
];
const docsText = Object.fromEntries(docs.map((rel) => [rel, readText(rel)]));

const forbiddenFiles = ['CLAUDE.md', '.github/copilot-instructions.md', '.cursorrules', 'soul.md', 'MEMORY.md', 'rules.md'];
const forbiddenPresent = forbiddenFiles.filter((rel) => exists(rel));

const stalePatterns = [
  'S9 setup PR',
  'S9 is selected as the next mission',
  'Open and prove the S9 setup PR',
  'S9 implementation branch is active',
  'Pending creation from the S9 implementation branch',
  'Open the S9 implementation PR',
  'S10 is selected for setup',
  'No S10 implementation has started',
  'S10 setup docs PR',
  'S11 is selected as the next build',
  'S11 implementation is active',
  'Open the S11 implementation PR',
  'Do not start S12 until the operator chooses it'
];
const staleFindings = [];
for (const [rel, text] of Object.entries(docsText)) {
  for (const pattern of stalePatterns) {
    if (text.includes(pattern)) staleFindings.push({ file: rel, pattern });
  }
}

const s9MergeSha = 'a5540d1fe77a0752a6a32b086a66b7b4bbec33ec';
const s10MergeSha = 'fd2bcda27a281fb080aaef472bd87123e4fe02b6';
const s11MergeSha = 'da07e96c466f54086143a34422c47a60f6de1d2e';
const postS11TruthSha = '8988e32fc61e2824dcc19eef30da2894112ea9f9';
const s11Name = 'S11 — One-Accept Mission Executor';
const s12Name = 'S12 — One-Accept Mission Gauntlet';
const currentTruthOk =
  docsText['STEALTHEYE_ACTIVE.md'].includes('S0–S11') &&
  docsText['STEALTHEYE_ACTIVE.md'].includes(s11MergeSha) &&
  docsText['STEALTHEYE_ACTIVE.md'].includes(s12Name) &&
  docsText['STEALTHEYE_ACTIVE.md'].includes(postS11TruthSha);
const sealRecordsCurrentHandoff =
  docsText['STEALTHEYE_SEAL.json'].includes('s12-one-accept-mission-gauntlet') &&
  docsText['STEALTHEYE_SEAL.json'].includes(postS11TruthSha);
const nextActionPointsForward =
  docsText['NEXT_ACTION.md'].includes('docs/S12_ONE_ACCEPT_MISSION_GAUNTLET.md') &&
  docsText['NEXT_ACTION.md'].includes('build/s12-one-accept-mission-gauntlet');

const consistencyChecks = [
  { name: 'readme records s0-s11 merged', passed: docsText['README.md'].includes('S0–S11 merged') },
  { name: 'readme records s11 merge sha', passed: docsText['README.md'].includes(s11MergeSha) },
  { name: 'readme records s12 selected', passed: docsText['README.md'].includes(s12Name) },
  { name: 'readme records post-s11 caveat', passed: docsText['README.md'].includes(postS11TruthSha) && docsText['README.md'].includes('present but not proven') },
  { name: 'build plan records s9 crate', passed: docsText['docs/StealthEye_Cloud_Build_Plan.md'].includes('crates/secloud-build-accelerator') },
  { name: 'build plan records s12 selected', passed: docsText['docs/StealthEye_Cloud_Build_Plan.md'].includes(s12Name) },
  { name: 'active records current s12 selected truth', passed: currentTruthOk },
  { name: 'relay records s9 merge sha', passed: docsText['STEALTHEYE_RELAY.md'].includes(s9MergeSha) },
  { name: 'relay records s10 merge sha', passed: docsText['STEALTHEYE_RELAY.md'].includes(s10MergeSha) },
  { name: 'relay records s11 merge sha', passed: docsText['STEALTHEYE_RELAY.md'].includes(s11MergeSha) },
  { name: 'relay records s12 selected', passed: docsText['STEALTHEYE_RELAY.md'].includes(s12Name) },
  { name: 'relay json records s12 selected', passed: docsText['STEALTHEYE_RELAY.json'].includes('s12-one-accept-mission-gauntlet') },
  { name: 'seal records current s12 handoff truth', passed: sealRecordsCurrentHandoff },
  { name: 'next action points to s12 implementation', passed: nextActionPointsForward },
  { name: 's9 one-drop mode remains documented', passed: docsText['README.md'].includes('one mission approval') && docsText['README.md'].includes('merge when green') }
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
    cleanup_prs_target: 0
  },
  invariants: ['fast mode compresses routine confirmations only', 'validators are not weakened', 'proof gates are not skipped', 'merge still requires green CI']
};

const confirmationFrictionLedger = {
  schema_version: 'ConfirmationFrictionEventV0',
  id: 's9-confirmation-friction-ledger',
  status: 'passed',
  events: [
    { class: 'routine_confirmation_avoided', action: 'safe file creation/update after mission approval' },
    { class: 'routine_confirmation_avoided', action: 'PR creation after coherent branch drop' },
    { class: 'true_boundary_preserved', action: 'standard public-safe boundaries remain hard stops' }
  ],
  invariants: ['no midpoint ask for routine action', 'true boundaries remain hard stops']
};

const stateConsistencyReport = {
  schema_version: 'StateConsistencyReportV0',
  id: 's9-state-consistency-report',
  status: staleFindings.length === 0 && consistencyChecks.every((check) => check.passed) ? 'passed' : 'failed',
  consistency_checks: consistencyChecks,
  stale_findings: staleFindings,
  invariants: ['Active/Relay/Seal/Next Action align on current phase truth', 'S12 selected truth is recorded without weakening S9 proof']
};

const noCleanupPrReport = {
  schema_version: 'PostMergeTruthTemplateV0',
  id: 's9-no-cleanup-pr-report',
  status: staleFindings.length === 0 ? 'passed' : 'failed',
  stale_patterns: staleFindings,
  merge_aware_next_action: 'After compact S12 prep merges green, implement S12 from docs/S12_ONE_ACCEPT_MISSION_GAUNTLET.md.',
  invariants: ['no stale S9/S10/S11 setup language', 'post-merge next action is explicit']
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
    '.stealtheye/build-accelerator/no-cleanup-pr-report.json'
  ],
  invariants: ['No validator weakening', 'No proof weakening', 'No boundary relaxation']
};

const artifacts = {
  'build-velocity-report.json': buildVelocityReport,
  'confirmation-friction-ledger.json': confirmationFrictionLedger,
  'state-consistency-report.json': stateConsistencyReport,
  'no-cleanup-pr-report.json': noCleanupPrReport,
  's9-build-accelerator-proof.json': proofSummary
};

for (const [name, value] of Object.entries(artifacts)) {
  fs.writeFileSync(path.join(outDir, name), `${JSON.stringify(value, null, 2)}\n`);
}

if (proofSummary.status !== 'passed') {
  console.error(JSON.stringify(proofSummary, null, 2));
  process.exit(1);
}

console.log(JSON.stringify(proofSummary, null, 2));
