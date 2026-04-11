#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
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

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    fail(`${options.step || command} failed to start: ${result.error.message}`);
  }
  if ((result.status ?? 1) !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
}

parseArgs(process.argv.slice(2));

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'generated',
  'server-openapi',
);
const mapPath = path.join(generatedRoot, 'dist', 'index.cjs.map');

function readMapSource() {
  if (!existsSync(mapPath)) {
    fail('TypeScript generated source map dist/index.cjs.map is missing.');
  }
  return readFileSync(mapPath, 'utf8');
}

run('node', [path.join(scriptDir, 'build-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript-generated-build:first',
});
const firstMapSource = readMapSource();

run('node', [path.join(scriptDir, 'build-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript-generated-build:second',
});
const secondMapSource = readMapSource();

if (/stable-typescript-build[\\/]+run-/.test(firstMapSource) || /stable-typescript-build[\\/]+run-/.test(secondMapSource)) {
  fail('TypeScript generated source maps must not embed run-specific temporary directory names.');
}

if (firstMapSource !== secondMapSource) {
  fail('TypeScript generated source maps drift across repeated builds with identical inputs.');
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated build determinism verification passed.');
