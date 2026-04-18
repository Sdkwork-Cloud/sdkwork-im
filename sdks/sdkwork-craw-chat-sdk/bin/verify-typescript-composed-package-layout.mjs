#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const typescriptRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript');
const composedRoot = path.join(typescriptRoot, 'composed');

const forbiddenComposedSourceFiles = [
  'src/portal-module.ts',
  'src/device-module.ts',
  'src/inbox-module.ts',
  'src/presence-module.ts',
  'src/session-module.ts',
  'src/streams-module.ts',
];
const forbiddenComposedDistArtifacts = [
  'dist/portal-module.js',
  'dist/portal-module.d.ts',
  'dist/portal-module.d.ts.map',
  'dist/device-module.js',
  'dist/device-module.d.ts',
  'dist/device-module.d.ts.map',
  'dist/inbox-module.js',
  'dist/inbox-module.d.ts',
  'dist/inbox-module.d.ts.map',
  'dist/presence-module.js',
  'dist/presence-module.d.ts',
  'dist/presence-module.d.ts.map',
  'dist/session-module.js',
  'dist/session-module.d.ts',
  'dist/session-module.d.ts.map',
  'dist/streams-module.js',
  'dist/streams-module.d.ts',
  'dist/streams-module.d.ts.map',
];

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function expectedPackageTaskScript(task) {
  return `call "%npm_node_execpath%" ./bin/package-task.mjs ${task} || "$npm_node_execpath" ./bin/package-task.mjs ${task} || node ./bin/package-task.mjs ${task}`;
}

function read(relativePath) {
  const absolutePath = path.join(composedRoot, relativePath);
  if (!existsSync(absolutePath)) {
    fail(`Missing TypeScript composed-package file: ${relativePath}`);
  }
  return readFileSync(absolutePath, 'utf8');
}

const packageJson = JSON.parse(read('package.json'));
const expectedComposedScripts = {
  typecheck: expectedPackageTaskScript('typecheck'),
  build: expectedPackageTaskScript('build'),
  test: expectedPackageTaskScript('test'),
};
for (const [scriptName, expectedCommand] of Object.entries(expectedComposedScripts)) {
  if (packageJson.scripts?.[scriptName] !== expectedCommand) {
    fail(`TypeScript composed package script "${scriptName}" must execute the local package-task runner through npm-aware Node fallbacks.`);
  }
}

for (const relativePath of ['bin/package-task.mjs']) {
  if (!existsSync(path.join(composedRoot, relativePath))) {
    fail(`TypeScript composed package must include ${relativePath} for local task execution.`);
  }
}

for (const relativePath of forbiddenComposedSourceFiles) {
  if (existsSync(path.join(composedRoot, relativePath))) {
    fail(`TypeScript composed package source must not carry dead authoring module ${relativePath}.`);
  }
}

for (const relativePath of forbiddenComposedDistArtifacts) {
  if (existsSync(path.join(composedRoot, relativePath))) {
    fail(`TypeScript composed package dist must not carry dead authoring artifact ${relativePath}.`);
  }
}

console.log('[sdkwork-craw-chat-sdk] TypeScript composed package layout verification passed.');
