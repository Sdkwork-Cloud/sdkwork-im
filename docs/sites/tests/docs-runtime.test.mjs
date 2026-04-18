import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

import * as viteRuntimeLib from '../../../scripts/dev/vite-runtime-lib.mjs';

const docsRoot = path.resolve(import.meta.dirname, '..');
const currentWorkspaceRoot = path.resolve(docsRoot, '..', '..');

function resolveCanonicalDocsRoot(workspaceRoot) {
  const isWorktreeCheckout = path.basename(path.dirname(workspaceRoot)) === '.worktrees';
  const canonicalRepoRoot = isWorktreeCheckout
    ? path.resolve(workspaceRoot, '..', '..')
    : workspaceRoot;

  return path.join(canonicalRepoRoot, 'docs', 'sites');
}

test('workspace donor roots include the canonical docs site when local docs node_modules are empty', () => {
  const donorRoots = viteRuntimeLib.resolveWorkspaceDonorRoots(docsRoot);
  const canonicalDocsRoot = resolveCanonicalDocsRoot(currentWorkspaceRoot);

  assert.equal(
    existsSync(path.join(canonicalDocsRoot, 'node_modules', 'vitepress', 'package.json')),
    true,
    'test setup should include a canonical docs site donor with vitepress installed',
  );
  assert.equal(
    donorRoots.includes(canonicalDocsRoot),
    true,
    'docs/sites should discover the canonical docs site as a donor root',
  );
});

test('media API reference documents upload session mutation responses', () => {
  const mediaDoc = readFileSync(path.join(docsRoot, 'api-reference', 'app', 'media.md'), 'utf8');

  assert.match(mediaDoc, /Success<\/strong><span>`200 MediaUploadMutationResponse`<\/span>/);
  assert.match(mediaDoc, /ApiSchemaTable schema="MediaUploadMutationResponse"/);
  assert.match(mediaDoc, /ApiSchemaTable schema="MediaUploadSession"/);
  assert.match(mediaDoc, /presigned upload session/i);
});

test('authority openapi contract exposes media upload mutation schemas', () => {
  const workspaceRoot = path.resolve(docsRoot, '..', '..');
  const openapiPath = path.join(
    workspaceRoot,
    'sdks',
    'sdkwork-craw-chat-sdk',
    'openapi',
    'craw-chat-app.openapi.yaml',
  );
  const openapi = readFileSync(openapiPath, 'utf8');

  assert.match(openapi, /operationId:\s*createMediaUpload[\s\S]*MediaUploadMutationResponse/);
  assert.match(openapi, /operationId:\s*completeMediaUpload[\s\S]*MediaUploadMutationResponse/);
  assert.match(openapi, /MediaUploadMutationResponse:/);
  assert.match(openapi, /MediaUploadSession:/);
});

test('sdk docs describe Flutter presigned upload flow', () => {
  const flutterDoc = readFileSync(path.join(docsRoot, 'sdk', 'flutter-sdk.md'), 'utf8');

  assert.match(flutterDoc, /sdk\.media\.upload\(/);
  assert.match(flutterDoc, /MediaUploadMutationResponse/);
  assert.match(flutterDoc, /MediaUploadSession/);
  assert.match(flutterDoc, /presigned upload session/i);
});

test('sdk docs describe TypeScript flat client creation and upload flow', () => {
  const typescriptDoc = readFileSync(path.join(docsRoot, 'sdk', 'typescript-sdk.md'), 'utf8');

  assert.match(typescriptDoc, /CrawChatSdkClient\.create\(\{\s*baseUrl:/);
  assert.doesNotMatch(typescriptDoc, /backendConfig/);
  assert.match(typescriptDoc, /sdk\.media\.upload\(/);
  assert.match(typescriptDoc, /MediaUploadMutationResponse/);
});

test('language support doc links to the dedicated TypeScript and Flutter SDK references', () => {
  const languageSupportDoc = readFileSync(path.join(docsRoot, 'sdk', 'language-support.md'), 'utf8');

  assert.match(languageSupportDoc, /\[TypeScript SDK\]\(\/sdk\/typescript-sdk\)/);
  assert.match(languageSupportDoc, /\[Flutter SDK\]\(\/sdk\/flutter-sdk\)/);
  assert.match(languageSupportDoc, /\[App SDK Overview\]\(\/sdk\/app-sdk\)/);
});

test('app sdk overview documents assembly metadata and verified workspace semantics', () => {
  const appSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'app-sdk.md'), 'utf8');

  assert.match(appSdkDoc, /\.sdkwork-assembly\.json/);
  assert.match(appSdkDoc, /manifestPath/);
  assert.match(appSdkDoc, /generatedAt/);
  assert.match(appSdkDoc, /generated[\s\S]*composed/i);
  assert.match(appSdkDoc, /verify-sdk\.mjs/);
});

test('admin sdk overview links to the dedicated admin TypeScript and Flutter SDK references', () => {
  const adminSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'admin-sdk.md'), 'utf8');

  assert.match(adminSdkDoc, /\[Admin TypeScript SDK\]\(\/sdk\/admin-typescript-sdk\)/);
  assert.match(adminSdkDoc, /\[Admin Flutter SDK\]\(\/sdk\/admin-flutter-sdk\)/);
  assert.match(adminSdkDoc, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(adminSdkDoc, /craw_chat_admin_sdk/);
  assert.match(adminSdkDoc, /\/api-reference\/control-plane\/social/);
  assert.match(adminSdkDoc, /\/api-reference\/control-plane\/social-runtime/);
});

test('admin sdk overview documents assembly metadata and stable package-layer release semantics', () => {
  const adminSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'admin-sdk.md'), 'utf8');

  assert.match(adminSdkDoc, /\.sdkwork-assembly\.json/);
  assert.match(adminSdkDoc, /manifestPath/);
  assert.match(adminSdkDoc, /generatedAt/);
  assert.match(adminSdkDoc, /generated[\s\S]*composed/i);
  assert.match(adminSdkDoc, /stable when assembly content is unchanged/i);
});

