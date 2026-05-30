#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './workspace-sdk-generator-root-shared.mjs';
import {
  cloneOpenApiJson,
  loadOpenApiDocument,
  writeOpenApiYamlDocument,
} from './workspace-openapi-source-shared.mjs';
import { applySdkworkV3OpenApiStandard } from './workspace-openapi-v3-standard.mjs';

const sdkRoot = path.dirname(fileURLToPath(import.meta.url));
const imRoot = path.join(sdkRoot, 'sdkwork-im-sdk');
const backendRoot = path.join(sdkRoot, 'sdkwork-im-backend-sdk');
const appRoot = path.join(sdkRoot, 'sdkwork-im-app-sdk');

const imAuthorityPath = path.join(imRoot, 'openapi', 'craw-chat-im.openapi.yaml');
const imDerivedPath = path.join(imRoot, 'openapi', 'craw-chat-im.sdkgen.yaml');
const imFlutterDerivedPath = path.join(imRoot, 'openapi', 'craw-chat-im.flutter.sdkgen.yaml');
const backendAuthorityPath = path.join(backendRoot, 'openapi', 'craw-chat-backend-api.openapi.yaml');
const backendDerivedPath = path.join(backendRoot, 'openapi', 'craw-chat-backend-api.sdkgen.yaml');
const appAuthorityPath = path.join(appRoot, 'openapi', 'craw-chat-app-api.openapi.yaml');
const appDerivedPath = path.join(appRoot, 'openapi', 'craw-chat-app-api.sdkgen.yaml');
const appFlutterDerivedPath = path.join(appRoot, 'openapi', 'craw-chat-app-api.flutter.sdkgen.yaml');

const backendPrefix = '/backend/v3/api';
const appPrefix = '/app/v3/api';
const imPrefix = '/im/v3/api';
const backendManagementGroups = new Set(['admin', 'audit', 'control', 'ops']);
const appForbiddenManagementGroups = new Set(['admin', 'audit', 'control', 'ops']);

function fail(message) {
  console.error(`[materialize-im-v3-openapi-boundaries] ${message}`);
  process.exit(1);
}

function stripPrefix(pathKey, prefix) {
  return pathKey.startsWith(`${prefix}/`) ? pathKey.slice(prefix.length + 1) : '';
}

function firstGroup(pathKey, prefix) {
  return stripPrefix(pathKey, prefix).split('/').filter(Boolean)[0] || '';
}

function secondGroup(pathKey, prefix) {
  return stripPrefix(pathKey, prefix).split('/').filter(Boolean)[1] || '';
}

function pathWithoutPrefix(pathKey, prefix) {
  const stripped = stripPrefix(pathKey, prefix);
  return stripped ? stripped.replace(/\{[^}]+\}/g, '{}') : '';
}

function rebasePath(pathKey, fromPrefix, toPrefix) {
  if (!pathKey.startsWith(`${fromPrefix}/`)) {
    fail(`Cannot rebase path outside ${fromPrefix}: ${pathKey}`);
  }
  return `${toPrefix}/${stripPrefix(pathKey, fromPrefix)}`;
}

function isImStandardPath(pathKey, prefix) {
  const route = pathWithoutPrefix(pathKey, prefix);
  return (
    route === 'devices/register'
    || route === 'devices/{}/sync_feed'
    || route.startsWith('chat/')
    || route.startsWith('device/sessions/')
    || route.startsWith('media/uploads')
    || route.startsWith('media/{}/')
    || route === 'media/{}'
    || route.startsWith('presence/')
    || route.startsWith('realtime/')
    || route.startsWith('rtc/sessions')
    || route.startsWith('social/')
    || route.startsWith('streams')
  );
}

function isBackendManagementPath(pathKey) {
  const group = firstGroup(pathKey, backendPrefix);
  if (backendManagementGroups.has(group)) {
    return true;
  }
  return group === 'automation' && secondGroup(pathKey, backendPrefix) === 'governance';
}

