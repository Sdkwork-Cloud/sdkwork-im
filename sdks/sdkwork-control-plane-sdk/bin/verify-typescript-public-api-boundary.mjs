#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  collectWorkspaceFiles,
  finishFileExpectationVerification,
  readWorkspaceSource,
  readWorkspaceSources,
} from '../../workspace-file-expectation-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const composedRoot = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-typescript',
  'composed',
);

const { packageJsonSource, indexSource, sdkSource, generatedBridgeSource, sdkContextSource } =
  readWorkspaceSources({
    workspaceRoot: composedRoot,
    files: {
      packageJsonSource: 'package.json',
      indexSource: path.join('src', 'index.ts'),
      sdkSource: path.join('src', 'sdk.ts'),
      generatedBridgeSource: path.join('src', 'generated-backend-types.ts'),
      sdkContextSource: path.join('src', 'sdk-context.ts'),
    },
  });

const failures = [];

if (!packageJsonSource.includes('@sdkwork/control-plane-sdk')) {
  failures.push('Composed package name must stay on @sdkwork/control-plane-sdk.');
}
if (!indexSource.includes('ControlPlaneSdkClient')) {
  failures.push('Composed public index must export ControlPlaneSdkClient.');
}
if (!sdkSource.includes('export class ControlPlaneSdkClient')) {
  failures.push('Composed SDK entry must define ControlPlaneSdkClient.');
}
if (!generatedBridgeSource.includes('../../generated/server-openapi/dist/index.js')) {
  failures.push('generated-backend-types.ts must be the bridge into generated transport types.');
}

for (const relativePath of collectWorkspaceFiles({
  workspaceRoot: composedRoot,
  relativeRoot: 'src',
  include: ({ relativePath, entry }) =>
    entry.isFile() &&
    relativePath.endsWith('.ts') &&
    relativePath !== 'src/generated-backend-types.ts' &&
    relativePath !== 'src/sdk-context.ts',
})) {
  const source = readWorkspaceSource({ workspaceRoot: composedRoot, relativePath });
  if (source.includes('../../generated/server-openapi/')) {
    failures.push(
      `Only generated-backend-types.ts may import from generated transport directly (${path.basename(relativePath)}).`,
    );
  }
}

if (!sdkContextSource.includes('../../generated/server-openapi/dist/index.js')) {
  failures.push('sdk-context.ts must keep the workspace fallback to generated/server-openapi/dist/index.js.');
}

finishFileExpectationVerification({
  prefix: 'sdkwork-control-plane-sdk',
  failures,
  failureHeader: 'TypeScript public API boundary verification failed:',
  successMessage: '[sdkwork-control-plane-sdk] TypeScript public API boundary verification passed.',
});
