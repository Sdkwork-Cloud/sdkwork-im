#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadYamlFromGenerator } from './generator-runtime.mjs';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    base: '',
    derived: '',
    preferDerived: false,
    targetLanguage: '',
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
    if (current === '--prefer-derived') {
      parsed.preferDerived = true;
      continue;
    }
    if (current === '--target-language') {
      parsed.targetLanguage = (argv[index + 1] || '').trim().toLowerCase();
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

async function loadYaml() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  try {
    return await loadYamlFromGenerator(workspaceRoot);
  } catch (error) {
    fail(error instanceof Error ? error.message : String(error));
  }
}

function loadOpenApiDocument(filePath, yaml) {
  if (!existsSync(filePath)) {
    fail(`OpenAPI file not found: ${filePath}`);
  }

  const raw = readFileSync(filePath, 'utf8');
  const trimmed = raw.trim();
  if (!trimmed) {
    fail(`OpenAPI file is empty: ${filePath}`);
  }

  const document = trimmed.startsWith('{') || trimmed.startsWith('[')
    ? JSON.parse(trimmed)
    : yaml.load(raw);

  if (!document || typeof document !== 'object') {
    fail(`OpenAPI file did not decode to an object: ${filePath}`);
  }
  if (typeof document.openapi !== 'string' || !document.openapi.startsWith('3.')) {
    fail(`Unsupported OpenAPI version in ${filePath}`);
  }

  return document;
}

function sdkSurfaceRouteHints() {
  return [
    routeHint(['get', 'post'], '/api/v1/sessions/{*path}', 'session-gateway', 'sessions'),
    routeHint(['get', 'post'], '/api/v1/presence/{*path}', 'session-gateway', 'presence'),
    routeHint(['get', 'post'], '/api/v1/realtime/{*path}', 'session-gateway', 'realtime'),
    routeHint(['get'], '/api/v1/realtime/ws', 'session-gateway', 'realtime', 'websocket', false),
    routeHint(['post'], '/api/v1/devices/register', 'projection-service', 'devices'),
    routeHint(['get'], '/api/v1/devices/{deviceId}/sync-feed', 'projection-service', 'devices'),
    routeHint(['get'], '/api/v1/contacts', 'projection-service', 'conversations'),
    routeHint(['get'], '/api/v1/inbox', 'projection-service', 'conversations'),
    routeHint(['post'], '/api/v1/conversations', 'conversation-runtime', 'conversations'),
    routeHint(['post'], '/api/v1/conversations/{*path}', 'conversation-runtime', 'conversations'),
    routeHint(['post'], '/api/v1/messages/{*path}', 'conversation-runtime', 'conversations'),
    routeHint(['get'], '/api/v1/conversations/{*path}', 'projection-service', 'conversations'),
    routeHint(['post'], '/api/v1/streams', 'streaming-service', 'streams'),
    routeHint(['get', 'post'], '/api/v1/streams/{*path}', 'streaming-service', 'streams'),
    routeHint(['get', 'post'], '/api/v1/rtc/{*path}', 'rtc-signaling-service', 'rtc'),
    routeHint(['get', 'post'], '/api/v1/media/{*path}', 'media-service', 'media'),
    routeHint(['get'], '/api/v1/notifications', 'notification-service', 'notifications'),
    routeHint(
      ['get', 'post'],
      '/api/v1/notifications/{*path}',
      'notification-service',
      'notifications',
    ),
    routeHint(['get', 'post'], '/api/v1/automation/{*path}', 'automation-service', 'automation'),
  ];
}

function routeHint(
  methods,
  pathPattern,
  serviceId,
  operationGroup,
  protocol = 'http',
  generated = true,
) {
  return {
    methods,
    pathPattern,
    serviceId,
    operationGroup,
    protocol,
    generated,
    sdkTarget: 'crawChatAppSdk',
    visibility: 'public',
  };
}

function isHttpOperationKey(key) {
  return ['get', 'post', 'put', 'patch', 'delete', 'options', 'head'].includes(key);
}

