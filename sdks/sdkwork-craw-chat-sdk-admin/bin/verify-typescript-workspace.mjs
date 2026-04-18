#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  assertNoArgs,
  runWorkspaceCommand,
} from '../../workspace-language-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk-admin';
assertNoArgs(process.argv.slice(2), { prefix });

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const composedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-admin-typescript',
  'composed',
);
const runTscPath = path.join(composedRoot, 'bin', 'run-tsc.mjs');
const cleanDistPath = path.join(composedRoot, 'bin', 'clean-dist.mjs');
const tsconfigPath = path.join(composedRoot, 'tsconfig.build.json');
const smokeTestPath = path.join(composedRoot, 'test', 'craw-chat-admin-client.test.mjs');

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
