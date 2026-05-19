import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const outDir = path.join(root, '.stealtheye', 'remediator');
const workDir = path.join(outDir, 'workspaces', 'synthetic-broken-repo');
const artifactsDir = path.join(outDir, 'artifacts');

function ensureDir(dir) {
  mkdirSync(dir, { recursive: true });
}

function writeJson(file, value) {
  writeFileSync(file, `${JSON.stringify(value, null, 2)}\n`);
}

function runNodeTest(cwd) {
  try {
    const stdout = execFileSync('node', ['--test', 'test/calc.test.mjs'], {
      cwd,
      encoding: 'utf8',
      stdio: ['ignore', 'pipe', 'pipe'],
    });
    return { exitCode: 0, output: stdout };
  } catch (error) {
    return {
      exitCode: typeof error.status === 'number' ? error.status : 1,
      output: `${error.stdout ?? ''}${error.stderr ?? ''}`,
    };
  }
}

function resetWorkspace() {
  rmSync(outDir, { recursive: true, force: true });
  ensureDir(path.join(workDir, 'src'));
  ensureDir(path.join(workDir, 'test'));
  ensureDir(artifactsDir);
  writeFileSync(
    path.join(workDir, 'src', 'calc.mjs'),
    'export function add(a, b) {\n  return a - b;\n}\n',
  );
  writeFileSync(
    path.join(workDir, 'test', 'calc.test.mjs'),
    "import assert from 'node:assert/strict';\nimport test from 'node:test';\nimport { add } from '../src/calc.mjs';\n\ntest('add sums positive integers', () => {\n  assert.equal(add(2, 3), 5);\n});\n",
  );
}

function applyBoundedPatch() {
  const target = path.join(workDir, 'src', 'calc.mjs');
  const before = readFileSync(target, 'utf8');
  const after = before.replace('return a - b;', 'return a + b;');
  if (before === after) {
    throw new Error('bounded patch did not modify the expected line');
  }
  writeFileSync(target, after);
  return {
    file: 'src/calc.mjs',
    before: 'return a - b;',
    after: 'return a + b;',
    changedLines: 1,
  };
}

