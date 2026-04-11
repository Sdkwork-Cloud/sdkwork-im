#!/usr/bin/env node
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
    timeout: options.timeoutMs,
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

parseArgs(process.argv.slice(2));

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatorRoot = path.resolve(workspaceRoot, '..', '..', '..', '..', 'sdk', 'sdkwork-sdk-generator');
const tscPath = path.join(generatorRoot, 'node_modules', 'typescript', 'bin', 'tsc');
const tsconfigPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'composed',
  'tsconfig.build.json',
);
const cleanDistPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'composed',
  'bin',
  'clean-dist.mjs',
);
const smokeTestPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'composed',
  'test',
  'craw-chat-client.test.mjs',
);

run('node', [path.join(scriptDir, 'build-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-build',
});
run('node', [path.join(scriptDir, 'verify-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-package',
});
run('node', [path.join(scriptDir, 'verify-typescript-generated-package-temp-cleanup.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-package-temp-cleanup',
});
run('node', [path.join(scriptDir, 'verify-auth-surface-alignment.mjs'), '--language', 'typescript'], {
  cwd: workspaceRoot,
  step: 'typescript:auth-surface',
});
run('node', [path.join(scriptDir, 'verify-typescript-public-api-boundary.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:public-api-boundary',
});
run('node', [tscPath, '-p', tsconfigPath, '--noEmit'], {
  cwd: workspaceRoot,
  step: 'typescript:typecheck',
});
run('node', [tscPath, '-p', tsconfigPath], {
  cwd: workspaceRoot,
  step: 'typescript:build',
});
run('node', [cleanDistPath], {
  cwd: workspaceRoot,
  step: 'typescript:clean-dist',
});
run('node', [smokeTestPath], {
  cwd: workspaceRoot,
  step: 'typescript:smoke-test',
});
