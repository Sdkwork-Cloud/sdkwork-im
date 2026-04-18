#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  collectWorkspaceFiles,
  finishFileExpectationVerification,
  readWorkspaceSource,
  readWorkspaceSources,
  workspacePathExists,
} from '../../workspace-file-expectation-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const typescriptRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript');
const composedSourceRoot = path.join(typescriptRoot, 'composed', 'src');

const failures = [];
if (!workspacePathExists({ workspaceRoot: composedSourceRoot, relativePath: 'index.ts' })) {
  failures.push('composed/src/index.ts must exist.');
}
if (!workspacePathExists({ workspaceRoot: composedSourceRoot, relativePath: 'sdk.ts' })) {
  failures.push('composed/src/sdk.ts must exist.');
}
if (!workspacePathExists({ workspaceRoot: composedSourceRoot, relativePath: 'generated-backend-types.ts' })) {
  failures.push('composed/src/generated-backend-types.ts must exist as the single generated type bridge.');
} else {
  const { bridgeSource } = readWorkspaceSources({
    workspaceRoot: composedSourceRoot,
    files: {
      bridgeSource: 'generated-backend-types.ts',
    },
  });

  if (!bridgeSource.includes("../../generated/server-openapi/src/types/index")) {
    failures.push(
      'composed/src/generated-backend-types.ts must bridge generated request and response types from src/types/index.',
    );
  }

  if (!bridgeSource.includes("../../generated/server-openapi/src/types/common")) {
    failures.push(
      'composed/src/generated-backend-types.ts must bridge generated common types from src/types/common.',
    );
  }

  if (!bridgeSource.includes("../../generated/server-openapi/src/types/string-map")) {
    failures.push(
      'composed/src/generated-backend-types.ts must bridge generated string-map types from src/types/string-map.',
    );
  }
}

if (workspacePathExists({ workspaceRoot: composedSourceRoot, relativePath: 'index.ts' })) {
  const indexSource = readWorkspaceSource({ workspaceRoot: composedSourceRoot, relativePath: 'index.ts' });
  if (!indexSource.includes('CrawChatSdkClient')) {
    failures.push('composed/src/index.ts must export CrawChatSdkClient.');
  }
}

if (workspacePathExists({ workspaceRoot: composedSourceRoot, relativePath: 'sdk.ts' })) {
  const sdkSource = readWorkspaceSource({ workspaceRoot: composedSourceRoot, relativePath: 'sdk.ts' });
  if (!sdkSource.includes('export class CrawChatSdkClient')) {
    failures.push('composed/src/sdk.ts must define CrawChatSdkClient.');
  }
}

for (const relativePath of collectWorkspaceFiles({
  workspaceRoot: composedSourceRoot,
  include: ({ relativePath: currentRelativePath, entry }) =>
    entry.isFile() && currentRelativePath.endsWith('.ts'),
})) {
  const source = readWorkspaceSource({ workspaceRoot: composedSourceRoot, relativePath });
  const matches = source.match(/generated\/server-openapi\/src\/[^\s'"`]+/g) || [];

  for (const matchedImportPath of matches) {
    if (relativePath === 'generated-backend-types.ts') {
      if (matchedImportPath.startsWith('generated/server-openapi/src/types/')) {
        continue;
      }
      failures.push(
        `generated-backend-types.ts must only bridge generated TypeScript type source paths, but found "${matchedImportPath}".`,
      );
      continue;
    }
    failures.push(
      `${relativePath} imports or exports generated TypeScript private source path "${matchedImportPath}".`,
    );
  }

  if (source.includes('@sdkwork/craw-chat-backend-sdk/src/')) {
    failures.push(`${relativePath} imports or exports @sdkwork/craw-chat-backend-sdk private source paths.`);
  }

  if (relativePath === 'types.ts' && !source.includes("from './generated-backend-types.js'")) {
    failures.push('types.ts must source generated backend types through ./generated-backend-types.js.');
  }
}

finishFileExpectationVerification({
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'TypeScript public API boundary verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] TypeScript public API boundary verification passed.',
});
