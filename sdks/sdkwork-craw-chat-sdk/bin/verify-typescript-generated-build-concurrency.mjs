#!/usr/bin/env node
import { existsSync } from 'node:fs';
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

parseArgs(process.argv.slice(2));

if (process.platform !== 'win32') {
  console.log('[sdkwork-craw-chat-sdk] TypeScript generated build concurrency verification skipped on non-Windows hosts.');
  process.exit(0);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const scriptPath = path.join(scriptDir, 'verify-typescript-generated-build-concurrency.ps1');
const logRoot = path.join(path.resolve(scriptDir, '..'), '.sdkwork', 'tmp', 'verify-typescript-generated-build-concurrency');

const result = spawnSync(
  'powershell',
  ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', scriptPath],
  {
    cwd: path.resolve(scriptDir, '..'),
    stdio: 'inherit',
    shell: false,
  },
);

if (result.error) {
  fail(`verify-typescript-generated-build-concurrency.ps1 failed to start: ${result.error.message}`);
}
if ((result.status ?? 1) !== 0) {
  fail(`verify-typescript-generated-build-concurrency.ps1 failed with exit code ${result.status}`);
}
if (existsSync(logRoot)) {
  fail('verify-typescript-generated-build-concurrency.ps1 must clean .sdkwork/tmp/verify-typescript-generated-build-concurrency after a successful run.');
}
