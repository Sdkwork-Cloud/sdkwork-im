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

const REQUIRED_FLUTTER_DOMAINS = [
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

const flutterAssembly = Array.isArray(assembly.languages)
  ? assembly.languages.find((entry) => entry?.language === 'flutter')
  : null;
if (!flutterAssembly || typeof flutterAssembly !== 'object') {
  fail('.sdkwork-assembly.json must include the Flutter language assembly.');
}

const consumerSurface = flutterAssembly.consumerSurface;
if (!consumerSurface || typeof consumerSurface !== 'object') {
  fail('Flutter assembly must expose consumerSurface derived from discoverySurface.');
}
if (consumerSurface.primaryClient !== 'CrawChatClient') {
  fail('Flutter consumerSurface.primaryClient must equal CrawChatClient.');
}

const consumerOperationGroups = sortedUnique(
  Array.isArray(consumerSurface.operationGroups) ? consumerSurface.operationGroups : [],
);
if (JSON.stringify(consumerOperationGroups) !== JSON.stringify(REQUIRED_OPERATION_GROUPS)) {
  fail(
    `Flutter consumerSurface.operationGroups must equal ${REQUIRED_OPERATION_GROUPS.join(', ')}. `
      + `Received ${consumerOperationGroups.join(', ') || '(empty)'}.`,
  );
}

const consumerDomains = Array.isArray(consumerSurface.domains) ? consumerSurface.domains : [];
for (const requiredDomain of REQUIRED_FLUTTER_DOMAINS) {
  const actualDomain = consumerDomains.find((entry) => entry?.name === requiredDomain.name);
  if (!actualDomain) {
    fail(`Flutter consumerSurface.domains must include ${requiredDomain.name}.`);
  }

  const actualOperationGroups = sortedUnique(
    Array.isArray(actualDomain.operationGroups) ? actualDomain.operationGroups : [],
  );
  if (JSON.stringify(actualOperationGroups) !== JSON.stringify(requiredDomain.operationGroups)) {
    fail(
      `Flutter consumerSurface domain ${requiredDomain.name} must cover `
        + `${requiredDomain.operationGroups.join(', ')}. `
        + `Received ${actualOperationGroups.join(', ') || '(empty)'}.`,
    );
  }
}

const sdkSource = read('sdkwork-craw-chat-sdk-flutter/composed/lib/craw_chat_sdk.dart');
if (!sdkSource.includes('class CrawChatClient')) {
  fail('Flutter composed SDK must expose CrawChatClient.');
}

for (const requiredDomain of REQUIRED_FLUTTER_DOMAINS) {
  const declarationPattern = new RegExp(`late final\\s+\\w+\\s+${requiredDomain.name};`);
  if (!declarationPattern.test(sdkSource)) {
    fail(`Flutter composed CrawChatClient must expose late final ${requiredDomain.name}.`);
  }

  const initializerPattern = new RegExp(`${requiredDomain.name}\\s*=\\s*\\w+\\(_context\\);`);
  if (!initializerPattern.test(sdkSource)) {
    fail(`Flutter composed CrawChatClient must initialize ${requiredDomain.name}.`);
  }
}

console.log('[sdkwork-craw-chat-sdk] Flutter discovery alignment verification passed.');
