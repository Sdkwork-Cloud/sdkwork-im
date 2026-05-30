import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync, statSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from '../workspace-sdk-generator-root-shared.mjs';
import { loadOpenApiDocument } from '../workspace-openapi-source-shared.mjs';

const testDir = path.dirname(fileURLToPath(import.meta.url));
const sdkRoot = path.resolve(testDir, '..');
const repoRoot = path.resolve(sdkRoot, '..');

function read(relativePath) {
  return readFileSync(path.join(sdkRoot, relativePath), 'utf8');
}

function readRepo(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

function marker(...parts) {
  return parts.join('');
}

function forbiddenPattern(...terms) {
  return new RegExp(terms.map((term) => term.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')).join('|'));
}

function toPosixPath(value) {
  return value.replaceAll('\\', '/');
}

const scannedTextExtensions = new Set([
  '.cs',
  '.dart',
  '.go',
  '.gradle',
  '.java',
  '.js',
  '.json',
  '.kt',
  '.kts',
  '.md',
  '.mjs',
  '.py',
  '.rs',
  '.swift',
  '.toml',
  '.ts',
  '.yaml',
  '.yml',
]);

function collectTextFiles(rootPath) {
  if (!existsSync(rootPath)) {
    return [];
  }
  const stats = statSync(rootPath);
  if (stats.isFile()) {
    return scannedTextExtensions.has(path.extname(rootPath)) ? [rootPath] : [];
  }
  if (!stats.isDirectory()) {
    return [];
  }
  const files = [];
  const visit = (targetPath) => {
    const targetStats = statSync(targetPath);
    if (targetStats.isDirectory()) {
      for (const entry of readdirSync(targetPath)) {
        if (
          entry === 'node_modules'
          || entry === 'dist'
          || entry === 'build'
          || entry === '.dart_tool'
          || entry === '.sdkwork'
          || entry === 'manual-backups'
          || entry === 'tmp'
          || entry === 'locks'
        ) {
          continue;
        }
        visit(path.join(targetPath, entry));
      }
      return;
    }
    if (targetStats.isFile() && scannedTextExtensions.has(path.extname(targetPath))) {
      files.push(targetPath);
    }
  };
  visit(rootPath);
  return files.sort();
}

function assertNoActiveAppBusinessSdkSurfaceInImFamily() {
  const scanTargets = [
    'sdkwork-im-sdk/bin/normalize-generated-auth-surface.mjs',
    'sdkwork-im-sdk/bin/verify-auth-surface-alignment.mjs',
    'sdkwork-im-sdk/sdkwork-im-sdk-typescript/README.md',
    'sdkwork-im-sdk/sdkwork-im-sdk-typescript/composed',
    'sdkwork-im-sdk/sdkwork-im-sdk-typescript/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-flutter/README.md',
    'sdkwork-im-sdk/sdkwork-im-sdk-flutter/composed',
    'sdkwork-im-sdk/sdkwork-im-sdk-flutter/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-rust/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-java/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-csharp/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-swift/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-kotlin/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-go/generated/server-openapi',
    'sdkwork-im-sdk/sdkwork-im-sdk-python/generated/server-openapi',
  ];
  const forbiddenAppBusinessSurfacePatterns = [
    {
      pattern: /(?:\/|\\\/)portal(?:\/|\\\/)/,
      reason: 'active /portal/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)automation(?:\/|\\\/)/,
      reason: 'active /automation/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)notifications(?:\/|\\\/)/,
      reason: 'active /notifications/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)iot(?:\/|\\\/)/,
      reason: 'active /iot/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /(?:\/|\\\/)principal(?:\/|\\\/)/,
      reason: 'active /principal/* routes belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /provider_health|provider_callbacks/,
      reason: 'provider health and callbacks belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /\/twin\b|\bDeviceTwin\b|\bdevicesTwin\b|\bgetDeviceTwin\b|\bupdateDeviceTwin/,
      reason: 'device twin APIs belong in sdkwork-im-app-sdk, not sdkwork-im-sdk',
    },
    {
      pattern: /\bPortalApi\b|\bcreatePortalApi\b/,
      reason: 'generated PortalApi belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bAutomationApi\b|\bcreateAutomationApi\b/,
      reason: 'generated AutomationApi belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bNotificationApi\b|\bcreateNotificationApi\b/,
      reason: 'generated NotificationApi belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bImPortalModule\b/,
      reason: 'composed portal module belongs in app SDK wrappers, not IM SDK',
    },
    {
      pattern: /\bclient\.(?:portal|automation|notification)\b|\bsdk\.portal\b/,
      reason: 'public app-business client surface belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bclient\.deviceSessions\b|\bsdk\.deviceSessions\b|\bdeviceSessions\b/,
      reason: 'device session APIs must be nested under device.sessions in sdkwork-im-sdk',
    },
    {
      pattern: /\breadonly\s+portal\b|\blate\s+final\s+\w*Portal\w*\s+portal\b/,
      reason: 'public portal property belongs in sdkwork-im-app-sdk',
    },
    {
      pattern: /\bPortalSnapshot\b|\bPortalUserView\b|\bPortalWorkspaceView\b|\bDeviceTwin\b/,
      reason: 'app-api DTOs must not be generated by sdkwork-im-sdk',
    },
  ];
  const violations = [];
  for (const scanTarget of scanTargets) {
    const targetPath = path.join(sdkRoot, ...scanTarget.split('/'));
    for (const filePath of collectTextFiles(targetPath)) {
      const source = readFileSync(filePath, 'utf8');
      for (const { pattern, reason } of forbiddenAppBusinessSurfacePatterns) {
        const match = pattern.exec(source);
        if (match) {
          violations.push(`${toPosixPath(path.relative(sdkRoot, filePath))}: ${reason}; matched ${match[0]}`);
        }
      }
    }
  }
  assert.deepEqual(
    violations,
    [],
    'sdkwork-im-sdk must not expose active app-business API routes, generated APIs, DTOs, or composed app-business surfaces.',
  );
}

const sharedFamilySource = read('workspace-im-v3-sdk-family.mjs');
const sharedOpenApiStandardSource = read('workspace-openapi-v3-standard.mjs');
const appConfigSource = read('sdkwork-im-app-sdk/bin/sdk-family-config.mjs');
const backendConfigSource = read('sdkwork-im-backend-sdk/bin/sdk-family-config.mjs');
const imConfigSource = read('sdkwork-im-sdk/bin/sdk-family-config.mjs');
const appGenerateSource = read('sdkwork-im-app-sdk/bin/generate-sdk.mjs');
const backendGenerateSource = read('sdkwork-im-backend-sdk/bin/generate-sdk.mjs');
const imGenerateSource = read('sdkwork-im-sdk/bin/generate-sdk.mjs');
const appVerifySource = read('sdkwork-im-app-sdk/bin/verify-sdk.mjs');
const backendVerifySource = read('sdkwork-im-backend-sdk/bin/verify-sdk.mjs');
const imVerifySource = read('sdkwork-im-sdk/bin/verify-sdk.mjs');
const appPrepareSource = read('sdkwork-im-app-sdk/bin/prepare-openapi-source.mjs');
const backendPrepareSource = read('sdkwork-im-backend-sdk/bin/prepare-openapi-source.mjs');
const appRefreshSource = read('sdkwork-im-app-sdk/bin/refresh-live-openapi-source.mjs');
const backendRefreshSource = read('sdkwork-im-backend-sdk/bin/refresh-live-openapi-source.mjs');
const imPrepareSource = read('sdkwork-im-sdk/bin/prepare-openapi-source.mjs');
const imRefreshSource = read('sdkwork-im-sdk/bin/refresh-live-openapi-source.mjs');
const imGeneratePowerShellSource = read('sdkwork-im-sdk/bin/generate-sdk.ps1');
const imGenerateShellSource = read('sdkwork-im-sdk/bin/generate-sdk.sh');
const sdkWorkspaceIndexSource = read('README.md');
const rtcReadmeSource = read('sdkwork-rtc-sdk/README.md');
const boundaryMaterializerSource = read('materialize-im-v3-openapi-boundaries.mjs');
const yaml = await loadGeneratorYaml(sdkRoot);
const imAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-im-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-sdk/openapi/craw-chat-im.openapi.yaml'),
  yaml,
});
const imDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-sdk/openapi/craw-chat-im.sdkgen.yaml'),
  yaml,
});
const imFlutterDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-sdk/openapi/craw-chat-im.flutter.sdkgen.yaml'),
  yaml,
});
const appAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-im-app-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-app-sdk/openapi/craw-chat-app-api.openapi.yaml'),
  yaml,
});
const appDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-app-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-app-sdk/openapi/craw-chat-app-api.sdkgen.yaml'),
  yaml,
});
const appFlutterDerived = loadOpenApiDocument({
  prefix: 'sdkwork-im-app-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-app-sdk/openapi/craw-chat-app-api.flutter.sdkgen.yaml'),
  yaml,
});
const backendAuthority = loadOpenApiDocument({
  prefix: 'sdkwork-im-backend-sdk',
  filePath: path.join(sdkRoot, 'sdkwork-im-backend-sdk/openapi/craw-chat-backend-api.openapi.yaml'),
  yaml,
});

