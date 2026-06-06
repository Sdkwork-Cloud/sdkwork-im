#!/usr/bin/env node

import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

import { resolveSdkworkChatIamCommandEnv } from '../../apps/sdkwork-chat-pc/scripts/sdkwork-chat-iam-env.mjs';

const __filename = fileURLToPath(import.meta.url);
const repoRoot = path.resolve(path.dirname(__filename), '..', '..');

function pnpmCommand(platform = process.platform) {
  return platform === 'win32' ? 'pnpm.cmd' : 'pnpm';
}

const RELEASE_BUILD_STEPS = Object.freeze([
  {
    args: ['prepare:shared-sdk'],
    cwd: '.',
    label: 'prepare shared SDK git sources',
  },
  {
    args: ['install', '--frozen-lockfile'],
    cwd: 'apps/sdkwork-chat-pc',
    label: 'install sdkwork-chat-pc workspace',
  },
  {
    args: ['build'],
    cwd: 'apps/sdkwork-chat-pc',
    label: 'build sdkwork-chat-pc Vite app',
  },
]);

export function createSdkworkChatPcReleaseBuildPlan({
  env = process.env,
  repoRoot: resolvedRepoRoot = repoRoot,
} = {}) {
  const resolvedIamEnv = resolveSdkworkChatIamCommandEnv({
    env,
    target: 'web-build',
  });
  if (resolvedIamEnv.errors.length > 0) {
    throw new Error(resolvedIamEnv.errors.join('\n'));
  }

  return {
    command: pnpmCommand(),
    cwd: resolvedRepoRoot,
    env: {
      ...resolvedIamEnv.env,
      SDKWORK_SHARED_SDK_MODE: 'git',
    },
    shell: false,
    steps: RELEASE_BUILD_STEPS.map((step) => ({
      args: [...step.args],
      cwd: path.resolve(resolvedRepoRoot, step.cwd),
      label: step.label,
    })),
  };
}

function runPnpm(step, plan) {
  const result = plan.spawnSyncImpl(plan.command, step.args, {
    cwd: step.cwd,
    env: plan.env,
    shell: false,
    stdio: 'inherit',
  });
  if (result.error) {
    throw new Error(`${step.label} failed: ${result.error.message}`);
  }
  if (result.status !== 0) {
    throw new Error(`${step.label} failed with exit code ${result.status ?? 'unknown'}`);
  }
}

export function runSdkworkChatPcReleaseBuild({
  env = process.env,
  repoRoot: resolvedRepoRoot = repoRoot,
  spawnSyncImpl = spawnSync,
} = {}) {
  const plan = createSdkworkChatPcReleaseBuildPlan({
    env,
    repoRoot: resolvedRepoRoot,
  });

  for (const step of plan.steps) {
    runPnpm(step, {
      ...plan,
      spawnSyncImpl,
    });
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === __filename) {
  try {
    runSdkworkChatPcReleaseBuild();
  } catch (error) {
    console.error(error instanceof Error ? error.message : String(error));
    process.exit(1);
  }
}
