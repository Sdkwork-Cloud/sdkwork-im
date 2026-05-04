#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-im-admin-sdk] ${message}`);
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
const task = (process.argv[2] || '').trim();

switch (task) {
  case 'typecheck':
    run('typescript:composed-typecheck', [path.join(scriptDir, 'run-tsc.mjs'), '-p', 'tsconfig.build.json', '--noEmit']);
    break;
  case 'build':
    run('typescript:composed-build', [path.join(scriptDir, 'run-tsc.mjs'), '-p', 'tsconfig.build.json']);
    run('typescript:composed-clean', [path.join(scriptDir, 'clean-dist.mjs')]);
    break;
  case 'test':
    run('typescript:composed-test', [path.join(packageRoot, 'test', 'im-admin-sdk-client.test.mjs')]);
    break;
  default:
    fail(`Unsupported package task "${task}". Expected one of: typecheck, build, test.`);
}
