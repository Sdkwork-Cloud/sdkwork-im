#!/usr/bin/env node
import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { resolveSdkworkGeneratorRoot } from './sdk-paths.mjs';

function findOriginalRepoRoot(startDirectory) {
  let current = path.resolve(startDirectory);
  while (true) {
    const parent = path.dirname(current);
    if (path.basename(parent) === '.worktrees') {
      return path.dirname(parent);
    }
    if (existsSync(path.join(current, '.git'))) {
      return current;
    }
    if (parent === current) {
      throw new Error(`Unable to resolve repository root from ${startDirectory}`);
    }
    current = parent;
  }
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const currentWorkspaceRoot = path.resolve(scriptDir, '..');
const originalRepoRoot = findOriginalRepoRoot(currentWorkspaceRoot);
const expectedGeneratorRoot = path.resolve(
  originalRepoRoot,
  '..',
  '..',
  'sdk',
  'sdkwork-sdk-generator',
);

const resolvedFromCurrentWorkspace = resolveSdkworkGeneratorRoot(currentWorkspaceRoot);
assert.equal(
  resolvedFromCurrentWorkspace,
  expectedGeneratorRoot,
  'worktree workspace root should resolve the shared sdkwork-sdk-generator path',
);

const originalWorkspaceRoot = path.join(originalRepoRoot, 'sdks', 'sdkwork-craw-chat-sdk');
const resolvedFromOriginalWorkspace = resolveSdkworkGeneratorRoot(originalWorkspaceRoot);
assert.equal(
  resolvedFromOriginalWorkspace,
  expectedGeneratorRoot,
  'original repository workspace root should resolve the same sdkwork-sdk-generator path',
);

const previousGeneratorRoot = process.env.SDKWORK_GENERATOR_ROOT;
process.env.SDKWORK_GENERATOR_ROOT = expectedGeneratorRoot;
assert.equal(
  resolveSdkworkGeneratorRoot(path.join(currentWorkspaceRoot, '__bogus__')),
  expectedGeneratorRoot,
  'SDKWORK_GENERATOR_ROOT should override auto-discovery',
);
if (previousGeneratorRoot === undefined) {
  delete process.env.SDKWORK_GENERATOR_ROOT;
} else {
  process.env.SDKWORK_GENERATOR_ROOT = previousGeneratorRoot;
}

console.log('[sdkwork-craw-chat-sdk] sdk-paths test passed.');
