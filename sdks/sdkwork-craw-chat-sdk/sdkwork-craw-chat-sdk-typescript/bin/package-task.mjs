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
const generatorRoot = path.resolve(
  packageRoot,
  '..',
  '..',
  '..',
  '..',
  '..',
  'sdk',
  'sdkwork-sdk-generator',
);
const typeScriptCliPath = path.join(generatorRoot, 'node_modules', 'typescript', 'bin', 'tsc');
const task = (process.argv[2] || '').trim();

switch (task) {
  case 'assemble':
    run('typescript:single-package-assemble', [path.join(scriptDir, 'assemble-single-package.mjs')]);
    break;
  case 'clean':
    run('typescript:single-package-clean', [path.join(scriptDir, 'clean-dist.mjs')]);
    break;
  case 'typecheck':
    run('typescript:single-package-assemble', [path.join(scriptDir, 'assemble-single-package.mjs')]);
    run('typescript:single-package-typecheck', [typeScriptCliPath, '-p', 'tsconfig.build.json', '--noEmit']);
    break;
  case 'build':
    run('typescript:single-package-assemble', [path.join(scriptDir, 'assemble-single-package.mjs')]);
    run('typescript:single-package-clean', [path.join(scriptDir, 'clean-dist.mjs')]);
    run('typescript:single-package-build', [typeScriptCliPath, '-p', 'tsconfig.build.json']);
    break;
  case 'smoke':
    run('typescript:single-package-smoke', [path.join(packageRoot, 'test', 'craw-chat-client.test.mjs')]);
    break;
  case 'test':
    run('typescript:single-package-assemble', [path.join(scriptDir, 'assemble-single-package.mjs')]);
    run('typescript:single-package-clean', [path.join(scriptDir, 'clean-dist.mjs')]);
    run('typescript:single-package-build', [typeScriptCliPath, '-p', 'tsconfig.build.json']);
    run('typescript:single-package-smoke', [path.join(packageRoot, 'test', 'craw-chat-client.test.mjs')]);
    break;
  default:
    fail(
      `Unsupported package task "${task}". Expected one of: assemble, clean, typecheck, build, smoke, test.`,
    );
}
