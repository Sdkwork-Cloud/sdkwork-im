#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertNoArgs, runWorkspaceCommand } from '../../workspace-language-verify-shared.mjs';
import {
  collectExistingPaths,
  failTypescriptGeneratedBuildVerification,
  removePathsIfPresent,
  resolveTypescriptGeneratedBuildVerifyPaths,
} from '../../workspace-typescript-generated-build-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';

assertNoArgs(process.argv.slice(2), { prefix });

if (process.platform !== 'win32') {
  console.log('[sdkwork-craw-chat-sdk] TypeScript generated build concurrency log cleanup verification skipped on non-Windows hosts.');
  process.exit(0);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { concurrencyLogRoot, concurrencyVerifierPath } = resolveTypescriptGeneratedBuildVerifyPaths({
  workspaceRoot,
  scriptDir,
});

removePathsIfPresent([concurrencyLogRoot]);

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [concurrencyVerifierPath],
  cwd: workspaceRoot,
  step: 'verify-typescript-generated-build-concurrency.mjs',
});

if (collectExistingPaths([concurrencyLogRoot]).length > 0) {
  failTypescriptGeneratedBuildVerification({
    prefix,
    message: 'TypeScript generated build concurrency verification must clean .sdkwork/tmp/verify-typescript-generated-build-concurrency after a successful run.',
  });
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated build concurrency log cleanup verification passed.');