function pathKeys(document) {
  return Object.keys(document.paths ?? {}).sort();
}

function relativeRoute(pathKey, prefix) {
  assert.ok(pathKey.startsWith(`${prefix}/`), `${pathKey} must start with ${prefix}/.`);
  return pathKey.slice(prefix.length + 1);
}

function firstRouteGroup(pathKey, prefix) {
  return relativeRoute(pathKey, prefix).split('/').filter(Boolean)[0] || '';
}

function routeWithoutPrefix(pathKey, prefix) {
  return relativeRoute(pathKey, prefix).replace(/\{[^}]+\}/g, '{}');
}

function assertAllPathsUsePrefix(label, document, prefix) {
  for (const pathKey of pathKeys(document)) {
    assert.ok(pathKey.startsWith(`${prefix}/`), `${label} path ${pathKey} must use ${prefix}.`);
  }
}

function assertNoPathsUsePrefix(label, document, forbiddenPrefix) {
  for (const pathKey of pathKeys(document)) {
    assert.ok(
      !pathKey.startsWith(`${forbiddenPrefix}/`),
      `${label} path ${pathKey} must not use ${forbiddenPrefix}.`,
    );
  }
}

function isImStandardRoute(route) {
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

function assertDocumentHasOnlyImStandardRoutes(label, document, prefix) {
  for (const pathKey of pathKeys(document)) {
    const route = routeWithoutPrefix(pathKey, prefix);
    assert.ok(
      isImStandardRoute(route),
      `${label} route ${pathKey} is not IM standardized development API and belongs in app-api or backend-api.`,
    );
  }
}

function assertDocumentHasNoImStandardRoutes(label, document, prefix) {
  for (const pathKey of pathKeys(document)) {
    const route = routeWithoutPrefix(pathKey, prefix);
    assert.ok(
      !isImStandardRoute(route),
      `${label} route ${pathKey} duplicates the IM standardized development API and must not be exposed by app-api.`,
    );
  }
}

function assertNoSemanticOverlap(leftLabel, leftDocument, leftPrefix, rightLabel, rightDocument, rightPrefix) {
  const leftRoutes = new Set(pathKeys(leftDocument).map((pathKey) => routeWithoutPrefix(pathKey, leftPrefix)));
  const overlaps = pathKeys(rightDocument)
    .map((pathKey) => routeWithoutPrefix(pathKey, rightPrefix))
    .filter((route) => leftRoutes.has(route));
  assert.deepEqual(
    [...new Set(overlaps)].sort(),
    [],
    `${leftLabel} and ${rightLabel} must not expose cloned semantic routes under different prefixes.`,
  );
}

function operationEntries(document) {
  const entries = [];
  for (const [pathKey, pathItem] of Object.entries(document.paths ?? {})) {
    for (const [method, operation] of Object.entries(pathItem ?? {})) {
      if (!['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'].includes(method)) {
        continue;
      }
      entries.push({ pathKey, method, operation });
    }
  }
  return entries;
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

function assertSchemasArePathReachable(label, document) {
  const schemas = document.components?.schemas ?? {};
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
  const unreachable = Object.keys(schemas)
    .filter((schemaName) => schemaName !== 'ProblemDetail' && !reachable.has(schemaName))
    .sort();
  assert.deepEqual(unreachable, [], `${label} must prune unreachable component schemas.`);
}

const retiredSdkMarkers = [
  marker('sdkwork', '-control', '-plane', '-sdk'),
  marker('sdkwork', '-im', '-admin', '-sdk'),
  marker('@sdkwork', '/control', '-plane', '-sdk'),
  marker('@sdkwork', '/im', '-admin', '-sdk'),
  marker('control', '_plane', '_sdk'),
  marker('im', '_admin', '_sdk'),
  marker('/sdk', '/control', '-plane', '-sdk'),
  marker('/sdk', '/im', '-admin', '-sdk'),
  marker('Control', '-Plane', ' SDK'),
  marker('IM', ' Admin', ' SDK'),
];

for (const verifierSourcePath of [
  'docs/sites/tests/docs-runtime.test.mjs',
  'docs/sites/sdk/verify-sdk-site-docs.mjs',
  'docs/sites/scripts/verify-api-docs.mjs',
  'docs/sites/scripts/verify-sdk-docs.mjs',
  'docs/sites/scripts/verify-docs-site.mjs',
  'sdks/test/verify-im-v3-sdk-family-contract.test.mjs',
]) {
  const verifierSource = readRepo(verifierSourcePath);
  for (const retiredMarker of retiredSdkMarkers) {
    assert.doesNotMatch(
      verifierSource,
      new RegExp(retiredMarker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
      `${verifierSourcePath} must construct retired SDK marker ${retiredMarker} instead of embedding it literally.`,
    );
  }
}

for (const retiredWorkspace of [
  marker('sdkwork', '-control', '-plane', '-sdk'),
  marker('sdkwork', '-im', '-admin', '-sdk'),
]) {
  assert.equal(
    existsSync(path.join(sdkRoot, retiredWorkspace)),
    false,
    `retired SDK workspace ${retiredWorkspace} must not exist under sdks/.`,
  );
  assert.doesNotMatch(
    boundaryMaterializerSource,
    new RegExp(retiredWorkspace.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `boundary materializer must not read retired SDK workspace ${retiredWorkspace}.`,
  );
}

for (const marker of [
  "'typescript'",
  "'flutter'",
  "'rust'",
  "'java'",
  "'csharp'",
  "'swift'",
  "'kotlin'",
  "'go'",
  "'python'",
]) {
  assert.match(
    sharedFamilySource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `shared app/backend SDK family must support official language ${marker}.`,
  );
}

for (const marker of [
  "'--type'",
  'config.sdkType',
  "'--fixed-sdk-version'",
  "'--standard-profile'",
  "'sdkwork-v3'",
  'config.primaryClient',
  'normalizeGeneratedTypeScriptAuthSurface',
  'renderGeneratedTypeScriptReadme',
  'custom/build-runtime.mjs',
  'dist/index.js',
  "'.java'",
  'forbiddenGeneratedAuthSurfaceText',
  'set_api_key',
  'SetApiKey',
  'API_KEY_HEADER',
  'defaultApiKeyHeader',
  'AuthToken',
  'AccessToken',
  'application/problem+json',
  'ProblemDetail',
  'forbiddenPathParts',
  'forbiddenGeneratedText',
]) {
  assert.match(
    sharedFamilySource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `shared app/backend SDK family must enforce ${marker}.`,
  );
}

assert.doesNotMatch(
  sharedFamilySource,
  /--requested-version/,
  'shared app/backend SDK family must not pass --requested-version to the generator version resolver.',
);

for (const marker of [
  'AuthToken',
  'type: \'http\'',
  'scheme: \'bearer\'',
  'AccessToken',
  'type: \'apiKey\'',
  'name: \'Access-Token\'',
  'document.security',
  'ProblemDetail',
  'application/problem+json',
]) {
  assert.match(
    sharedOpenApiStandardSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `sdkwork-v3 OpenAPI standardizer must enforce ${marker}.`,
  );
}

for (const [label, document, prefix] of [
  ['IM authority', imAuthority, '/im/v3/api'],
  ['IM derived', imDerived, '/im/v3/api'],
  ['IM Flutter derived', imFlutterDerived, '/im/v3/api'],
]) {
  assertAllPathsUsePrefix(label, document, prefix);
  assertNoPathsUsePrefix(label, document, '/app/v3/api');
  assertNoPathsUsePrefix(label, document, '/backend/v3/api');
  assertDocumentHasOnlyImStandardRoutes(label, document, prefix);
  assertSchemasArePathReachable(label, document);
}

assertNoActiveAppBusinessSdkSurfaceInImFamily();

for (const [label, document, prefix] of [
  ['app authority', appAuthority, '/app/v3/api'],
  ['app derived', appDerived, '/app/v3/api'],
  ['app Flutter derived', appFlutterDerived, '/app/v3/api'],
]) {
  assertAllPathsUsePrefix(label, document, prefix);
  assertNoPathsUsePrefix(label, document, '/im/v3/api');
  assertNoPathsUsePrefix(label, document, '/backend/v3/api');
  assertDocumentHasNoImStandardRoutes(label, document, prefix);
  for (const pathKey of pathKeys(document)) {
    assert.ok(
      !new Set(['admin', 'audit', 'control', 'ops']).has(firstRouteGroup(pathKey, prefix)),
      `app-api path ${pathKey} is a management route and belongs in backend-api.`,
    );
  }
  assertSchemasArePathReachable(label, document);
}

assertNoSemanticOverlap(
  'IM authority',
  imAuthority,
  '/im/v3/api',
  'app authority',
  appAuthority,
  '/app/v3/api',
);
assert.ok(
  !pathKeys(appFlutterDerived).some((pathKey) => pathKey.endsWith('/realtime/ws')),
  'app Flutter sdkgen input must remove websocket upgrade routes.',
);

assertAllPathsUsePrefix('backend authority', backendAuthority, '/backend/v3/api');
assertNoPathsUsePrefix('backend authority', backendAuthority, '/im/v3/api');
assertNoPathsUsePrefix('backend authority', backendAuthority, '/app/v3/api');
assertSchemasArePathReachable('backend authority', backendAuthority);
for (const pathKey of pathKeys(backendAuthority)) {
  const group = firstRouteGroup(pathKey, '/backend/v3/api');
  assert.ok(
    group === 'automation' || new Set(['admin', 'audit', 'control', 'ops']).has(group),
    `backend-api path ${pathKey} must be management/admin/control/operator scoped.`,
  );
}
for (const [label, document] of [
  ['IM authority', imAuthority],
  ['app authority', appAuthority],
  ['backend authority', backendAuthority],
]) {
  for (const { pathKey, method, operation } of operationEntries(document)) {
    assert.ok(
      !String(operation?.operationId ?? '').startsWith('admin.'),
      `${label} ${method.toUpperCase()} ${pathKey} must not expose a retired admin SDK operation namespace.`,
    );
    assert.ok(
      !String(operation?.operationId ?? '').startsWith('controlPlane.'),
      `${label} ${method.toUpperCase()} ${pathKey} must not expose a retired control-plane SDK operation namespace.`,
    );
  }
}

assert.match(appConfigSource, /sdkType:\s*'app'/, 'app SDK family config must use app sdkType.');
assert.match(appConfigSource, /sdkTarget:\s*'app'/, 'app SDK family config must use app sdkTarget.');
assert.match(appConfigSource, /primaryClient:\s*'SdkworkAppClient'/, 'app SDK family must verify SdkworkAppClient.');
assert.match(
  appConfigSource,
  /generatedApiLabel:\s*'Craw Chat app-development API'/,
  'app SDK family config must label generated app-development transport packages.',
);
assert.match(appConfigSource, /apiPrefix:\s*'\/app\/v3\/api'/, 'app SDK family must target /app/v3/api.');
assert.match(appConfigSource, /schemaUrl:\s*'\/app\/v3\/openapi\.json'/, 'app SDK family must target /app/v3/openapi.json.');
for (const appRequiredPath of [
  '/app/v3/api/portal/access',
  '/app/v3/api/devices/{deviceId}/twin',
  '/app/v3/api/notifications/requests',
  '/app/v3/api/automation/executions',
  '/app/v3/api/media/provider_health',
  '/app/v3/api/principal/profiles/provider_health',
  '/app/v3/api/iot/protocol/uplink',
  '/app/v3/api/rtc/provider_health',
  '/app/v3/api/rtc/provider_callbacks',
]) {
  assert.match(
    appConfigSource,
    new RegExp(appRequiredPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK config must require non-management API path ${appRequiredPath}.`,
  );
}
for (const imStandardPath of [
  '/app/v3/api/device/sessions/resume',
  '/app/v3/api/chat/conversations',
  '/app/v3/api/chat/messages/{messageId}/edit',
  '/app/v3/api/social/friend_requests',
  '/app/v3/api/media/uploads',
  '/app/v3/api/rtc/sessions',
  '/app/v3/api/streams',
]) {
  assert.doesNotMatch(
    appConfigSource,
    new RegExp(imStandardPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `app SDK config must not require IM standard API path ${imStandardPath}.`,
  );
}
assert.doesNotMatch(
  appConfigSource,
  forbiddenPattern(
    marker('chat', '-runtime'),
    marker('device', '-sessions'),
    marker('/api', '/v1'),
    marker('/auth', '/login'),
    marker('/auth', '/me'),
    marker('/portal', '/auth'),
  ),
  'app SDK config must not keep legacy path debt.',
);
assert.doesNotMatch(appConfigSource, /SdkworkBackendClient/, 'app SDK config must reject backend client bleed-through.');

assert.match(backendConfigSource, /sdkType:\s*'backend'/, 'backend SDK family config must use backend sdkType.');
assert.match(backendConfigSource, /sdkTarget:\s*'backend'/, 'backend SDK family config must use backend sdkTarget.');
assert.match(backendConfigSource, /primaryClient:\s*'SdkworkBackendClient'/, 'backend SDK family must verify SdkworkBackendClient.');
assert.match(
  backendConfigSource,
  /generatedApiLabel:\s*'Craw Chat backend\/operator API'/,
  'backend SDK family config must label generated backend/operator transport packages.',
);
assert.match(backendConfigSource, /apiPrefix:\s*'\/backend\/v3\/api'/, 'backend SDK family must target /backend/v3/api.');
assert.match(backendConfigSource, /schemaUrl:\s*'\/backend\/v3\/openapi\.json'/, 'backend SDK family must target /backend/v3/openapi.json.');
for (const backendRequiredPath of [
  '/backend/v3/api/ops/health',
  '/backend/v3/api/audit/records',
  '/backend/v3/api/automation/governance',
  '/backend/v3/api/control/protocol_registry',
  '/backend/v3/api/admin/api_keys',
]) {
  assert.match(
    backendConfigSource,
    new RegExp(backendRequiredPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK config must require consolidated backend path ${backendRequiredPath}.`,
  );
}
assert.doesNotMatch(
  backendConfigSource,
  /marker\('\/api', '\/(?:admin|control)'\)/,
  'backend SDK config must not reject admin or control routes; both are backend modules.',
);
assert.doesNotMatch(
  backendConfigSource,
  forbiddenPattern(
    marker('chat', '-runtime'),
    marker('device', '-sessions'),
    marker('/api', '/v1'),
    marker('/auth', '/login'),
    marker('/auth', '/me'),
    marker('/portal', '/auth'),
  ),
  'backend SDK config must not keep legacy path debt.',
);
assert.doesNotMatch(backendConfigSource, /SdkworkAppClient/, 'backend SDK config must reject app client bleed-through.');
for (const nonManagementBackendPath of [
  '/backend/v3/api/media/provider_health',
  '/backend/v3/api/iot/protocol/uplink',
  '/backend/v3/api/rtc/provider_health',
]) {
  assert.doesNotMatch(
    backendConfigSource,
    new RegExp(nonManagementBackendPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `backend SDK config must not keep non-management API path ${nonManagementBackendPath}.`,
  );
}

assert.match(imConfigSource, /sdkType:\s*'im'/, 'IM SDK family config must use im sdkType.');
assert.match(imConfigSource, /sdkTarget:\s*'im'/, 'IM SDK family config must use im sdkTarget.');
assert.match(imConfigSource, /primaryClient:\s*'SdkworkImClient'/, 'IM SDK family must verify SdkworkImClient.');
assert.match(
  imConfigSource,
  /generatedApiLabel:\s*'Craw Chat IM standardized development API'/,
  'IM SDK family config must label generated IM standardized development transport packages.',
);
assert.match(imConfigSource, /apiPrefix:\s*'\/im\/v3\/api'/, 'IM SDK family must target /im/v3/api.');
assert.match(imConfigSource, /schemaUrl:\s*'\/im\/v3\/openapi\.json'/, 'IM SDK family must target /im/v3/openapi.json.');
for (const imRequiredPath of [
  '/im/v3/api/device/sessions/resume',
  '/im/v3/api/device/sessions/disconnect',
  '/im/v3/api/devices/register',
  '/im/v3/api/devices/{deviceId}/sync_feed',
  '/im/v3/api/chat/conversations',
  '/im/v3/api/chat/messages/{messageId}/edit',
  '/im/v3/api/social/friend_requests',
  '/im/v3/api/media/uploads',
  '/im/v3/api/rtc/sessions',
  '/im/v3/api/streams',
]) {
  assert.match(
    imConfigSource,
    new RegExp(imRequiredPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `IM SDK config must require standardized API path ${imRequiredPath}.`,
  );
}
for (const nonImPath of [
  '/im/v3/api/portal/access',
  '/im/v3/api/devices/{deviceId}/twin',
  '/im/v3/api/notifications/requests',
  '/im/v3/api/automation/executions',
  '/im/v3/api/media/provider_health',
  '/im/v3/api/principal/profiles/provider_health',
  '/im/v3/api/iot/protocol/uplink',
]) {
  assert.doesNotMatch(
    imConfigSource,
    new RegExp(nonImPath.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `IM SDK config must not require app-business API path ${nonImPath}.`,
  );
}
assert.doesNotMatch(imConfigSource, /SdkworkAppClient/, 'IM SDK config must reject app client bleed-through.');
assert.doesNotMatch(imConfigSource, /SdkworkBackendClient/, 'IM SDK config must reject backend client bleed-through.');

for (const [label, document] of [
  ['IM authority', imAuthority],
  ['IM derived', imDerived],
  ['IM Flutter derived', imFlutterDerived],
]) {
  const operationIds = operationEntries(document).map(({ operation }) => String(operation?.operationId ?? ''));
  assert.ok(
    operationIds.includes('device.sessions.resume'),
    `${label} must expose device.sessions.resume as a dotted SDK operation.`,
  );
  assert.ok(
    operationIds.includes('device.sessions.disconnect'),
    `${label} must expose device.sessions.disconnect as a dotted SDK operation.`,
  );
  assert.ok(
    !operationIds.some((operationId) => /deviceSessions|DeviceSessions/.test(operationId)),
    `${label} must not expose top-level deviceSessions operation namespace.`,
  );
}

for (const [label, source] of [
  ['app generate', appGenerateSource],
  ['backend generate', backendGenerateSource],
  ['IM generate', imGenerateSource],
]) {
  assert.match(source, /runGenerateSdkFamily/, `${label} entrypoint must delegate to shared SDK family generation.`);
  assert.match(source, /sdkFamilyConfig/, `${label} entrypoint must use its local SDK family config.`);
}

assert.match(
  imGenerateSource,
  /assemble-sdk\.mjs/,
  'IM generate entrypoint must assemble layered TypeScript/Flutter workspaces after generator output is verified.',
);
assert.doesNotMatch(
  imGenerateSource,
  /normalize-generated-auth-surface/,
  'IM generate entrypoint must not call the retired generated auth surface normalizer.',
);

for (const [label, source] of [
  ['app verify', appVerifySource],
  ['backend verify', backendVerifySource],
  ['IM verify', imVerifySource],
]) {
  assert.match(source, /runVerifySdkFamily/, `${label} entrypoint must delegate to shared SDK family verification.`);
  assert.match(source, /sdkFamilyConfig/, `${label} entrypoint must use its local SDK family config.`);
}

for (const [label, source] of [
  ['app prepare', appPrepareSource],
  ['backend prepare', backendPrepareSource],
  ['im prepare', imPrepareSource],
  ['app refresh', appRefreshSource],
  ['backend refresh', backendRefreshSource],
  ['im refresh', imRefreshSource],
]) {
  assert.match(
    source,
    /applySdkworkV3OpenApiStandard/,
    `${label} must apply the shared sdkwork-v3 OpenAPI standardizer.`,
  );
}

for (const [label, source] of [
  ['im PowerShell generate', imGeneratePowerShellSource],
  ['im shell generate', imGenerateShellSource],
]) {
  assert.match(source, /fixed-sdk-version/, `${label} must use fixed sdk version resolution.`);
  assert.doesNotMatch(source, /requested-version|RequestedVersion/, `${label} must not keep requested-version debt.`);
}

for (const marker of [
  'sdkwork-im-sdk',
  'sdkwork-im-app-sdk',
  'sdkwork-im-backend-sdk',
  'sdkwork-rtc-sdk',
  '/im/v3/api',
  '/app/v3/api',
  '/backend/v3/api',
]) {
  assert.match(
    sdkWorkspaceIndexSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `SDK workspace index must document ${marker}.`,
  );
}

for (const retiredPublicFamily of retiredSdkMarkers.slice(0, 4)) {
  assert.doesNotMatch(
    sdkWorkspaceIndexSource,
    new RegExp(retiredPublicFamily.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `SDK workspace index must not publish retired SDK family ${retiredPublicFamily}.`,
  );
}

assert.match(
  sdkWorkspaceIndexSource,
  /\/backend\/v3\/api\/control\/\*/,
  'SDK workspace index must document control routes as backend SDK modules.',
);
assert.match(
  sdkWorkspaceIndexSource,
  /\/backend\/v3\/api\/admin\/\*/,
  'SDK workspace index must document admin routes as backend SDK modules.',
);
assert.match(
  rtcReadmeSource,
  /not a route-generated SDK workspace|does not[\s\S]*OpenAPI-generated transport problem/i,
  'RTC SDK README must keep RTC independent from OpenAPI-generated HTTP SDK families.',
);

assert.doesNotMatch(
  sdkWorkspaceIndexSource,
  /Generator-ready/,
  'SDK workspace index must be tightened to list generated app/backend SDK languages instead of Generator-ready placeholders.',
);

console.log('im v3 sdk family contract test passed');