function splitPathSegments(pathValue) {
  return String(pathValue || '')
    .trim()
    .replace(/^\/+|\/+$/g, '')
    .split('/')
    .filter(Boolean);
}

function isParamSegment(segment) {
  return segment.startsWith('{') && segment.endsWith('}');
}

function isCatchAllSegment(segment) {
  return segment.startsWith('{*') && segment.endsWith('}');
}

function routeMatchScore(pattern, concretePath) {
  const patternSegments = splitPathSegments(pattern);
  const pathSegments = splitPathSegments(concretePath);
  let score = 0;
  let index = 0;

  while (index < patternSegments.length) {
    const patternSegment = patternSegments[index];
    if (isCatchAllSegment(patternSegment)) {
      return score;
    }

    const pathSegment = pathSegments[index];
    if (!pathSegment) {
      return null;
    }
    if (patternSegment === pathSegment) {
      score += 2;
    } else if (isParamSegment(patternSegment)) {
      score += 1;
    } else {
      return null;
    }
    index += 1;
  }

  return index === pathSegments.length ? score : null;
}

function resolveRouteHint(method, concretePath, hints) {
  let bestMatch = null;
  let bestScore = -1;

  for (const hint of hints) {
    if (!hint.methods.includes(method)) {
      continue;
    }

    const score = routeMatchScore(hint.pathPattern, concretePath);
    if (score === null) {
      continue;
    }
    if (!bestMatch || score > bestScore) {
      bestMatch = hint;
      bestScore = score;
    }
  }

  return bestMatch;
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
  const hints = sdkSurfaceRouteHints();
  const generatedBindings = [];
  const manualBindings = [];
  const unresolved = [];

  for (const [pathKey, pathItem] of Object.entries(document.paths || {})) {
    if (!pathItem || typeof pathItem !== 'object') {
      continue;
    }

    for (const [operationKey, operation] of Object.entries(pathItem)) {
      const method = String(operationKey || '').trim().toLowerCase();
      if (!isHttpOperationKey(method) || !operation || typeof operation !== 'object') {
        continue;
      }

      const hint = resolveRouteHint(method, pathKey, hints);
      if (!hint) {
        unresolved.push(`${method.toUpperCase()} ${pathKey}`);
        continue;
      }

      const binding = {
        operationId:
          typeof operation.operationId === 'string' && operation.operationId.trim()
            ? operation.operationId.trim()
            : `${method}:${pathKey}`,
        method,
        path: pathKey,
        serviceId: hint.serviceId,
        operationGroup: hint.operationGroup,
        sdkTarget: hint.sdkTarget,
        visibility: hint.visibility,
        protocol: hint.protocol,
      };

      if (hint.generated) {
        annotateOperation(operation, binding);
        generatedBindings.push(binding);
      } else {
        manualBindings.push({
          ...binding,
          generated: false,
          reason: 'manual_transport_boundary',
        });
      }
    }
  }

  if (unresolved.length > 0) {
    fail(
      'Unable to resolve SDK discovery metadata for authority operations:\n'
        + unresolved.map((entry) => `- ${entry}`).join('\n'),
    );
  }

  generatedBindings.sort((left, right) =>
    left.path.localeCompare(right.path)
    || left.method.localeCompare(right.method)
    || left.operationId.localeCompare(right.operationId),
  );
  manualBindings.sort((left, right) =>
    left.path.localeCompare(right.path)
    || left.method.localeCompare(right.method)
    || left.operationId.localeCompare(right.operationId),
  );

  document['x-sdkwork-sdk-surface'] = {
    sdkTarget: 'crawChatAppSdk',
    visibility: 'public',
    generatedProtocols: ['http'],
    manualTransports: manualBindings,
    services: summarizeServices(generatedBindings),
    surfaceGroups: summarizeSurfaceGroups(generatedBindings),
    operationBindings: generatedBindings,
  };
}

