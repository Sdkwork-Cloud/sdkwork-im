#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function expectedPackageTaskScript(task) {
  return `call "%npm_node_execpath%" ./bin/package-task.mjs ${task} || "$npm_node_execpath" ./bin/package-task.mjs ${task} || node ./bin/package-task.mjs ${task}`;
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

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'generated',
  'server-openapi',
);
const generatedBuilderPath = path.join(scriptDir, 'build-typescript-generated-package.mjs');
const manifestPath = path.join(generatedRoot, 'package.json');
const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
const scripts = manifest.scripts ?? {};
const expectedBuildScript = expectedPackageTaskScript('build');
const expectedDevScript = expectedPackageTaskScript('dev');

for (const forbiddenLifecycleScript of ['prepublishOnly', 'prepare', 'prepack', 'postinstall']) {
  if (Object.prototype.hasOwnProperty.call(scripts, forbiddenLifecycleScript)) {
    fail(
      `TypeScript generated package must not define install-time lifecycle script "${forbiddenLifecycleScript}" in ${manifestPath}.`,
    );
  }
}

if (scripts.build !== expectedBuildScript) {
  fail(
    `TypeScript generated package build script must be "${expectedBuildScript}", received "${scripts.build ?? ''}".`,
  );
}
if (scripts.dev !== expectedDevScript) {
  fail(
    `TypeScript generated package dev script must be "${expectedDevScript}", received "${scripts.dev ?? ''}".`,
  );
}

for (const relativePath of ['bin/package-task.mjs']) {
  if (!existsSync(path.join(generatedRoot, relativePath))) {
    fail(`TypeScript generated package must include ${relativePath} for local task execution.`);
  }
}

run(process.execPath, [generatedBuilderPath], {
  cwd: workspaceRoot,
  step: 'typescript-generated-package:direct-build-entrypoint',
});

console.log('[sdkwork-craw-chat-sdk] TypeScript generated package install safety verification passed.');
