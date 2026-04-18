#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  if (argv.length !== 2 || argv[0] !== '--package') {
    fail('Usage: verify-typescript-live-contract.mjs --package <root|composed>');
  }

  const packageTarget = (argv[1] || '').trim();
  if (!['root', 'composed'].includes(packageTarget)) {
    fail(`Unsupported package target: ${packageTarget}`);
  }

  return packageTarget;
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
  if (typeof result.status === 'number' && result.status !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${options.step || command} terminated with signal ${result.signal}`);
  }
}

const packageTarget = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatorRoot = path.resolve(
  workspaceRoot,
  '..',
  '..',
  '..',
  '..',
  'sdk',
  'sdkwork-sdk-generator',
);
const typeScriptCliPath = path.join(generatorRoot, 'node_modules', 'typescript', 'bin', 'tsc');
const packageRoot =
  packageTarget === 'root'
    ? path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript')
    : path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'composed');
const tsconfigPath = path.join(packageRoot, 'tsconfig.live-contract.json');

run(process.execPath, [typeScriptCliPath, '-p', tsconfigPath, '--noEmit'], {
  cwd: packageRoot,
  step: `typescript:${packageTarget}:live-contract`,
});

console.log(`[sdkwork-craw-chat-sdk] TypeScript ${packageTarget} live contract verification passed.`);
