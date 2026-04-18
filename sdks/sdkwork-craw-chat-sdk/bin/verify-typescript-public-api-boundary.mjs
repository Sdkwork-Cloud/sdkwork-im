#!/usr/bin/env node
import { existsSync, readdirSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const typescriptRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript');
const composedSourceRoot = path.join(typescriptRoot, 'composed', 'src');
const generatedTypesBridgePath = path.join(composedSourceRoot, 'generated-backend-types.ts');

function collectFiles(rootDirectory) {
  const files = [];
  const queue = [rootDirectory];

  while (queue.length > 0) {
    const currentDirectory = queue.shift();
    for (const entry of readdirSync(currentDirectory, { withFileTypes: true })) {
      const absolutePath = path.join(currentDirectory, entry.name);
      if (entry.isDirectory()) {
        queue.push(absolutePath);
        continue;
      }
      if (entry.isFile() && absolutePath.endsWith('.ts')) {
        files.push(absolutePath);
      }
    }
  }

  return files;
}

const failures = [];
if (!existsSync(generatedTypesBridgePath)) {
  failures.push('composed/src/generated-backend-types.ts must exist as the single generated type bridge.');
} else {
  const bridgeSource = readFileSync(generatedTypesBridgePath, 'utf8');

  if (bridgeSource.includes('generated/server-openapi/src/')) {
    failures.push(
      'composed/src/generated-backend-types.ts must not bridge generated private source paths.',
    );
  }

  if (!bridgeSource.includes("@sdkwork/craw-chat-backend-sdk")) {
    failures.push(
      'composed/src/generated-backend-types.ts must bridge generated types from @sdkwork/craw-chat-backend-sdk.',
    );
  }
}

for (const absolutePath of collectFiles(composedSourceRoot)) {
  const relativePath = path.relative(composedSourceRoot, absolutePath).replace(/\\/g, '/');
  const source = readFileSync(absolutePath, 'utf8');
  const matches = source.match(/generated\/server-openapi\/src\/[^\s'"`]+/g) || [];

  for (const matchedImportPath of matches) {
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

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] TypeScript public API boundary verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] TypeScript public API boundary verification passed.');
