import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const automationSource = read('bin/verify-sdk-automation.mjs');
const docsContractRunnerSource = read('bin/verify-docs-contract-tests.mjs');

assert.match(
  docsContractRunnerSource,
  /verify-sdk-automation-scope-contract\.test\.mjs/,
  'verify-docs-contract-tests.mjs must require verify-sdk-automation-scope-contract.test.mjs.',
);

for (const forbiddenLiteral of [
  "const readmeSource = read('README.md');",
  "const internalDocsReadmeSource = read('docs/README.md');",
  "const packageStandardsSource = read('docs/package-standards.md');",
  "const generationPipelineSource = read('docs/generation-pipeline.md');",
  "const verificationMatrixSource = read('docs/verification-matrix.md');",
  "const realtimeExtensionBoundarySource = read('docs/realtime-extension-boundary.md');",
  "const publicDocsSyncSource = read('docs/sites/README.md');",
  "const typescriptReadmeSource = read('sdkwork-craw-chat-sdk-typescript/README.md');",
  "const typescriptComposedReadmeSource = read('sdkwork-craw-chat-sdk-typescript/composed/README.md');",
  "const typescriptGeneratedReadmeSource = read('sdkwork-craw-chat-sdk-typescript/generated/server-openapi/README.md');",
  "const flutterReadmeSource = read('sdkwork-craw-chat-sdk-flutter/README.md');",
  "const flutterComposedReadmeSource = read('sdkwork-craw-chat-sdk-flutter/composed/README.md');",
  "const flutterGeneratedReadmeSource = read('sdkwork-craw-chat-sdk-flutter/generated/server-openapi/README.md');",
]) {
  assert.doesNotMatch(
    automationSource,
    new RegExp(forbiddenLiteral.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `verify-sdk-automation.mjs must not parse duplicated docs source ${forbiddenLiteral}.`,
  );
}

for (const forbiddenMessage of [
  'Workspace README must document a verification entrypoint.',
  'Workspace README must document the TypeScript single-package standard.',
  'docs/README.md must provide an internal SDK standards or documentation map entrypoint.',
  'docs/package-standards.md must document the TypeScript official package and primary client.',
  'docs/generation-pipeline.md must document the live schema endpoint.',
  'docs/verification-matrix.md must document the root verification entrypoint and automation verifier.',
  'docs/realtime-extension-boundary.md must document the manual-owned realtime extension boundary.',
  'docs/sites/README.md must document the TypeScript public package and primary client.',
  'OpenAPI README must document that the authority snapshot is refreshed from the live service export.',
  'OpenAPI README must document the live service schema endpoint.',
  'TypeScript workspace README must document sdk.createTextMessage(...) as the primary message builder entrypoint.',
  'TypeScript generated README must point app consumers to the root @sdkwork/craw-chat-sdk package and CrawChatSdkClient.',
  'Flutter workspace README must document mounted client.auth/client.portal and sdk.auth/sdk.portal modules.',
  'Flutter generated README must document that realtime websocket runtime remains outside generated delivery.',
]) {
  assert.doesNotMatch(
    automationSource,
    new RegExp(forbiddenMessage.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `verify-sdk-automation.mjs must delegate duplicated docs rule "${forbiddenMessage}" to dedicated verifiers.`,
  );
}

console.log('sdk automation scope contract test passed');
