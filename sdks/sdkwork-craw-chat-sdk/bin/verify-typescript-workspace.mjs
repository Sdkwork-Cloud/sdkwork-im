#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  assertNoArgs,
  runWorkspaceCommand,
} from '../../workspace-language-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';
assertNoArgs(process.argv.slice(2), { prefix });

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const tsconfigPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'composed',
  'tsconfig.build.json',
);
const runTscPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'composed',
  'bin',
  'run-tsc.mjs',
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

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'build-typescript-generated-package.mjs')],
  cwd: workspaceRoot,
  step: 'typescript:generated-build',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-typescript-generated-package.mjs')],
  cwd: workspaceRoot,
  step: 'typescript:generated-package',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-typescript-generated-package-temp-cleanup.mjs')],
  cwd: workspaceRoot,
  step: 'typescript:generated-package-temp-cleanup',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-auth-surface-alignment.mjs'), '--language', 'typescript'],
  cwd: workspaceRoot,
  step: 'typescript:auth-surface',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-typescript-usage-surface.mjs')],
  cwd: workspaceRoot,
  step: 'typescript:usage-surface',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-typescript-public-api-boundary.mjs')],
  cwd: workspaceRoot,
  step: 'typescript:public-api-boundary',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [runTscPath, '-p', tsconfigPath, '--noEmit'],
  cwd: workspaceRoot,
  step: 'typescript:typecheck',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [runTscPath, '-p', tsconfigPath],
  cwd: workspaceRoot,
  step: 'typescript:build',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [cleanDistPath],
  cwd: workspaceRoot,
  step: 'typescript:clean-dist',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [smokeTestPath],
  cwd: workspaceRoot,
  step: 'typescript:smoke-test',
});
