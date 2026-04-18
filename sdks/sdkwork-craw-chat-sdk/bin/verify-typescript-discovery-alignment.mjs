#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function read(relativePath) {
  const absolutePath = path.join(workspaceRoot, relativePath);
  if (!existsSync(absolutePath)) {
    fail(`Missing required file: ${relativePath}`);
  }
  return readFileSync(absolutePath, 'utf8');
}

function sortedUnique(values) {
  return [...new Set(values)].sort();
}

const REQUIRED_OPERATION_GROUPS = [
  'conversations',
  'devices',
  'media',
  'presence',
  'realtime',
  'rtc',
  'sessions',
  'streams',
];

const REQUIRED_TYPESCRIPT_DOMAINS = [
  { name: 'session', operationGroups: ['sessions'] },
  { name: 'presence', operationGroups: ['presence'] },
  { name: 'realtime', operationGroups: ['realtime'] },
  { name: 'devices', operationGroups: ['devices'] },
  { name: 'inbox', operationGroups: ['conversations'] },
  { name: 'conversations', operationGroups: ['conversations'] },
  { name: 'messages', operationGroups: ['conversations'] },
  { name: 'media', operationGroups: ['media'] },
  { name: 'streams', operationGroups: ['streams'] },
  { name: 'rtc', operationGroups: ['rtc'] },
];

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');
if (!existsSync(assemblyPath)) {
  fail('Missing .sdkwork-assembly.json. Run bin/assemble-sdk.mjs first.');
}

const assembly = readJson(assemblyPath);
const discoverySurface = assembly.discoverySurface;
if (!discoverySurface || typeof discoverySurface !== 'object') {
  fail('.sdkwork-assembly.json must expose discoverySurface sourced from x-sdkwork-sdk-surface.');
}

const surfaceGroups = Array.isArray(discoverySurface.surfaceGroups)
  ? discoverySurface.surfaceGroups
  : [];
if (surfaceGroups.length === 0) {
  fail('assembly.discoverySurface.surfaceGroups must not be empty.');
}

const discoveredOperationGroups = sortedUnique(
  surfaceGroups
    .map((entry) => entry?.operationGroup)
    .filter((value) => typeof value === 'string' && value.length > 0),
);
if (JSON.stringify(discoveredOperationGroups) !== JSON.stringify(REQUIRED_OPERATION_GROUPS)) {
  fail(
    `assembly.discoverySurface.surfaceGroups must expose ${REQUIRED_OPERATION_GROUPS.join(', ')}. `
      + `Received ${discoveredOperationGroups.join(', ') || '(empty)'}.`,
  );
}

const manualTransports = Array.isArray(discoverySurface.manualTransports)
  ? discoverySurface.manualTransports
  : [];
if (
  !manualTransports.some(
    (entry) =>
      entry?.operationGroup === 'realtime'
      && entry?.protocol === 'websocket'
      && entry?.path === '/api/v1/realtime/ws',
  )
) {
  fail(
    'assembly.discoverySurface.manualTransports must include the realtime websocket manual transport boundary.',
  );
}

const typescriptAssembly = Array.isArray(assembly.languages)
  ? assembly.languages.find((entry) => entry?.language === 'typescript')
  : null;
if (!typescriptAssembly || typeof typescriptAssembly !== 'object') {
  fail('.sdkwork-assembly.json must include the TypeScript language assembly.');
}

const consumerSurface = typescriptAssembly.consumerSurface;
if (!consumerSurface || typeof consumerSurface !== 'object') {
  fail('TypeScript assembly must expose consumerSurface derived from discoverySurface.');
}
if (consumerSurface.primaryClient !== 'CrawChatClient') {
  fail('TypeScript consumerSurface.primaryClient must equal CrawChatClient.');
}

const consumerOperationGroups = sortedUnique(
  Array.isArray(consumerSurface.operationGroups) ? consumerSurface.operationGroups : [],
);
if (JSON.stringify(consumerOperationGroups) !== JSON.stringify(REQUIRED_OPERATION_GROUPS)) {
  fail(
    `TypeScript consumerSurface.operationGroups must equal ${REQUIRED_OPERATION_GROUPS.join(', ')}. `
      + `Received ${consumerOperationGroups.join(', ') || '(empty)'}.`,
  );
}

const consumerDomains = Array.isArray(consumerSurface.domains) ? consumerSurface.domains : [];
for (const requiredDomain of REQUIRED_TYPESCRIPT_DOMAINS) {
  const actualDomain = consumerDomains.find((entry) => entry?.name === requiredDomain.name);
  if (!actualDomain) {
    fail(`TypeScript consumerSurface.domains must include ${requiredDomain.name}.`);
  }

  const actualOperationGroups = sortedUnique(
    Array.isArray(actualDomain.operationGroups) ? actualDomain.operationGroups : [],
  );
  if (JSON.stringify(actualOperationGroups) !== JSON.stringify(requiredDomain.operationGroups)) {
    fail(
      `TypeScript consumerSurface domain ${requiredDomain.name} must cover `
        + `${requiredDomain.operationGroups.join(', ')}. `
        + `Received ${actualOperationGroups.join(', ') || '(empty)'}.`,
    );
  }
}

const sdkSource = read('sdkwork-craw-chat-sdk-typescript/composed/src/sdk.ts');
if (!sdkSource.includes('export class CrawChatClient')) {
  fail('TypeScript composed SDK must expose CrawChatClient.');
}

for (const requiredDomain of REQUIRED_TYPESCRIPT_DOMAINS) {
  const propertyPattern = new RegExp(`readonly\\s+${requiredDomain.name}:`);
  if (!propertyPattern.test(sdkSource)) {
    fail(`TypeScript composed CrawChatClient must expose readonly ${requiredDomain.name}.`);
  }

  const initializerPattern = new RegExp(`this\\.${requiredDomain.name}\\s*=\\s*new\\s+`, 'm');
  if (!initializerPattern.test(sdkSource)) {
    fail(`TypeScript composed CrawChatClient must initialize ${requiredDomain.name}.`);
  }
}

console.log('[sdkwork-craw-chat-sdk] TypeScript discovery alignment verification passed.');