test('admin TypeScript sdk docs describe flat client creation and the browser admin helper surface', () => {
  const adminTypescriptDoc = readFileSync(
    path.join(docsRoot, 'sdk', 'admin-typescript-sdk.md'),
    'utf8',
  );

  assert.match(adminTypescriptDoc, /CrawChatAdminSdkClient\.create\(\{\s*baseUrl:/);
  assert.doesNotMatch(adminTypescriptDoc, /backendConfig/);
  assert.match(adminTypescriptDoc, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(adminTypescriptDoc, /loginAdminUser/);
  assert.match(adminTypescriptDoc, /\/api\/admin\/\*/);
  assert.match(adminTypescriptDoc, /\/api-reference\/control-plane\/social/);
  assert.match(adminTypescriptDoc, /\/api-reference\/control-plane\/social-runtime/);
});

test('admin Flutter sdk docs describe flat client creation and native Dart verification', () => {
  const adminFlutterDoc = readFileSync(
    path.join(docsRoot, 'sdk', 'admin-flutter-sdk.md'),
    'utf8',
  );

  assert.match(adminFlutterDoc, /package:craw_chat_admin_sdk\/craw_chat_admin_sdk\.dart/);
  assert.match(adminFlutterDoc, /CrawChatAdminSdkClient\.create\(\s*baseUrl:/);
  assert.doesNotMatch(adminFlutterDoc, /backendConfig/);
  assert.match(adminFlutterDoc, /verify-flutter-workspace\.mjs --with-dart/);
  assert.match(adminFlutterDoc, /verify-flutter-dart-analysis\.dart/);
  assert.match(adminFlutterDoc, /\/api-reference\/control-plane\/social/);
  assert.match(adminFlutterDoc, /\/api-reference\/control-plane\/social-runtime/);
});

test('language support doc links to the dedicated admin TypeScript and Flutter SDK references', () => {
  const languageSupportDoc = readFileSync(path.join(docsRoot, 'sdk', 'language-support.md'), 'utf8');

  assert.match(languageSupportDoc, /\[Admin TypeScript SDK\]\(\/sdk\/admin-typescript-sdk\)/);
  assert.match(languageSupportDoc, /\[Admin Flutter SDK\]\(\/sdk\/admin-flutter-sdk\)/);
});

test('control-plane overview documents admin sdk alignment and social domains', () => {
  const controlPlaneDoc = readFileSync(
    path.join(docsRoot, 'api-reference', 'control-plane-api.md'),
    'utf8',
  );

  assert.doesNotMatch(
    controlPlaneDoc,
    /does not yet include a checked-in admin OpenAPI authority file/i,
  );
  assert.match(controlPlaneDoc, /\/sdk\/admin-sdk/);
  assert.match(controlPlaneDoc, /\/sdk\/admin-typescript-sdk/);
  assert.match(controlPlaneDoc, /\/sdk\/admin-flutter-sdk/);
  assert.match(controlPlaneDoc, /\/api-reference\/control-plane\/social/);
  assert.match(controlPlaneDoc, /\/api-reference\/control-plane\/social-runtime/);
});

test('cli docs describe admin sdk refresh, verification, and assembly commands', () => {
  const cliDoc = readFileSync(path.join(docsRoot, 'reference', 'cli-and-scripts.md'), 'utf8');

  assert.match(cliDoc, /sdkwork-craw-chat-sdk-admin/);
  assert.match(cliDoc, /fetch-openapi-source\.mjs/);
  assert.match(cliDoc, /prepare-openapi-source\.mjs/);
  assert.match(cliDoc, /verify-sdk\.mjs --language typescript --language flutter/);
  assert.match(cliDoc, /\.sdkwork-assembly\.json/);
});

test('cli docs describe app sdk verification and assembly commands', () => {
  const cliDoc = readFileSync(path.join(docsRoot, 'reference', 'cli-and-scripts.md'), 'utf8');

  assert.match(cliDoc, /sdkwork-craw-chat-sdk/);
  assert.match(cliDoc, /craw-chat-app\.sdkgen\.yaml/);
  assert.match(cliDoc, /craw-chat-app\.flutter\.sdkgen\.yaml/);
  assert.match(cliDoc, /node \.\\sdks\\sdkwork-craw-chat-sdk\\bin\\verify-sdk\.mjs/);
  assert.match(cliDoc, /\.sdkwork-assembly\.json/);
});

test('typescript sdk guide documents package contract, assembly metadata, and maintainer workflow', () => {
  const typescriptDoc = readFileSync(path.join(docsRoot, 'sdk', 'typescript-sdk.md'), 'utf8');

  assert.match(typescriptDoc, /Current Delivery Reality/);
  assert.match(typescriptDoc, /Package Contract/);
  assert.match(typescriptDoc, /Local Workspace Workflow/);
  assert.match(typescriptDoc, /What To Read Next/);
  assert.match(typescriptDoc, /@sdkwork\/craw-chat-sdk/);
  assert.match(typescriptDoc, /@sdkwork\/craw-chat-backend-sdk/);
  assert.match(typescriptDoc, /CrawChatSdkClient/);
  assert.match(typescriptDoc, /generated\/server-openapi/);
  assert.match(typescriptDoc, /composed/);
  assert.match(typescriptDoc, /\.sdkwork-assembly\.json/);
  assert.match(typescriptDoc, /manifestPath/);
  assert.match(typescriptDoc, /generatedAt/);
  assert.match(typescriptDoc, /verify-sdk\.mjs/);
  assert.match(typescriptDoc, /verify-typescript-workspace\.mjs/);
  assert.match(typescriptDoc, /\/sdk\/app-sdk/);
  assert.match(typescriptDoc, /\/sdk\/language-support/);
  assert.match(typescriptDoc, /\/api-reference\/app\/media/);
});

test('flutter sdk guide documents current parity, assembly metadata, and local workspace workflow', () => {
  const flutterDoc = readFileSync(path.join(docsRoot, 'sdk', 'flutter-sdk.md'), 'utf8');

  assert.match(flutterDoc, /Current Delivery Reality/);
  assert.match(flutterDoc, /Package Contract/);
  assert.match(flutterDoc, /Current Surface Reality/);
  assert.match(flutterDoc, /Current Parity Gap/);
  assert.match(flutterDoc, /Local Workspace Workflow/);
  assert.match(flutterDoc, /What To Read Next/);
  assert.match(flutterDoc, /craw_chat_sdk/);
  assert.match(flutterDoc, /backend_sdk/);
  assert.match(flutterDoc, /CrawChatSdkClient/);
  assert.match(flutterDoc, /CrawChatBuilders/);
  assert.match(flutterDoc, /generated\/server-openapi/);
  assert.match(flutterDoc, /pubspec_overrides\.yaml/);
  assert.doesNotMatch(flutterDoc, /backendConfig/);
  assert.match(flutterDoc, /\.sdkwork-assembly\.json/);
  assert.match(flutterDoc, /manifestPath/);
  assert.match(flutterDoc, /generatedAt/);
  assert.match(flutterDoc, /verify-sdk\.mjs --with-dart/);
  assert.match(flutterDoc, /\/sdk\/app-sdk/);
  assert.match(flutterDoc, /\/sdk\/language-support/);
  assert.match(flutterDoc, /\/api-reference\/app\/media/);
});

test('language support guide explains workspace boundaries, official package names, and release semantics', () => {
  const languageSupportDoc = readFileSync(path.join(docsRoot, 'sdk', 'language-support.md'), 'utf8');

  assert.match(languageSupportDoc, /Current Verified Baseline/);
  assert.match(languageSupportDoc, /How To Use This Page/);
  assert.match(languageSupportDoc, /repo contract/i);
  assert.match(languageSupportDoc, /generated\/server-openapi/);
  assert.match(languageSupportDoc, /composed/);
  assert.match(languageSupportDoc, /\.sdkwork-assembly\.json/);
  assert.match(languageSupportDoc, /@sdkwork\/craw-chat-sdk/);
  assert.match(languageSupportDoc, /craw_chat_sdk/);
  assert.match(languageSupportDoc, /@sdkwork\/craw-chat-admin-sdk/);
  assert.match(languageSupportDoc, /craw_chat_admin_sdk/);
  assert.match(languageSupportDoc, /verify-sdk\.mjs/);
  assert.match(languageSupportDoc, /not_published/);
  assert.match(languageSupportDoc, /\/sdk\/typescript-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/flutter-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/admin-typescript-sdk/);
  assert.match(languageSupportDoc, /\/sdk\/admin-flutter-sdk/);
});

test('admin sdk overview does not document backendConfig as public create surface', () => {
  const adminSdkDoc = readFileSync(path.join(docsRoot, 'sdk', 'admin-sdk.md'), 'utf8');

  assert.doesNotMatch(adminSdkDoc, /backendConfig/);
});