function stripRealtimeWebsocketPath(document) {
  if (!document.paths || typeof document.paths !== 'object') {
    return;
  }

  delete document.paths['/api/v1/realtime/ws'];
  if (document.info && typeof document.info === 'object') {
    const description = typeof document.info.description === 'string'
      ? document.info.description.trim()
      : '';
    const suffix =
      'Derived sdkgen input excludes the realtime websocket upgrade route. Websocket transport stays manual-owned.';
    document.info.description = description ? `${description}\n${suffix}` : suffix;
  }
}

function isPrimitiveComponentSchema(schema) {
  if (!schema || typeof schema !== 'object' || Array.isArray(schema)) {
    return false;
  }

  if (['string', 'integer', 'number', 'boolean'].includes(schema.type)) {
    return true;
  }

  return schema.type === 'object' && schema.additionalProperties && !schema.properties;
}

function inlinePrimitiveComponentRefs(node, primitiveRefMap) {
  if (Array.isArray(node)) {
    for (let index = 0; index < node.length; index += 1) {
      node[index] = inlinePrimitiveComponentRefs(node[index], primitiveRefMap);
    }
    return node;
  }

  if (!node || typeof node !== 'object') {
    return node;
  }

  if (typeof node.$ref === 'string' && primitiveRefMap.has(node.$ref)) {
    const replacement = cloneJson(primitiveRefMap.get(node.$ref));
    for (const [key, value] of Object.entries(node)) {
      if (key === '$ref') {
        continue;
      }
      replacement[key] = inlinePrimitiveComponentRefs(value, primitiveRefMap);
    }
    return inlinePrimitiveComponentRefs(replacement, primitiveRefMap);
  }

  for (const [key, value] of Object.entries(node)) {
    node[key] = inlinePrimitiveComponentRefs(value, primitiveRefMap);
  }

  return node;
}

function applyFlutterCompatibilityTransforms(document) {
  const schemas = document?.components?.schemas;
  if (!schemas || typeof schemas !== 'object') {
    return;
  }

  const primitiveSchemaEntries = Object.entries(schemas).filter(([, schema]) =>
    isPrimitiveComponentSchema(schema),
  );
  if (primitiveSchemaEntries.length === 0) {
    return;
  }

  const primitiveRefMap = new Map(
    primitiveSchemaEntries.map(([name, schema]) => [`#/components/schemas/${name}`, cloneJson(schema)]),
  );

  inlinePrimitiveComponentRefs(document, primitiveRefMap);

  for (const [name] of primitiveSchemaEntries) {
    delete schemas[name];
  }

  if (document.info && typeof document.info === 'object') {
    const description = typeof document.info.description === 'string'
      ? document.info.description.trim()
      : '';
    const suffix =
      'Flutter-compatible derived sdkgen input expands primitive component refs so the generated Dart models stay strongly typed.';
    document.info.description = description ? `${description}\n${suffix}` : suffix;
  }
}

function writeYamlDocument(filePath, document, yaml) {
  const nextContents = yaml.dump(document, {
    noRefs: true,
    sortKeys: false,
    lineWidth: 120,
  });

  if (existsSync(filePath)) {
    const currentContents = readFileSync(filePath, 'utf8');
    if (currentContents === nextContents) {
      return;
    }
  }

  mkdirSync(path.dirname(filePath), { recursive: true });
  writeFileSync(filePath, nextContents, 'utf8');
}

const args = parseArgs(process.argv.slice(2));
const yaml = await loadYaml();
const basePath = path.resolve(args.base);
const derivedPath = path.resolve(args.derived);
const authority = loadOpenApiDocument(basePath, yaml);
const derived = cloneJson(authority);

applySdkDiscoveryMetadata(derived);
stripRealtimeWebsocketPath(derived);
if (args.targetLanguage === 'flutter') {
  applyFlutterCompatibilityTransforms(derived);
}
writeYamlDocument(derivedPath, derived, yaml);

process.stdout.write(args.preferDerived ? derivedPath : basePath);
