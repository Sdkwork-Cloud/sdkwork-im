#!/usr/bin/env node
import { existsSync, mkdirSync, rmSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

import { resolveGeneratorModulePath } from './generator-runtime.mjs';

const prefix = 'sdkwork-control-plane-sdk';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const composedRoot = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-typescript',
  'composed',
);
const controlPlaneTypesRoot = path.resolve(
  workspaceRoot,
  '..',
  '..',
  'apps',
  'craw-chat-admin',
  'packages',
  'sdkwork-control-plane-types',
);
const typesSourceRoot = path.join(controlPlaneTypesRoot, 'src');
const stubRoot = path.join(
  composedRoot,
  '.sdkwork',
  'type-stubs',
  'sdkwork-control-plane-types',
);
const tscPath = resolveGeneratorModulePath(workspaceRoot, 'typescript', 'bin', 'tsc');

function fail(message) {
  console.error(`[${prefix}] ${message}`);
  process.exit(1);
}

if (!existsSync(typesSourceRoot)) {
  fail(`Control-plane types source root is missing: ${typesSourceRoot}`);
}

rmSync(stubRoot, { force: true, recursive: true });
mkdirSync(stubRoot, { recursive: true });

const result = spawnSync(
  process.execPath,
  [
    tscPath,
    '--declaration',
    '--declarationMap',
    '--emitDeclarationOnly',
    '--module',
    'ES2022',
    '--target',
    'ES2022',
    '--moduleResolution',
    'Bundler',
    '--verbatimModuleSyntax',
    '--isolatedModules',
    '--strict',
    '--skipLibCheck',
    '--rootDir',
    typesSourceRoot,
    '--outDir',
    stubRoot,
    path.join(typesSourceRoot, 'index.ts'),
    path.join(typesSourceRoot, 'storage.ts'),
  ],
  {
    cwd: workspaceRoot,
    stdio: 'inherit',
  },
);

if (result.error) {
  throw result.error;
}

if ((result.status ?? 1) !== 0) {
  process.exit(result.status ?? 1);
}
