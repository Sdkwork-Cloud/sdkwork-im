#!/usr/bin/env node
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
const workspaceRoot = path.resolve(packageRoot, '..', '..', '..');
const generatedBuilderPath = path.join(workspaceRoot, 'bin', 'build-typescript-generated-package.mjs');
const task = (process.argv[2] || '').trim();

switch (task) {
  case 'build':
  case 'dev':
    run('typescript-generated:workspace-build', [generatedBuilderPath]);
    break;
  default:
    fail(`Unsupported package task "${task}". Expected one of: build, dev.`);
}
