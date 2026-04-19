#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  assertNoArgs,
  runWorkspaceCommand,
} from '../../workspace-language-verify-shared.mjs';

const prefix = 'sdkwork-control-plane-sdk';
assertNoArgs(process.argv.slice(2), { prefix });

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const composedRoot = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-typescript',
  'composed',
);
const typeStubEntryPath = path.join(
  composedRoot,
  '.sdkwork',
  'type-stubs',
  'sdkwork-control-plane-types',
  'index.d.ts',
);
const runTscPath = path.join(composedRoot, 'bin', 'run-tsc.mjs');
const cleanDistPath = path.join(composedRoot, 'bin', 'clean-dist.mjs');
const tsconfigPath = path.join(composedRoot, 'tsconfig.build.json');
const smokeTestPath = path.join(composedRoot, 'test', 'control-plane-client.test.mjs');
const removedLegacySmokeTestPath = path.join(
  composedRoot,
  'test',
  'control-plane-sdk-client.test.mjs',
);
const removedLegacyShimPath = path.join(composedRoot, 'src', 'shims-sdk-common.d.ts');
const publicSdkSource = path.join(composedRoot, 'src', 'sdk.ts');
const publicSdkIndexSource = path.join(composedRoot, 'src', 'index.ts');
const tsconfigSource = readFileSync(tsconfigPath, 'utf8');

function assertWorkspaceInvariant(condition, message) {
  if (!condition) {
    console.error(`[${prefix}] ${message}`);
    process.exit(1);
  }
}

assertWorkspaceInvariant(existsSync(smokeTestPath), 'TypeScript smoke test file is missing.');
assertWorkspaceInvariant(
  !existsSync(removedLegacySmokeTestPath),
  'Removed legacy ImSdkAdminClient smoke test must not exist.',
);
assertWorkspaceInvariant(
  !existsSync(removedLegacyShimPath),
  'Removed legacy sdk-common shim must not exist.',
);
assertWorkspaceInvariant(
  tsconfigSource.includes('"rootDir": "src"'),
  'Composed TypeScript tsconfig.build.json must constrain rootDir to src.',
);
assertWorkspaceInvariant(
  tsconfigSource.includes('.sdkwork/type-stubs/sdkwork-control-plane-types/index.d.ts'),
  'Composed TypeScript tsconfig.build.json must resolve sdkwork-control-plane-types through local declaration stubs.',
);
assertWorkspaceInvariant(
  !tsconfigSource.includes('sdkwork-craw-chat-admin-types/src/index.ts'),
  'Composed TypeScript tsconfig.build.json must not import admin package source files directly.',
);

for (const sourcePath of [publicSdkSource, publicSdkIndexSource, smokeTestPath]) {
  const source = readFileSync(sourcePath, 'utf8');
  assertWorkspaceInvariant(
    !source.includes('ImSdkAdminClient'),
    `Removed legacy ImSdkAdminClient alias must not appear in ${path.basename(sourcePath)}.`,
  );
}

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'build-control-plane-types-stubs.mjs')],
  cwd: workspaceRoot,
  step: 'typescript:control-plane-types-stubs',
});
assertWorkspaceInvariant(
  existsSync(typeStubEntryPath),
  'Control-plane type stub build must produce composed/.sdkwork/type-stubs/sdkwork-control-plane-types/index.d.ts.',
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
