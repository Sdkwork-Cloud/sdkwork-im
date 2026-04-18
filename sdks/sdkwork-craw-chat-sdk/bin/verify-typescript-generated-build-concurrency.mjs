#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { assertNoArgs, runWorkspaceCommand } from '../../workspace-language-verify-shared.mjs';
import {
  collectExistingPaths,
  failTypescriptGeneratedBuildVerification,
  resolveTypescriptGeneratedBuildVerifyPaths,
} from '../../workspace-typescript-generated-build-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';

assertNoArgs(process.argv.slice(2), { prefix });

if (process.platform !== 'win32') {
  console.log('[sdkwork-craw-chat-sdk] TypeScript generated build concurrency verification skipped on non-Windows hosts.');
  process.exit(0);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { concurrencyLogRoot, concurrencyPowerShellPath } = resolveTypescriptGeneratedBuildVerifyPaths({
  workspaceRoot,
  scriptDir,
});

runWorkspaceCommand({
  prefix,
  command: 'powershell',
  args: ['-NoProfile', '-ExecutionPolicy', 'Bypass', '-File', concurrencyPowerShellPath],
  cwd: workspaceRoot,
  step: 'verify-typescript-generated-build-concurrency.ps1',
});

if (collectExistingPaths([concurrencyLogRoot]).length > 0) {
  failTypescriptGeneratedBuildVerification({
    prefix,
    message: 'verify-typescript-generated-build-concurrency.ps1 must clean .sdkwork/tmp/verify-typescript-generated-build-concurrency after a successful run.',
  });
}
