import assert from 'node:assert/strict';
import { mkdtemp, mkdir, readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import test from 'node:test';

import {
  COMMAND_FAILURE_EXIT_CODE,
  assessCapacityEvidenceIndex,
  buildCommercialReadinessChecks,
  resolvePnpmExecutable,
  runCommercialReadiness,
  shouldUseShellForCommand,
} from './commercial-readiness.mjs';

const repoRoot = path.resolve(import.meta.dirname, '..', '..');

test('commercial readiness checks cover the verified frontend and backend gate commands', () => {
  const checks = buildCommercialReadinessChecks({
    repoRoot,
    platform: 'win32',
  });

  assert.deepEqual(
    checks.map((check) => check.id),
    [
      'admin-install',
      'admin-test',
      'admin-typecheck',
      'admin-build',
      'portal-test',
      'portal-build',
      'control-plane-api-tests',
      'commercial-gate-contract',
      'session-gateway-tests',
      'performance-quant-baseline',
      'performance-drill-catalog',
    ],
  );

  assert.equal(resolvePnpmExecutable('win32'), 'pnpm.cmd');
  assert.equal(checks[0].command, 'pnpm.cmd');
  assert.equal(checks[0].env?.npm_config_update_notifier, 'false');
  assert.equal(
    checks.find((check) => check.id === 'admin-test')?.cwd,
    path.join(repoRoot, 'apps', 'craw-chat-admin'),
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'admin-test')?.args,
    ['test'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'portal-build')?.args,
    ['build'],
  );
  assert.deepEqual(
    checks.find((check) => check.id === 'control-plane-api-tests')?.args,
    ['test', '-p', 'control-plane-api', '--tests'],
  );
  assert.equal(
    checks.find((check) => check.id === 'portal-build')?.env?.npm_config_update_notifier,
    'false',
  );
  assert.equal(
    checks.find((check) => check.id === 'control-plane-api-tests')?.env,
    undefined,
  );
  assert.equal(shouldUseShellForCommand('pnpm.cmd', 'win32'), true);
  assert.equal(shouldUseShellForCommand('cargo', 'win32'), false);
});

test('capacity evidence assessment blocks template-only commercial readiness claims', () => {
  const assessment = assessCapacityEvidenceIndex({
    tier: 'Capacity Tier',
    state: 'template_only_pending_execution',
    collectionSummary: {
      pendingSlots: 7,
    },
    evidenceSlots: [
      { id: 'connection_capacity', status: 'pending_collection' },
      { id: 'message_capacity', status: 'pending_collection' },
    ],
  });

  assert.equal(assessment.ok, false);
  assert.match(assessment.summary, /Capacity Tier/);
  assert.match(assessment.summary, /template_only_pending_execution/);
  assert.match(assessment.summary, /7 pending slots/);
  assert.match(assessment.blockers.join('\n'), /connection_capacity/);
  assert.match(assessment.blockers.join('\n'), /message_capacity/);
});

test('capacity evidence assessment accepts fully collected capacity evidence', () => {
  const assessment = assessCapacityEvidenceIndex({
    tier: 'Capacity Tier',
    state: 'evidence_collected_gate_passed',
    collectionSummary: {
      pendingSlots: 0,
    },
    evidenceSlots: [
      { id: 'connection_capacity', status: 'collected' },
      { id: 'message_capacity', status: 'collected' },
    ],
  });

  assert.equal(assessment.ok, true);
  assert.match(assessment.summary, /fully collected/i);
  assert.equal(assessment.blockers.length, 0);
});

test('release README documents the commercial readiness command and honest capacity blocker', async () => {
  const releaseReadmePath = path.join(repoRoot, 'docs', 'release', 'README.md');
  const releaseReadme = await readFile(releaseReadmePath, 'utf8');

  assert.match(releaseReadme, /node scripts\/release\/commercial-readiness\.mjs/);
  assert.match(releaseReadme, /capacity-tier-evidence-index\.json/);
  assert.match(releaseReadme, /exit code `?1`?/i);
  assert.match(releaseReadme, /exit code `?2`?/i);
});

test('deployment validation index links the unified commercial readiness gate', async () => {
  const operatorIndexPath = path.join(
    repoRoot,
    'docs',
    '部署',
    '兼容矩阵与SDK-CLI-operator验证索引.md',
  );
  const operatorIndex = await readFile(operatorIndexPath, 'utf8');

  assert.match(operatorIndex, /node scripts\/release\/commercial-readiness\.mjs/);
  assert.match(operatorIndex, /docs\/release\/README\.md/);
  assert.match(operatorIndex, /exit code `?1`?/i);
});

test('commercial readiness converts thrown command execution errors into a controlled command failure result', async () => {
  const logs = createLoggerCapture();

  const result = await runCommercialReadiness({
    repoRoot,
    logger: logs.logger,
    runCheck: async (check) => {
      if (check.id === 'admin-install') {
        throw new Error('spawn pnpm ENOENT');
      }

      return {
        ...check,
        ok: true,
        exitCode: 0,
      };
    },
  });

  assert.equal(result.ok, false);
  assert.equal(result.exitCode, COMMAND_FAILURE_EXIT_CODE);
  assert.equal(result.capacityAssessment, null);
  assert.equal(result.checks.length, 0);
  assert.deepEqual(result.failure, {
    stage: 'admin-install',
    summary: 'spawn pnpm ENOENT',
  });
  assert.match(logs.stderr.join('\n'), /admin-install/);
  assert.match(logs.stderr.join('\n'), /spawn pnpm ENOENT/);
});

test('commercial readiness converts malformed capacity evidence into a controlled command failure result', async () => {
  const tempRepoRoot = await mkdtemp(path.join(os.tmpdir(), 'commercial-readiness-'));
  const evidenceDir = path.join(tempRepoRoot, 'artifacts', 'perf', 'step-11', 'capacity');
  await mkdir(evidenceDir, { recursive: true });
  await writeFile(
    path.join(evidenceDir, 'capacity-tier-evidence-index.json'),
    '{"tier":"Capacity Tier",',
    'utf8',
  );
  const logs = createLoggerCapture();

  const result = await runCommercialReadiness({
    repoRoot: tempRepoRoot,
    logger: logs.logger,
    runCheck: async (check) => ({
      ...check,
      ok: true,
      exitCode: 0,
    }),
  });

  assert.equal(result.ok, false);
  assert.equal(result.exitCode, COMMAND_FAILURE_EXIT_CODE);
  assert.equal(result.capacityAssessment, null);
  assert.equal(result.checks.length, 11);
  assert.equal(result.failure.stage, 'capacity-evidence-load');
  assert.match(result.failure.summary, /JSON/i);
  assert.match(result.failure.evidenceIndexPath, /capacity-tier-evidence-index\.json$/);
  assert.match(logs.stderr.join('\n'), /capacity evidence index/);
  assert.match(logs.stderr.join('\n'), /JSON/i);
});

function createLoggerCapture() {
  const stdout = [];
  const stderr = [];

  return {
    stdout,
    stderr,
    logger: {
      log(message) {
        stdout.push(String(message));
      },
      error(message) {
        stderr.push(String(message));
      },
    },
  };
}
