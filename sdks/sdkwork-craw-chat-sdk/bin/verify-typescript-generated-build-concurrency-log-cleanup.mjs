#!/usr/bin/env node
import { existsSync, rmSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  if (argv.length > 0) {
    fail(`Unknown argument: ${argv[0]}`);
  }
}

function removeLogRoot(logRoot) {
  if (!existsSync(logRoot)) {
    return;
  }

  rmSync(logRoot, { recursive: true, force: true });
}

parseArgs(process.argv.slice(2));

if (process.platform !== 'win32') {
  console.log('[sdkwork-craw-chat-sdk] TypeScript generated build concurrency log cleanup verification skipped on non-Windows hosts.');
  process.exit(0);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const logRoot = path.join(workspaceRoot, '.sdkwork', 'tmp', 'verify-typescript-generated-build-concurrency');

removeLogRoot(logRoot);

const result = spawnSync('node', [path.join(scriptDir, 'verify-typescript-generated-build-concurrency.mjs')], {
  cwd: workspaceRoot,
  stdio: 'inherit',
  shell: false,
});

if (result.error) {
  fail(`verify-typescript-generated-build-concurrency.mjs failed to start: ${result.error.message}`);
}
if ((result.status ?? 1) !== 0) {
  fail(`verify-typescript-generated-build-concurrency.mjs failed with exit code ${result.status}`);
}
if (existsSync(logRoot)) {
  fail('TypeScript generated build concurrency verification must clean .sdkwork/tmp/verify-typescript-generated-build-concurrency after a successful run.');
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated build concurrency log cleanup verification passed.');
