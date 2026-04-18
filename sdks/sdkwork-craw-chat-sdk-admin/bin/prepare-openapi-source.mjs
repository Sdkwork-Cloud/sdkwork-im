#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk-admin] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    base: '',
    derived: '',
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--base') {
      parsed.base = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--derived') {
      parsed.derived = argv[index + 1] || '';
      index += 1;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  if (!parsed.base) {
    fail('Missing required argument: --base');
  }
  if (!parsed.derived) {
    fail('Missing required argument: --derived');
  }

  return parsed;
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}

function loadOpenApiDocument(filePath) {
  if (!existsSync(filePath)) {
    fail(`OpenAPI file not found: ${filePath}`);
  }

  const raw = readFileSync(filePath, 'utf8').trim();
  if (!raw) {
    fail(`OpenAPI file is empty: ${filePath}`);
  }

  const document = JSON.parse(raw);
  if (typeof document.openapi !== 'string' || !document.openapi.startsWith('3.')) {
    fail(`Unsupported OpenAPI version in ${filePath}`);
  }

  return document;
}

function isHttpOperationKey(key) {
  return ['get', 'post', 'put', 'patch', 'delete', 'options', 'head'].includes(key);
}

function classifyOperationGroup(pathKey) {
  if (pathKey === '/healthz') {
    return 'system';
  }
  if (pathKey.startsWith('/api/v1/control/protocol-')) {
    return 'protocol';
  }
  if (pathKey.startsWith('/api/v1/control/provider-')) {
    return 'providers';
  }
  if (pathKey.startsWith('/api/v1/control/social/')) {
    return 'social';
  }
  if (pathKey.startsWith('/api/v1/control/nodes/')) {
    return 'cluster';
  }
  return 'control';
}

function annotateOperation(operation, binding) {
  operation['x-sdkwork-service'] = binding.serviceId;
  operation['x-sdkwork-operation-group'] = binding.operationGroup;
  operation['x-sdkwork-sdk-target'] = binding.sdkTarget;
  operation['x-sdkwork-visibility'] = binding.visibility;
  operation['x-sdkwork-protocol'] = binding.protocol;
}

function summarizeServices(bindings) {
  const grouped = new Map();

  for (const binding of bindings) {
    const current = grouped.get(binding.serviceId) || {
      serviceId: binding.serviceId,
      operationGroups: new Set(),
      protocols: new Set(),
      operationCount: 0,
    };
    current.operationGroups.add(binding.operationGroup);
    current.protocols.add(binding.protocol);
    current.operationCount += 1;
    grouped.set(binding.serviceId, current);
  }

  return [...grouped.values()]
    .map((entry) => ({
      serviceId: entry.serviceId,
      operationGroups: [...entry.operationGroups].sort(),
      protocols: [...entry.protocols].sort(),
      operationCount: entry.operationCount,
    }))
    .sort((left, right) => left.serviceId.localeCompare(right.serviceId));
}

function summarizeSurfaceGroups(bindings) {
  const grouped = new Map();

  for (const binding of bindings) {
    const key = `${binding.serviceId}::${binding.operationGroup}`;
    const current = grouped.get(key) || {
      serviceId: binding.serviceId,
      operationGroup: binding.operationGroup,
      protocols: new Set(),
      operationCount: 0,
    };
    current.protocols.add(binding.protocol);
    current.operationCount += 1;
    grouped.set(key, current);
  }

  return [...grouped.values()]
    .map((entry) => ({
      serviceId: entry.serviceId,
      operationGroup: entry.operationGroup,
      protocols: [...entry.protocols].sort(),
      operationCount: entry.operationCount,
    }))
    .sort((left, right) =>
      left.serviceId.localeCompare(right.serviceId)
      || left.operationGroup.localeCompare(right.operationGroup),
    );
}

function applySdkDiscoveryMetadata(document) {
  const generatedBindings = [];

  for (const [pathKey, pathItem] of Object.entries(document.paths || {})) {
    if (!pathItem || typeof pathItem !== 'object') {
      continue;
    }

    for (const [operationKey, operation] of Object.entries(pathItem)) {
      const method = String(operationKey || '').trim().toLowerCase();
      if (!isHttpOperationKey(method) || !operation || typeof operation !== 'object') {
        continue;
      }

      const binding = {
        operationId:
          typeof operation.operationId === 'string' && operation.operationId.trim()
            ? operation.operationId.trim()
            : `${method}:${pathKey}`,
        method,
        path: pathKey,
        serviceId: 'control-plane-api',
        operationGroup: classifyOperationGroup(pathKey),
        sdkTarget: 'crawChatAdminSdk',
        visibility: 'admin',
        protocol: 'http',
      };

      annotateOperation(operation, binding);
      generatedBindings.push(binding);
    }
  }

  generatedBindings.sort((left, right) =>
    left.path.localeCompare(right.path)
    || left.method.localeCompare(right.method)
    || left.operationId.localeCompare(right.operationId),
  );

  document['x-sdkwork-sdk-surface'] = {
    sdkTarget: 'crawChatAdminSdk',
    visibility: 'admin',
    generatedProtocols: ['http'],
    manualTransports: [],
    services: summarizeServices(generatedBindings),
    surfaceGroups: summarizeSurfaceGroups(generatedBindings),
    operationBindings: generatedBindings,
  };
}

function appendDerivedDescription(document) {
  if (!document.info || typeof document.info !== 'object') {
    return;
  }

  const description = typeof document.info.description === 'string'
    ? document.info.description.trim()
    : '';
  const suffix =
    'Derived sdkgen input for admin SDK assembly. Operation discovery metadata is embedded under x-sdkwork-sdk-surface.';
  document.info.description = description ? `${description}\n${suffix}` : suffix;
}

function writeJson(filePath, document) {
  const nextContents = `${JSON.stringify(document, null, 2)}\n`;
  mkdirSync(path.dirname(filePath), { recursive: true });
  if (!existsSync(filePath) || readFileSync(filePath, 'utf8') !== nextContents) {
    writeFileSync(filePath, nextContents, 'utf8');
  }
}

const args = parseArgs(process.argv.slice(2));
const authority = loadOpenApiDocument(path.resolve(args.base));
const derived = cloneJson(authority);

applySdkDiscoveryMetadata(derived);
appendDerivedDescription(derived);
writeJson(path.resolve(args.derived), derived);

process.stdout.write(path.resolve(args.derived));