function main() {
  resetWorkspace();

  const intake = {
    schema_version: 'RemediationIntakeV0',
    repo_ref: 'synthetic-broken-repo',
    reported_failure: 'node test for add(2, 3) returns 5 fails',
    requested_scope: ['src/calc.mjs', 'test/calc.test.mjs'],
    intake_kind: 'ci_failure',
    public_safe: true,
  };

  const permissions = {
    schema_version: 'RemediationPermissionsV0',
    allowed_actions: ['read_repo', 'discover_commands', 'run_bounded_proof', 'apply_bounded_patch', 'emit_report'],
    forbidden_actions: ['read_secrets', 'browser_cookie_session_automation', 'paid_compute', 'production_deployment', 'database_mutation'],
    requires_explicit_approval_for: ['secrets', 'paid_compute', 'production_deployment', 'database_mutation'],
  };

  const realityMap = {
    schema_version: 'RemediationRealityMapV0',
    files_observed: ['src/calc.mjs', 'test/calc.test.mjs'],
    languages: ['javascript'],
    candidate_entrypoints: ['node --test test/calc.test.mjs'],
    private_material_detected: false,
  };

  const commandDiscovery = {
    schema_version: 'RemediationCommandDiscoveryV0',
    discovered_commands: ['node --test test/calc.test.mjs'],
    selected_reproduction_command: 'node --test test/calc.test.mjs',
    bounded: true,
    secret_dependent: false,
  };

  const environment = {
    schema_version: 'RemediationEnvironmentV0',
    runtime: `node ${process.version}`,
    platform: process.platform,
    paid_compute_required: false,
    environment_notes: ['uses only built-in node:test and assert modules'],
  };

  const firstRun = runNodeTest(workDir);
  if (firstRun.exitCode === 0) {
    throw new Error('synthetic failure did not reproduce; proof would be diagnosis-only');
  }

  const reproduction = {
    schema_version: 'RemediationReproductionV0',
    command: 'node --test test/calc.test.mjs',
    reproduced: true,
    exit_code: firstRun.exitCode,
    evidence_excerpt: firstRun.output.slice(0, 1200),
    diagnosis_only_if_unreproduced: false,
  };

  const taxonomy = {
    schema_version: 'RemediationFailureTaxonomyV0',
    failure_class: 'test_failure',
    confidence: 0.95,
    reproduction_required: true,
  };

  const localization = {
    schema_version: 'RemediationLocalizationV0',
    suspect_files: ['src/calc.mjs'],
    signals: ['failing_command', 'stderr_excerpt', 'changed_file_hint', 'minimal_reproduction_path'],
  };

  const repairStrategy = {
    schema_version: 'RemediationRepairStrategyV0',
    selected_strategy: 'minimal_patch',
    requires_reproduction: true,
    rationale: 'single-line arithmetic implementation contradicts reproduced test expectation',
  };

  const boundedPatch = applyBoundedPatch();
  const secondRun = runNodeTest(workDir);
  if (secondRun.exitCode !== 0) {
    throw new Error(`bounded patch failed proof rerun: ${secondRun.output}`);
  }

  const patchTournament = {
    schema_version: 'RemediationPatchTournamentV0',
    candidates: ['minimal arithmetic correction'],
    winner: 'minimal arithmetic correction',
    winning_candidate_passed_proof: true,
    rejected_unproven_candidates: [],
  };

  const proofPlan = {
    schema_version: 'RemediationProofPlanV0',
    gates: ['reproduce failing test', 'apply bounded patch', 'rerun selected proof command'],
    requires_reproduction: true,
    requires_bounded_patch: true,
    requires_green_proof: true,
  };

  const report = {
    schema_version: 'RemediationReportV0',
    status: 'remediated',
    failure_reproduced: true,
    bounded_patch_applied: true,
    proof_gates_passed: true,
    claim: 'Remediated: failing behavior was reproduced, a bounded one-line patch was applied, and the proof gate passed.',
    artifact_paths: [
      '.stealtheye/remediator/artifacts/reproduction.log',
      '.stealtheye/remediator/artifacts/proof.log',
      '.stealtheye/remediator/remediation-report.json',
    ],
  };

  const commercial = {
    schema_version: 'RemediationCommercialV0',
    quote_range: 'public-proof demo only; no billing activated',
    risk_band: 'low',
    scope_assumptions: ['synthetic repo', 'single failing node test', 'bounded one-line patch'],
    billing_activated: false,
  };

  const diagnosisOnly = {
    schema_version: 'RemediationReportV0',
    status: 'diagnosis_only',
    failure_reproduced: false,
    bounded_patch_applied: false,
    proof_gates_passed: false,
    claim: 'Diagnosis only: remediation cannot be claimed because the failure was not reproduced.',
    artifact_paths: ['.stealtheye/remediator/diagnosis-only-report.json'],
  };

  const receipt = {
    schema_version: 'RemediatorExecutionReceiptV0',
    receipt_id: 's8-remediator-synthetic-proof',
    status: 'remediated',
    artifacts: report.artifact_paths,
    proof_summary: 'failure reproduced; bounded patch applied; selected proof command passed after patch',
  };

  writeFileSync(path.join(artifactsDir, 'reproduction.log'), firstRun.output);
  writeFileSync(path.join(artifactsDir, 'proof.log'), secondRun.output);
  writeJson(path.join(outDir, 'intake.json'), intake);
  writeJson(path.join(outDir, 'permissions.json'), permissions);
  writeJson(path.join(outDir, 'reality-map.json'), realityMap);
  writeJson(path.join(outDir, 'command-discovery.json'), commandDiscovery);
  writeJson(path.join(outDir, 'environment.json'), environment);
  writeJson(path.join(outDir, 'reproduction.json'), reproduction);
  writeJson(path.join(outDir, 'failure-taxonomy.json'), taxonomy);
  writeJson(path.join(outDir, 'localization.json'), localization);
  writeJson(path.join(outDir, 'repair-strategy.json'), repairStrategy);
  writeJson(path.join(outDir, 'bounded-patch.json'), boundedPatch);
  writeJson(path.join(outDir, 'patch-tournament.json'), patchTournament);
  writeJson(path.join(outDir, 'proof-plan.json'), proofPlan);
  writeJson(path.join(outDir, 'remediation-report.json'), report);
  writeJson(path.join(outDir, 'commercial.json'), commercial);
  writeJson(path.join(outDir, 'diagnosis-only-report.json'), diagnosisOnly);
  writeJson(path.join(outDir, 'execution-receipt.json'), receipt);

  if (!existsSync(path.join(outDir, 'remediation-report.json'))) {
    throw new Error('remediation report was not emitted');
  }

  console.log('S8 Remediator proof passed: reproduced failure, applied bounded patch, reran proof green, emitted report.');
}

main();
