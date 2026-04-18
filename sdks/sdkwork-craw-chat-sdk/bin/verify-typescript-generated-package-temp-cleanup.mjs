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

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { packageVerifierPath, tempVerifyTargets } = resolveTypescriptGeneratedBuildVerifyPaths({
  workspaceRoot,
  scriptDir,
});

removePathsIfPresent(tempVerifyTargets);

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [packageVerifierPath],
  cwd: workspaceRoot,
  step: 'verify-typescript-generated-package.mjs',
});

const leakedTargets = collectExistingPaths(tempVerifyTargets);
if (leakedTargets.length > 0) {
  failTypescriptGeneratedBuildVerification({
    prefix,
    message: `TypeScript generated package verification must clean temporary verify directories after a successful run: ${leakedTargets.join(', ')}`,
  });
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated package temp cleanup verification passed.');