function isAppManagementPath(pathKey) {
  const group = firstGroup(pathKey, appPrefix);
  if (appForbiddenManagementGroups.has(group)) {
    return true;
  }
  return group === 'automation' && secondGroup(pathKey, appPrefix) === 'governance';
}

function collectRebasedPaths({
  sources,
  fromPrefix,
  toPrefix,
  shouldInclude,
  failOutsidePrefix = true,
}) {
  const paths = {};
  for (const source of sources) {
    for (const [pathKey, pathItem] of Object.entries(source.paths ?? {})) {
      if (!pathKey.startsWith(`${fromPrefix}/`)) {
        if (failOutsidePrefix) {
          fail(`OpenAPI authority contains a path outside ${fromPrefix}: ${pathKey}`);
        }
        continue;
      }
      if (!shouldInclude(pathKey)) {
        continue;
      }
      const nextPathKey = fromPrefix === toPrefix ? pathKey : rebasePath(pathKey, fromPrefix, toPrefix);
      paths[nextPathKey] = cloneOpenApiJson(pathItem);
    }
  }
  return paths;
}

function normalizeBackendOperationIds(document) {
  const opsProviderBindings = document.paths?.['/backend/v3/api/ops/provider_bindings']?.get;
  if (opsProviderBindings) {
    opsProviderBindings.operationId = 'ops.providerBindings.list';
  }

  const opsProviderBindingDrift = document.paths?.['/backend/v3/api/ops/provider_bindings/drift']?.get;
  if (opsProviderBindingDrift) {
    opsProviderBindingDrift.operationId = 'ops.providerBindings.drift.retrieve';
  }

  const controlProviderBindings = document.paths?.['/backend/v3/api/control/provider_bindings']?.get;
  if (controlProviderBindings) {
    controlProviderBindings.operationId = 'control.providerBindings.list';
  }

  const controlProviderBindingsCreate = document.paths?.['/backend/v3/api/control/provider_bindings']?.post;
  if (controlProviderBindingsCreate) {
    controlProviderBindingsCreate.operationId = 'control.providerBindings.create';
  }
}

function normalizeTags(document, allowedTagNames) {
  const existingTags = document.tags ?? [];
  const normalizedTags = [];
  const seen = new Set();
  for (const tag of existingTags) {
    if (!allowedTagNames.has(tag.name) || seen.has(tag.name)) {
      continue;
    }
    normalizedTags.push(cloneOpenApiJson(tag));
    seen.add(tag.name);
  }
  for (const tagName of allowedTagNames) {
    if (!seen.has(tagName)) {
      normalizedTags.push({ name: tagName });
      seen.add(tagName);
    }
  }
  document.tags = normalizedTags;
}

