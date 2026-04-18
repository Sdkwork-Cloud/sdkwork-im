#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const typescriptRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript');
const workspaceVerifyPath = path.join(scriptDir, 'verify-typescript-workspace.mjs');
const publishabilityVerifyPath = path.join(
  scriptDir,
  'verify-typescript-single-package-publishability.mjs',
);

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function expectedPackageTaskScript(task) {
  return `call "%npm_node_execpath%" ./bin/package-task.mjs ${task} || "$npm_node_execpath" ./bin/package-task.mjs ${task} || node ./bin/package-task.mjs ${task}`;
}

function read(relativePath) {
  const absolutePath = path.join(typescriptRoot, relativePath);
  if (!existsSync(absolutePath)) {
    fail(`Missing TypeScript single-package file: ${relativePath}`);
  }
  return readFileSync(absolutePath, 'utf8');
}

const packageJsonSource = read('package.json');
const packageJson = JSON.parse(packageJsonSource);
const expectedRootPackageTaskScripts = {
  assemble: expectedPackageTaskScript('assemble'),
  clean: expectedPackageTaskScript('clean'),
  typecheck: expectedPackageTaskScript('typecheck'),
  build: expectedPackageTaskScript('build'),
  smoke: expectedPackageTaskScript('smoke'),
  test: expectedPackageTaskScript('test'),
};
if (packageJson.name !== '@sdkwork/craw-chat-sdk') {
  fail('TypeScript root package name must be @sdkwork/craw-chat-sdk.');
}
for (const [scriptName, expectedCommand] of Object.entries(expectedRootPackageTaskScripts)) {
  if (packageJson.scripts?.[scriptName] !== expectedCommand) {
    fail(`TypeScript root package script "${scriptName}" must execute the local package-task runner through npm-aware Node fallbacks.`);
  }
}

for (const relativePath of ['bin/package-task.mjs']) {
  if (!existsSync(path.join(typescriptRoot, relativePath))) {
    fail(`TypeScript root package must include ${relativePath} for local task execution.`);
  }
}

const requiredFiles = [
  'src/auth-module.ts',
  'src/generated/index.ts',
  'src/generated/sdk.ts',
  'src/generated/types/common.ts',
  'src/index.ts',
  'src/live-module.ts',
  'src/sdk.ts',
  'src/sync-module.ts',
  'src/generated-backend-types.ts',
];
const forbiddenRootSourceFiles = [
  'src/portal-module.ts',
  'src/device-module.ts',
  'src/inbox-module.ts',
  'src/presence-module.ts',
  'src/session-module.ts',
  'src/streams-module.ts',
];

for (const relativePath of requiredFiles) {
  read(relativePath);
}

for (const relativePath of forbiddenRootSourceFiles) {
  if (existsSync(path.join(typescriptRoot, relativePath))) {
    fail(`TypeScript root single-package source must not carry dead authoring module ${relativePath}.`);
  }
}

const rootSdkSource = read('src/sdk.ts');
if (!rootSdkSource.includes('export class CrawChatSdkClient')) {
  fail('TypeScript root SDK must expose CrawChatSdkClient.');
}
if (rootSdkSource.includes('export { CrawChatSdkClient as CrawChatClient')) {
  fail('TypeScript root SDK must not emit the legacy CrawChatClient compatibility alias.');
}
if (!rootSdkSource.includes('readonly auth:')) {
  fail('TypeScript root SDK must expose an auth domain.');
}
if (!rootSdkSource.includes('readonly portal:')) {
  fail('TypeScript root SDK must expose a portal domain.');
}
if (!rootSdkSource.includes('readonly messages:')) {
  fail('TypeScript root SDK must expose a messages domain.');
}
if (!rootSdkSource.includes('readonly sync:')) {
  fail('TypeScript root SDK must expose a sync domain.');
}
if (!rootSdkSource.includes('connect(options')) {
  fail('TypeScript root SDK must expose connect(...) as the live entrypoint.');
}

