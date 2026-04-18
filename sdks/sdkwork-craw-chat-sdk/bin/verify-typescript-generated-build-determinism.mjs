#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertNoArgs, runWorkspaceCommand } from '../../workspace-language-verify-shared.mjs';
import {
  failTypescriptGeneratedBuildVerification,
  readRequiredTypescriptBuildFile,
  resolveTypescriptGeneratedBuildVerifyPaths,
} from '../../workspace-typescript-generated-build-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';

assertNoArgs(process.argv.slice(2), { prefix });

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { buildScriptPath, mapPath } = resolveTypescriptGeneratedBuildVerifyPaths({
  workspaceRoot,
  scriptDir,
});

function readMapSource() {
  return readRequiredTypescriptBuildFile({
    prefix,
    filePath: mapPath,
    missingMessage: 'TypeScript generated source map dist/index.cjs.map is missing.',
  });
}

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [buildScriptPath],
  cwd: workspaceRoot,
  step: 'typescript-generated-build:first',
});
const firstMapSource = readMapSource();

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [buildScriptPath],
  cwd: workspaceRoot,
  step: 'typescript-generated-build:second',
});
const secondMapSource = readMapSource();

if (/stable-typescript-build[\\/]+run-/.test(firstMapSource) || /stable-typescript-build[\\/]+run-/.test(secondMapSource)) {
  failTypescriptGeneratedBuildVerification({
    prefix,
    message: 'TypeScript generated source maps must not embed run-specific temporary directory names.',
  });
}

if (firstMapSource !== secondMapSource) {
  failTypescriptGeneratedBuildVerification({
    prefix,
    message: 'TypeScript generated source maps drift across repeated builds with identical inputs.',
  });
}

console.log('[sdkwork-craw-chat-sdk] TypeScript generated build determinism verification passed.');
