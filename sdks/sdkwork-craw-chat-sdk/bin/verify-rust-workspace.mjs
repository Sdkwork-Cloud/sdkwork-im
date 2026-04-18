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
    env: options.env,
    stdio: 'inherit',
    shell: false,
    timeout: options.timeoutMs,
  });

  if (result.error) {
    fail(`${options.step || command} failed to start: ${result.error.message}`);
  }

  if (
    options.allowOfflineFallback &&
    !options.env?.CARGO_NET_OFFLINE &&
    typeof result.status === 'number' &&
    result.status !== 0
  ) {
    console.warn(
      `[sdkwork-craw-chat-sdk] ${options.step || command} failed; retrying once with CARGO_NET_OFFLINE=true.`,
    );
    return run(command, args, {
      ...options,
      allowOfflineFallback: false,
      env: {
        ...options.env,
        CARGO_NET_OFFLINE: 'true',
      },
    });
  }

  if (typeof result.status === 'number' && result.status !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${options.step || command} terminated with signal ${result.signal}`);
  }

  return result;
}

parseArgs(process.argv.slice(2));

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedDir = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-rust',
  'generated',
  'server-openapi',
);
const composedCargoToml = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-rust',
  'composed',
  'Cargo.toml',
);
const rustEnv = {
  ...process.env,
  CARGO_TARGET_DIR: path.join(workspaceRoot, '.sdkwork', 'rust-target'),
};

run('node', [path.join(scriptDir, 'verify-auth-surface-alignment.mjs'), '--language', 'rust'], {
  cwd: workspaceRoot,
  step: 'rust:auth-surface',
});
run('node', [path.join(scriptDir, 'verify-rust-package-metadata.mjs')], {
  cwd: workspaceRoot,
  step: 'rust:package-metadata',
});
run('node', [path.join(scriptDir, 'verify-rust-public-api-boundary.mjs')], {
  cwd: workspaceRoot,
  step: 'rust:public-api-boundary',
});
run(
  'node',
  ['./bin/publish-core.mjs', '--language', 'rust', '--project-dir', '.', '--action', 'check'],
  {
    cwd: generatedDir,
    env: rustEnv,
    step: 'rust:generated-check',
    allowOfflineFallback: true,
    timeoutMs: 600000,
  },
);
run(
  'node',
  ['./bin/publish-core.mjs', '--language', 'rust', '--project-dir', '.', '--action', 'build'],
  {
    cwd: generatedDir,
    env: rustEnv,
    step: 'rust:generated-build',
    allowOfflineFallback: true,
    timeoutMs: 600000,
  },
);
run('cargo', ['test', '--manifest-path', composedCargoToml], {
  cwd: workspaceRoot,
  env: rustEnv,
  step: 'rust:composed-test',
  allowOfflineFallback: true,
  timeoutMs: 600000,
});
