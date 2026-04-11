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

function removeIfPresent(targetPath) {
  if (!existsSync(targetPath)) {
    return;
  }

  rmSync(targetPath, { recursive: true, force: true });
}

parseArgs(process.argv.slice(2));

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedTmpRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'generated',
  'server-openapi',
  '.sdkwork',
  'tmp',
);
const tempTargets = [
  path.join(generatedTmpRoot, 'tsc-dts-verify'),
  path.join(generatedTmpRoot, 'tsc-js-verify'),
];

for (const tempTarget of tempTargets) {
  removeIfPresent(tempTarget);
}

const result = spawnSync('node', [path.join(scriptDir, 'verify-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  stdio: 'inherit',
  shell: false,
});

if (result.error) {
  fail(`verify-typescript-generated-package.mjs failed to start: ${result.error.message}`);
}
if ((result.status ?? 1) !== 0) {
  fail(`verify-typescript-generated-package.mjs failed with exit code ${result.status}`);
}

const leakedTargets = tempTargets.filter((tempTarget) => existsSync(tempTarget));
if (leakedTargets.length > 0) {
  fail(`TypeScript generated package verification must clean temporary verify directories after a successful run: ${leakedTargets.join(', ')}`);
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated package temp cleanup verification passed.');
