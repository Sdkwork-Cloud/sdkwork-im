#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  collectMissingTypescriptPackageArtifacts,
  failTypescriptPackageVerification,
  readTypescriptGeneratedPackageJson,
  resolveTypescriptGeneratedPackagePaths,
} from '../../workspace-typescript-package-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk-admin';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const { packageJsonPath, distRoot } = resolveTypescriptGeneratedPackagePaths({
  workspaceRoot,
  relativeGeneratedRoot: path.join(
    'sdkwork-craw-chat-sdk-admin-typescript',
    'generated',
    'server-openapi',
  ),
});
const indexJsPath = path.join(distRoot, 'index.js');
const indexDtsPath = path.join(distRoot, 'index.d.ts');

if (!existsSync(packageJsonPath)) {
  failTypescriptPackageVerification({ prefix, message: 'TypeScript generated package.json is missing.' });
}

const packageJson = readTypescriptGeneratedPackageJson({ packageJsonPath });
if (packageJson.name !== '@sdkwork/craw-chat-admin-backend-sdk') {
  failTypescriptPackageVerification({
    prefix,
    message: 'TypeScript generated package name must stay on @sdkwork/craw-chat-admin-backend-sdk.',
  });
}
if (packageJson.main !== './dist/index.js') {
  failTypescriptPackageVerification({
    prefix,
    message: 'TypeScript generated package main must stay on ./dist/index.js.',
  });
}
const missingArtifacts = collectMissingTypescriptPackageArtifacts({
  distRoot,
  requiredArtifacts: ['index.js', 'index.d.ts'],
});
if (missingArtifacts.length > 0) {
  failTypescriptPackageVerification({
    prefix,
    message: `TypeScript generated package dist artifacts are incomplete: ${missingArtifacts.join(', ')}.`,
  });
}

const generatedModule = await import(pathToFileURL(indexJsPath).href);
for (const exportName of [
  'AdminApiError',
  'DEFAULT_TIMEOUT',
  'CrawChatAdminBackendClient',
  'createClient',
]) {
  if (!(exportName in generatedModule)) {
    failTypescriptPackageVerification({
      prefix,
      message: `TypeScript generated package missing export ${exportName}.`,
    });
  }
}

const client = generatedModule.createClient({
  baseUrl: 'https://admin.example.test',
});
for (const propertyName of ['meta', 'protocol', 'providers', 'social', 'socialRuntime', 'nodes']) {
  if (!client[propertyName]) {
    failTypescriptPackageVerification({
      prefix,
      message: `TypeScript generated client missing module ${propertyName}.`,
    });
  }
}

console.log('[sdkwork-craw-chat-sdk-admin] TypeScript generated package verification passed.');
