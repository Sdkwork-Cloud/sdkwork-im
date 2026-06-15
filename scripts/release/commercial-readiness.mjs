import { spawn } from 'node:child_process';
import { existsSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

export const COMMAND_FAILURE_EXIT_CODE = 1;
export const READINESS_BLOCKED_EXIT_CODE = 2;

export function resolvePnpmExecutable(platform = process.platform) {
  return platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

export function shouldUseShellForCommand(command, platform = process.platform) {
  return platform === 'win32' && /\.cmd$/i.test(String(command ?? '').trim());
}

export function buildCommercialReadinessChecks({
  repoRoot = resolveRepoRoot(),
  platform = process.platform,
} = {}) {
  const pnpmExecutable = resolvePnpmExecutable(platform);
  const nodeExecutable = process.execPath;
  const pnpmRuntimeEnv = {
    npm_config_update_notifier: 'false',
  };

  return [
    {
      id: 'pc-install',
      label: 'Sdkwork IM PC frozen install',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['install', '--frozen-lockfile', '--ignore-scripts'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-lint',
      label: 'Sdkwork IM PC typecheck',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'lint'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-build',
      label: 'Sdkwork IM PC production build',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'build'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-auth-appbase-ui-contract',
      label: 'Sdkwork IM PC appbase auth UI contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: nodeExecutable,
      args: ['scripts/auth-appbase-ui-contract.test.mjs'],
    },
    {
      id: 'pc-notary-app-sdk-integration',
      label: 'Sdkwork IM PC notary app SDK integration contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:notary-app-sdk-integration'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'pc-qr-scan-standard',
      label: 'Sdkwork IM PC QR scan standard contract',
      cwd: path.join(repoRoot, 'apps', 'sdkwork-im-pc'),
      command: pnpmExecutable,
      args: ['run', 'test:qr-scan-standard'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'dependency-management',
      label: 'SDKWork dependency management standard',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'check:dependency-management'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'workflow-commercial-gates',
      label: 'Workflow commercial governance gates',
      cwd: repoRoot,
      command: pnpmExecutable,
      args: ['run', 'test:workflow-commercial-gates'],
      env: pnpmRuntimeEnv,
    },
    {
      id: 'control-plane-api-tests',
      label: 'Control-plane API tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'control-plane-api', '--tests'],
    },
    {
      id: 'commercial-gate-contract',
      label: 'Commercial contract gate',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'local-minimal-node', '--test', 'commercial_gate_contract_test'],
    },
    {
      id: 'session-gateway-tests',
      label: 'Session gateway tests',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'session-gateway', '--tests'],
    },
    {
      id: 'performance-quant-baseline',
      label: 'Local performance quant baseline',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'local-minimal-node', '--test', 'performance_quant_baseline_test'],
    },
    {
      id: 'performance-drill-catalog',
      label: 'Performance drill catalog',
      cwd: repoRoot,
      command: 'cargo',
      args: ['test', '-p', 'local-minimal-node', '--test', 'performance_drill_catalog_test'],
    },
  ];
}

export function assessCapacityEvidenceIndex(indexJson) {
  const tier = typeof indexJson?.tier === 'string' ? indexJson.tier : 'Capacity Tier';
  const state = typeof indexJson?.state === 'string' ? indexJson.state : 'unknown';
  const pendingSlots = normalizePendingSlots(indexJson?.collectionSummary?.pendingSlots);
  const pendingEvidenceIds = Array.isArray(indexJson?.evidenceSlots)
    ? indexJson.evidenceSlots
      .filter((slot) => slot?.status === 'pending_collection')
      .map((slot) => slot?.id)
      .filter((slotId) => typeof slotId === 'string' && slotId.length > 0)
    : [];

  if (state === 'evidence_collected_gate_passed' && pendingSlots === 0) {
    return {
      ok: true,
      summary: `${tier} evidence is fully collected and ready for commercial sign-off.`,
      blockers: [],
    };
  }

  return {
    ok: false,
    summary: `${tier} remains ${state} with ${pendingSlots} pending slots.`,
    blockers: pendingEvidenceIds.map((slotId) => `${slotId} is still pending collection.`),
  };
}

export function resolveCapacityEvidenceIndexPath(repoRoot = resolveRepoRoot()) {
  return path.join(
    repoRoot,
    'artifacts',
    'perf',
    'step-11',
    'capacity',
    'capacity-tier-evidence-index.json',
  );
}

export async function loadCapacityEvidenceIndex(repoRoot = resolveRepoRoot()) {
  const evidenceIndexPath = resolveCapacityEvidenceIndexPath(repoRoot);
  const source = await readFile(evidenceIndexPath, 'utf8');

  return {
    evidenceIndexPath,
    indexJson: JSON.parse(source),
  };
}

export async function runCommercialReadiness({
  repoRoot = resolveRepoRoot(),
  platform = process.platform,
  logger = console,
  runCheck = executeCheck,
} = {}) {
  const checks = buildCommercialReadinessChecks({ repoRoot, platform });
  const results = [];

  for (const check of checks) {
    logger.log(`[commercial-readiness] running ${check.id}: ${formatCommand(check)}`);
    let result;
    try {
      result = await runCheck(check);
    } catch (error) {
      const summary = formatErrorSummary(error);
      logger.error(`[commercial-readiness] failed ${check.id} due to execution error: ${summary}`);
      return {
        ok: false,
        exitCode: COMMAND_FAILURE_EXIT_CODE,
        checks: results,
        capacityAssessment: null,
        failure: {
          stage: check.id,
          summary,
        },
      };
    }

    results.push(result);
    if (!result.ok) {
      logger.error(`[commercial-readiness] failed ${check.id} with exit code ${result.exitCode}.`);
      return {
        ok: false,
        exitCode: COMMAND_FAILURE_EXIT_CODE,
        checks: results,
        capacityAssessment: null,
        failure: {
          stage: check.id,
          summary: `exit code ${result.exitCode}`,
        },
      };
    }

    logger.log(`[commercial-readiness] passed ${check.id}`);
  }

  let evidenceIndex;
  try {
    evidenceIndex = await loadCapacityEvidenceIndex(repoRoot);
  } catch (error) {
    const evidenceIndexPath = resolveCapacityEvidenceIndexPath(repoRoot);
    const summary = formatErrorSummary(error);
    logger.error(
      `[commercial-readiness] failed to load capacity evidence index ${evidenceIndexPath}: ${summary}`,
    );
    return {
      ok: false,
      exitCode: COMMAND_FAILURE_EXIT_CODE,
      checks: results,
      capacityAssessment: null,
      failure: {
        stage: 'capacity-evidence-load',
        summary,
        evidenceIndexPath,
      },
    };
  }

  const { evidenceIndexPath, indexJson } = evidenceIndex;
  const capacityAssessment = assessCapacityEvidenceIndex(indexJson);

  if (!capacityAssessment.ok) {
    logger.error(`[commercial-readiness] blocked by capacity evidence: ${capacityAssessment.summary}`);
    for (const blocker of capacityAssessment.blockers) {
      logger.error(`[commercial-readiness] ${blocker}`);
    }

    return {
      ok: false,
      exitCode: READINESS_BLOCKED_EXIT_CODE,
      checks: results,
      capacityAssessment: {
        ...capacityAssessment,
        evidenceIndexPath,
      },
    };
  }

  logger.log(`[commercial-readiness] ${capacityAssessment.summary}`);
  return {
    ok: true,
    exitCode: 0,
    checks: results,
    capacityAssessment: {
      ...capacityAssessment,
      evidenceIndexPath,
    },
  };
}

async function executeCheck(check) {
  if (!existsSync(check.cwd)) {
    throw new Error(`configured cwd does not exist: ${check.cwd}`);
  }

  const exitCode = await spawnCommand(check.command, check.args, {
    cwd: check.cwd,
    env: check.env,
    stdio: 'inherit',
  });

  return {
    ...check,
    ok: exitCode === 0,
    exitCode,
  };
}

function spawnCommand(command, args, options) {
  return new Promise((resolve, reject) => {
    const child = spawn(command, args, {
      ...options,
      env: options?.env ? { ...process.env, ...options.env } : process.env,
      shell: shouldUseShellForCommand(command, process.platform),
    });

    child.once('error', reject);
    child.once('exit', (code, signal) => {
      if (signal) {
        resolve(1);
        return;
      }

      resolve(code ?? 1);
    });
  });
}

function formatCommand(check) {
  return [check.command, ...check.args].join(' ');
}

function normalizePendingSlots(value) {
  const parsed = Number(value);
  return Number.isFinite(parsed) && parsed >= 0 ? parsed : 0;
}

function formatErrorSummary(error) {
  if (error instanceof Error && typeof error.message === 'string' && error.message.length > 0) {
    return error.message;
  }

  return String(error);
}

function resolveRepoRoot() {
  return path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..', '..');
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const result = await runCommercialReadiness();
  process.exitCode = result.exitCode;
}