function collectSchemaRefs(value, refs = new Set()) {
  if (!value || typeof value !== 'object') {
    return refs;
  }
  if (Array.isArray(value)) {
    for (const item of value) {
      collectSchemaRefs(item, refs);
    }
    return refs;
  }
  if (typeof value.$ref === 'string') {
    const schemaName = value.$ref.match(/^#\/components\/schemas\/([^/]+)$/)?.[1];
    if (schemaName) {
      refs.add(schemaName);
    }
  }
  for (const child of Object.values(value)) {
    collectSchemaRefs(child, refs);
  }
  return refs;
}

function pruneUnreachableSchemas(document) {
  const schemas = document.components?.schemas;
  if (!schemas || typeof schemas !== 'object') {
    return;
  }

  const reachable = collectSchemaRefs(document.paths ?? {});
  let changed = true;
  while (changed) {
    changed = false;
    for (const schemaName of [...reachable]) {
      const before = reachable.size;
      collectSchemaRefs(schemas[schemaName], reachable);
      if (reachable.size !== before) {
        changed = true;
      }
    }
  }

  for (const schemaName of Object.keys(schemas)) {
    if (schemaName !== 'ProblemDetail' && !reachable.has(schemaName)) {
      delete schemas[schemaName];
    }
  }
}

function normalizeImAuthority(im) {
  const next = cloneOpenApiJson(im);
  next.info = {
    title: 'Craw Chat IM Standardized Development API',
    version: im.info?.version || '0.1.0',
    description:
      'IM standardized development OpenAPI contract for conversations, messages, realtime, media, streams, RTC signaling, and social IM flows.',
  };
  next.paths = collectRebasedPaths({
    sources: [im],
    fromPrefix: imPrefix,
    toPrefix: imPrefix,
    shouldInclude: (pathKey) => isImStandardPath(pathKey, imPrefix),
  });

  const allowedTagNames = new Set();
  for (const pathKey of Object.keys(next.paths)) {
    const group = firstGroup(pathKey, imPrefix);
    if (group) {
      allowedTagNames.add(group);
    }
  }
  normalizeTags(next, allowedTagNames);
  pruneUnreachableSchemas(next);
  return next;
}

function normalizeBackendAuthority(backend) {
  const next = cloneOpenApiJson(backend);
  next.info = {
    title: 'Craw Chat Backend Management API',
    version: backend.info?.version || '0.1.0',
    description:
      'Backend management OpenAPI contract for operator, governance, control, and admin APIs under one SDK family.',
  };
  next.paths = {};

  for (const [pathKey, pathItem] of Object.entries(backend.paths ?? {})) {
    if (!pathKey.startsWith(`${backendPrefix}/`)) {
      fail(`Backend authority contains a path outside ${backendPrefix}: ${pathKey}`);
    }
    if (!isBackendManagementPath(pathKey)) {
      fail(`Backend authority contains non-management API path that belongs in ${appPrefix}: ${pathKey}`);
    }
    next.paths[pathKey] = cloneOpenApiJson(pathItem);
  }

  normalizeTags(next, new Set(['admin', 'audit', 'automation', 'control', 'ops']));
  normalizeBackendOperationIds(next);
  pruneUnreachableSchemas(next);
  return next;
}

function normalizeAppAuthority(app, im) {
  const next = cloneOpenApiJson(app);
  next.info = {
    title: app.info?.title || 'Craw Chat App API',
    version: app.info?.version || '0.1.0',
    description:
      'App-development OpenAPI contract for app-business and non-management HTTP APIs outside the IM standard API.',
  };
  const appNativePaths = collectRebasedPaths({
    sources: [app],
    fromPrefix: appPrefix,
    toPrefix: appPrefix,
    shouldInclude: (pathKey) => !isAppManagementPath(pathKey) && !isImStandardPath(pathKey, appPrefix),
  });
  const appPathsFromImAuthority = collectRebasedPaths({
    sources: [im],
    fromPrefix: imPrefix,
    toPrefix: appPrefix,
    shouldInclude: (pathKey) => !isImStandardPath(pathKey, imPrefix),
  });
  next.paths = {
    ...appPathsFromImAuthority,
    ...appNativePaths,
  };

  const allowedTagNames = new Set();
  for (const pathKey of Object.keys(next.paths)) {
    if (!pathKey.startsWith(`${appPrefix}/`)) {
      fail(`App authority contains a path outside ${appPrefix}: ${pathKey}`);
    }
    if (isAppManagementPath(pathKey)) {
      fail(`App authority contains management API path that belongs in ${backendPrefix}: ${pathKey}`);
    }
    const group = firstGroup(pathKey, appPrefix);
    if (group) {
      allowedTagNames.add(group);
    }
  }

  normalizeTags(next, allowedTagNames);
  pruneUnreachableSchemas(next);
  return next;
}

function appFlutterDerivedDocument(app) {
  const next = cloneOpenApiJson(app);
  for (const pathKey of Object.keys(next.paths ?? {})) {
    if (pathKey.endsWith('/realtime/ws')) {
      delete next.paths[pathKey];
    }
  }
  return next;
}

const yaml = await loadGeneratorYaml(sdkRoot);
const im = loadOpenApiDocument({ prefix: 'sdkwork-im-sdk', filePath: imAuthorityPath, yaml });
const backend = loadOpenApiDocument({ prefix: 'sdkwork-im-backend-sdk', filePath: backendAuthorityPath, yaml });
const app = loadOpenApiDocument({ prefix: 'sdkwork-im-app-sdk', filePath: appAuthorityPath, yaml });

const consolidatedIm = normalizeImAuthority(im);
applySdkworkV3OpenApiStandard(consolidatedIm);
const consolidatedImFlutter = appFlutterDerivedDocument(consolidatedIm);

const consolidatedBackend = normalizeBackendAuthority(backend);
applySdkworkV3OpenApiStandard(consolidatedBackend);

const consolidatedApp = normalizeAppAuthority(app, im);
applySdkworkV3OpenApiStandard(consolidatedApp);
const consolidatedAppFlutter = appFlutterDerivedDocument(consolidatedApp);

if (!consolidatedIm.paths['/im/v3/api/chat/conversations']) {
  fail('IM authority is missing /im/v3/api/chat/conversations.');
}
if (!consolidatedIm.paths['/im/v3/api/realtime/ws']) {
  fail('IM authority is missing /im/v3/api/realtime/ws.');
}
if (consolidatedIm.paths['/im/v3/api/automation/executions']) {
  fail('IM authority must not contain /im/v3/api/automation/executions.');
}
if (consolidatedIm.paths['/im/v3/api/portal/access']) {
  fail('IM authority must not contain /im/v3/api/portal/access.');
}
if (!consolidatedBackend.paths['/backend/v3/api/control/protocol_registry']) {
  fail('Backend authority is missing /backend/v3/api/control/protocol_registry.');
}
if (!consolidatedBackend.paths['/backend/v3/api/admin/api_keys']) {
  fail('Backend authority is missing /backend/v3/api/admin/api_keys.');
}
if (!consolidatedBackend.paths['/backend/v3/api/automation/governance']) {
  fail('Backend authority is missing /backend/v3/api/automation/governance.');
}
if (!consolidatedApp.paths['/app/v3/api/iot/protocol/uplink']) {
  fail('App authority is missing /app/v3/api/iot/protocol/uplink.');
}
if (!consolidatedApp.paths['/app/v3/api/automation/executions']) {
  fail('App authority is missing /app/v3/api/automation/executions.');
}
if (!consolidatedApp.paths['/app/v3/api/rtc/provider_health']) {
  fail('App authority is missing /app/v3/api/rtc/provider_health.');
}
if (consolidatedApp.paths['/app/v3/api/chat/conversations']) {
  fail('App authority must not contain /app/v3/api/chat/conversations.');
}

writeOpenApiYamlDocument({ filePath: imAuthorityPath, document: consolidatedIm, yaml });
writeOpenApiYamlDocument({ filePath: imDerivedPath, document: consolidatedIm, yaml });
writeOpenApiYamlDocument({ filePath: imFlutterDerivedPath, document: consolidatedImFlutter, yaml });
writeOpenApiYamlDocument({ filePath: backendAuthorityPath, document: consolidatedBackend, yaml });
writeOpenApiYamlDocument({ filePath: backendDerivedPath, document: consolidatedBackend, yaml });
writeOpenApiYamlDocument({ filePath: appAuthorityPath, document: consolidatedApp, yaml });
writeOpenApiYamlDocument({ filePath: appDerivedPath, document: consolidatedApp, yaml });
writeOpenApiYamlDocument({ filePath: appFlutterDerivedPath, document: consolidatedAppFlutter, yaml });

console.log('[materialize-im-v3-openapi-boundaries] OpenAPI SDK boundaries materialized.');