const rootIndexSource = read('src/index.ts');
if (!rootIndexSource.includes("export * as generated from './generated/index.js';")) {
  fail('TypeScript root package must expose generated exports through a generated namespace.');
}

if (!existsSync(publishabilityVerifyPath)) {
  fail('TypeScript workspace must include verify-typescript-single-package-publishability.mjs.');
}

const workspaceVerifySource = readFileSync(workspaceVerifyPath, 'utf8');
if (!workspaceVerifySource.includes('verify-typescript-single-package-publishability.mjs')) {
  fail('TypeScript workspace verification must execute the single-package publishability verifier.');
}

const generatedTypeBridgeSource = read('src/generated-backend-types.ts');
if (!generatedTypeBridgeSource.includes("from './generated/types/index.js'")) {
  fail('TypeScript root generated type bridge must read request and response types from src/generated/types/index.js.');
}
if (!generatedTypeBridgeSource.includes("from './generated/types/common.js'")) {
  fail('TypeScript root generated type bridge must read common types from src/generated/types/common.js.');
}
if (!generatedTypeBridgeSource.includes("from './generated/types/string-map.js'")) {
  fail('TypeScript root generated type bridge must read string-map types from src/generated/types/string-map.js.');
}

const sdkContextSource = read('src/sdk-context.ts');
if (!sdkContextSource.includes("from './generated/sdk.js'")) {
  fail('TypeScript root sdk-context must create generated clients from the local generated sdk module.');
}
if (sdkContextSource.includes('@sdkwork/craw-chat-backend-sdk')) {
  fail('TypeScript root sdk-context must not depend on @sdkwork/craw-chat-backend-sdk at runtime.');
}
if (!sdkContextSource.includes("'baseUrl or apiBaseUrl is required'")) {
  fail('TypeScript root sdk-context must use public baseUrl/apiBaseUrl language in constructor errors.');
}
if (sdkContextSource.includes("'backendClient, baseUrl, or apiBaseUrl is required'")) {
  fail('TypeScript root sdk-context must not leak backendClient in constructor errors.');
}

const websocketReceiverSource = read('src/websocket-receiver.ts');
if (!websocketReceiverSource.includes(
  "'websocketBaseUrl or connect({ url }) is required to establish realtime connectivity.'",
)) {
  fail('TypeScript root websocket receiver must use connect({ url }) language for missing realtime URLs.');
}
if (!websocketReceiverSource.includes(
  "'No global WebSocket implementation is available. Provide webSocketFactory to establish realtime connectivity in this runtime.'",
)) {
  fail('TypeScript root websocket receiver must use public webSocketFactory guidance for missing global WebSocket runtimes.');
}
if (!websocketReceiverSource.includes(
  "'The default WebSocket implementation cannot attach Authorization headers. Provide webSocketFactory for authenticated realtime connections.'",
)) {
  fail('TypeScript root websocket receiver must use public webSocketFactory guidance for authenticated realtime connections.');
}
if (websocketReceiverSource.includes('realtime websocket receiver')) {
  fail('TypeScript root websocket receiver must not leak internal "realtime websocket receiver" wording.');
}
if (
  websocketReceiverSource.includes('Provide createSocket(...)')
  || websocketReceiverSource.includes('createSocket(...) when creating')
) {
  fail('TypeScript root websocket receiver must not leak the internal createSocket(...) term in public guidance.');
}

const smokeTestSource = read('test/craw-chat-client.test.mjs');
for (const requiredTest of [
  'testConstructorErrorUsesPublicConfigLanguage',
  'testConnectMissingWebSocketUrlUsesPublicLanguage',
  'testConnectDefaultWebSocketFactoryErrorUsesPublicLanguage',
  'testConnectMissingGlobalWebSocketUsesPublicLanguage',
]) {
  if (!smokeTestSource.includes(requiredTest)) {
    fail(`TypeScript root smoke test must include ${requiredTest} after single-package assembly.`);
  }
}

console.log('[sdkwork-craw-chat-sdk] TypeScript single-package layout verification passed.');
