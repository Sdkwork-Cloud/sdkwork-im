#!/usr/bin/env node
import { existsSync, mkdirSync, realpathSync, rmSync, symlinkSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function run(step, args, cwd = packageRoot) {
  const result = spawnSync(process.execPath, args, {
    cwd,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    fail(`${step} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    fail(`${step} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${step} terminated with signal ${result.signal}`);
  }
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const generatorRoot = path.resolve(
  packageRoot,
  '..',
  '..',
  '..',
  '..',
  '..',
  '..',
  'sdk',
  'sdkwork-sdk-generator',
);
const typeScriptCliPath = path.join(generatorRoot, 'node_modules', 'typescript', 'bin', 'tsc');
const generatedPackageRoot = path.resolve(packageRoot, '..', 'generated', 'server-openapi');
const generatedPackageLinkPath = path.join(
  packageRoot,
  'node_modules',
  '@sdkwork',
  'craw-chat-backend-sdk',
);
const task = (process.argv[2] || '').trim();

function resolveExistingRealPath(targetPath) {
  try {
    if (!existsSync(targetPath)) {
      return null;
    }
    return realpathSync(targetPath);
  } catch {
    return null;
  }
}

function ensureLocalGeneratedBackendPackageLink() {
  if (!existsSync(path.join(generatedPackageRoot, 'package.json'))) {
    fail(`Generated TypeScript backend package is missing: ${generatedPackageRoot}`);
  }

  const expectedTarget = realpathSync(generatedPackageRoot);
  const currentTarget = resolveExistingRealPath(generatedPackageLinkPath);
  if (currentTarget === expectedTarget) {
    return;
  }

  mkdirSync(path.dirname(generatedPackageLinkPath), { recursive: true });
  if (existsSync(generatedPackageLinkPath)) {
    rmSync(generatedPackageLinkPath, { recursive: true, force: true });
  }
  symlinkSync(generatedPackageRoot, generatedPackageLinkPath, process.platform === 'win32' ? 'junction' : 'dir');
}

ensureLocalGeneratedBackendPackageLink();

switch (task) {
  case 'typecheck':
    run('typescript:composed-typecheck', [typeScriptCliPath, '-p', 'tsconfig.build.json', '--noEmit']);
    break;
  case 'build':
    run('typescript:composed-build', [typeScriptCliPath, '-p', 'tsconfig.build.json']);
    run('typescript:composed-clean', [path.join(scriptDir, 'clean-dist.mjs')]);
    break;
  case 'test':
    run('typescript:composed-test', [path.join(packageRoot, 'test', 'craw-chat-client.test.mjs')]);
    break;
  default:
    fail(`Unsupported package task "${task}". Expected one of: typecheck, build, test.`);
}
