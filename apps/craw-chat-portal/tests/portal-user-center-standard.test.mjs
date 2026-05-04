import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import { pathToFileURL } from 'node:url';
import test from 'node:test';

import {
  resolvePortalAppRoot,
  resolvePortalAppbaseRoot,
} from './helpers/portal-paths.mjs';

const appRoot = resolvePortalAppRoot(import.meta.url);
const appbaseRoot = resolvePortalAppbaseRoot(import.meta.url);

function read(relativePath) {
  return readFileSync(path.join(appRoot, relativePath), 'utf8');
}

function readAppbase(relativePath) {
  return readFileSync(path.join(appbaseRoot, relativePath), 'utf8');
}

function collectSourceFiles(directory) {
  return readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const resolvedPath = path.join(directory, entry.name);

    if (entry.isDirectory()) {
      return collectSourceFiles(resolvedPath);
    }

    return entry.isFile() && /\.(?:[cm]?js|tsx?)$/u.test(entry.name) ? [resolvedPath] : [];
  });
}

function normalizeTestPath(filePath) {
  return filePath.split(path.sep).join('/');
}

async function loadPortalUserCenterBridge() {
  return import(
    `${pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/userCenter.js'),
    ).href}?cacheBust=${Date.now()}`
  );
}

async function loadPortalValidationBridge() {
  return import(
    `${pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/validation.js'),
    ).href}?cacheBust=${Date.now()}`
  );
}

async function loadPortalUserCenterRuntimeBridge() {
  return import(
    `${pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/userCenterRuntime.js'),
    ).href}?cacheBust=${Date.now()}`
  );
}

async function loadPortalApi() {
  return import(
    `${pathToFileURL(
      path.join(appRoot, 'packages/craw-chat-portal-portal-api/src/index.js'),
    ).href}?cacheBust=${Date.now()}`
  );
}

function storageDouble() {
  const store = new Map();

  return {
    getItem(key) {
      return store.has(key) ? store.get(key) : null;
    },
    setItem(key, value) {
      store.set(key, String(value));
    },
    removeItem(key) {
      store.delete(key);
    },
  };
}

function throwingStorageDouble() {
  return {
    getItem() {
      throw new Error('storage unavailable');
    },
    setItem() {
      throw new Error('storage unavailable');
    },
    removeItem() {
      throw new Error('storage unavailable');
    },
  };
}

function runtimeResponseDouble(payload, { headers = {}, ok = true, status = 200 } = {}) {
  const normalizedHeaders = new Map(
    Object.entries(headers).map(([key, value]) => [key.toLowerCase(), value]),
  );

  return {
    headers: {
      get(name) {
        return normalizedHeaders.get(name.toLowerCase()) ?? null;
      },
    },
    json: async () => payload,
    ok,
    status,
  };
}

function createStandardUserCenterAuthEnvNames(prefix) {
  return [
    `${prefix}AUTHORIZATION_HEADER_NAME`,
    `${prefix}ACCESS_TOKEN_HEADER_NAME`,
    `${prefix}REFRESH_TOKEN_HEADER_NAME`,
    `${prefix}SESSION_HEADER_NAME`,
    `${prefix}AUTHORIZATION_SCHEME`,
    `${prefix}ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN`,
  ];
}

test('craw-chat portal user-center bridge aligns to the sdkwork-appbase canonical core package through the public barrel', () => {
  const upstreamIndexPath = path.join(
    appbaseRoot,
    'packages/pc-react/identity/sdkwork-user-center-core-pc-react/src/index.ts',
  );
  const bridgePath = path.join(
    appRoot,
    'packages/craw-chat-portal-portal-api/src/userCenter.js',
  );

  assert.equal(existsSync(upstreamIndexPath), true);
  assert.equal(existsSync(bridgePath), true);

  const upstreamIndexSource = readAppbase(
    'packages/pc-react/identity/sdkwork-user-center-core-pc-react/src/index.ts',
  );
  const upstreamConfigSource = readAppbase(
    'packages/pc-react/identity/sdkwork-user-center-core-pc-react/src/domain/userCenterConfig.ts',
  );
  const upstreamTypesSource = readAppbase(
    'packages/pc-react/identity/sdkwork-user-center-core-pc-react/src/types/userCenterTypes.ts',
  );
  const bridgeSource = read('packages/craw-chat-portal-portal-api/src/userCenter.js');

  assert.match(upstreamIndexSource, /@sdkwork\/user-center-core-pc-react/);
  assert.match(upstreamConfigSource, /USER_CENTER_STANDARD_ENTITY_NAMES/);
  assert.match(
    upstreamTypesSource,
    /export type UserCenterIntegrationKind =[\s\S]*"builtin-local"[\s\S]*"sdkwork-cloud-app-api"[\s\S]*"external-user-center";/u,
  );
  assert.match(upstreamTypesSource, /export interface UserCenterStorageTopology/);

  for (const exportName of [
    'CRAW_CHAT_PORTAL_USER_CENTER_SOURCE_PACKAGE',
    'CRAW_CHAT_PORTAL_USER_CENTER_NAMESPACE',
    'CRAW_CHAT_PORTAL_USER_CENTER_STANDARD_ENTITIES',
    'CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN',
    'CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API',
    'CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
    'CRAW_CHAT_PORTAL_USER_CENTER_ROUTES',
    'CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_PREFIX',
    'CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_PREFIX',
    'CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_ARTIFACT_BASENAME',
    'CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_ARTIFACT_BASENAME',
    'createCrawChatPortalUserCenterHandshakeSigningMessage',
    'createCrawChatPortalUserCenterHandshakeVerificationContext',
    'createCrawChatPortalUserCenterSignedHandshakeHeaders',
    'createCrawChatPortalUserCenterConfig',
    'createCrawChatPortalUserCenterPluginDefinition',
    'createCrawChatPortalUserCenterPortalDeploymentProfiles',
    'createCrawChatPortalUserCenterSessionStore',
    'createCrawChatPortalUserCenterTokenStore',
  ]) {
    assert.match(bridgeSource, new RegExp(`export\\s+(?:const|function)\\s+${exportName}`));
  }

  assert.match(bridgeSource, /USER_CENTER_SOURCE_PACKAGE_NAME/);
  assert.match(bridgeSource, /sdkwork-user-center-core-pc-react\/src\/index\.ts/);
  assert.doesNotMatch(bridgeSource, /sdkwork-user-center-core-pc-react\/src\/domain\//);
  assert.doesNotMatch(bridgeSource, /sdkwork-user-center-core-pc-react\/src\/types\//);
  assert.match(bridgeSource, /USER_CENTER_STANDARD_ENTITY_NAMES/);
  assert.match(bridgeSource, /CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH/);
  assert.match(bridgeSource, /createUserCenterBridgeConfig/);
  assert.match(bridgeSource, /createUserCenterLocalApiRoutes/);
  assert.match(bridgeSource, /createUserCenterStoragePlan/);
  assert.doesNotMatch(bridgeSource, /function normalizeIdentifier/);
  assert.doesNotMatch(bridgeSource, /function normalizeDatabaseKey/);
  assert.doesNotMatch(bridgeSource, /function createStorageTopology/);
  assert.doesNotMatch(bridgeSource, /PlusUser/);
  assert.doesNotMatch(bridgeSource, /PlusTenant/);
  assert.doesNotMatch(bridgeSource, /PlusAccountEntity/);
  assert.doesNotMatch(bridgeSource, /PlusVipUser/);
  assert.doesNotMatch(bridgeSource, /PlusOrganizationMember/);
  assert.doesNotMatch(bridgeSource, /PlusMemberRelations/);
});
test('craw-chat portal user-center runtime bridge aligns to the canonical runtime client and validation preflight standard', () => {
  const runtimeBridgePath = path.join(
    appRoot,
    'packages/craw-chat-portal-portal-api/src/userCenterRuntime.js',
  );
  const runtimeBridgeSource = read('packages/craw-chat-portal-portal-api/src/userCenterRuntime.js');
  const indexSource = read('packages/craw-chat-portal-portal-api/src/index.js');

  assert.equal(existsSync(runtimeBridgePath), true);

  for (const exportName of [
    'CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_SQLITE_PATH',
    'CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_DATABASE_KEY',
    'CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_MIGRATION_NAMESPACE',
    'CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_TABLE_PREFIX',
    'createCrawChatPortalCanonicalUserCenterConfig',
    'createCrawChatPortalUserCenterRuntimeClient',
  ]) {
    assert.match(runtimeBridgeSource, new RegExp(`export\\s+(?:const|function)\\s+${exportName}`));
  }

  assert.match(runtimeBridgeSource, /sdkwork-user-center-core-pc-react\/src\/index\.ts/);
  assert.match(runtimeBridgeSource, /from '\.\/userCenter\.js'/);
  assert.match(runtimeBridgeSource, /from '\.\/validation\.js'/);
  assert.match(runtimeBridgeSource, /createCrawChatPortalUserCenterValidationInteropContract/);
  assert.match(runtimeBridgeSource, /validationInteropContract/);
  assert.match(indexSource, /export \* from '\.\/userCenterRuntime\.js';/);
});

test('craw-chat portal packages restrict canonical appbase user-center imports to the local bridge module', () => {
  const directImportPattern =
    /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]identity[\\/]sdkwork-user-center-core-pc-react[\\/]src[\\/]index\.ts/u;
  const allowedImporters = new Set([
    normalizeTestPath(
      path.join(appRoot, 'packages', 'craw-chat-portal-portal-api', 'src', 'userCenter.js'),
    ),
    normalizeTestPath(
      path.join(appRoot, 'packages', 'craw-chat-portal-portal-api', 'src', 'userCenterRuntime.js'),
    ),
  ]);

  const offenders = collectSourceFiles(path.join(appRoot, 'packages'))
    .filter((filePath) => directImportPattern.test(readFileSync(filePath, 'utf8')))
    .map((filePath) => normalizeTestPath(filePath))
    .filter((filePath) => !allowedImporters.has(filePath));

  assert.deepEqual(offenders, []);
});

test('craw-chat portal runtime bridge materializes canonical runtime config and lets upstream preflight fail closed before requests', async () => {
  const runtimeBridge = await loadPortalUserCenterRuntimeBridge();
  const validationBridge = await loadPortalValidationBridge();

  const runtimeConfig = runtimeBridge.createCrawChatPortalCanonicalUserCenterConfig({
    mode: 'app-api-hub',
    provider: {
      baseUrl: 'https://api.sdkwork.local/craw-chat',
      kind: 'sdkwork-cloud-app-api',
      providerKey: 'craw-chat-remote',
    },
  });

  assert.equal(runtimeConfig.storage.dialect, 'sqlite');
  assert.equal(
    runtimeConfig.storage.sqlitePath,
    runtimeBridge.CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_SQLITE_PATH,
  );
  assert.equal(
    runtimeConfig.storageTopology.databaseKey,
    runtimeBridge.CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_DATABASE_KEY,
  );
  assert.equal(
    runtimeConfig.storageTopology.migrationNamespace,
    runtimeBridge.CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_MIGRATION_NAMESPACE,
  );
  assert.equal(
    runtimeConfig.storageTopology.tablePrefix,
    runtimeBridge.CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_TABLE_PREFIX,
  );

  let requestCount = 0;
  const client = runtimeBridge.createCrawChatPortalUserCenterRuntimeClient(
    {
      mode: 'app-api-hub',
      provider: {
        baseUrl: 'https://api.sdkwork.local/craw-chat',
        kind: 'sdkwork-cloud-app-api',
        providerKey: 'craw-chat-remote',
      },
    },
    {
      fetch: async () => {
        requestCount += 1;
        return runtimeResponseDouble({
          code: '2000',
          data: {
            ok: true,
          },
        });
      },
      validationInteropContract: {
        ...validationBridge.createCrawChatPortalUserCenterValidationInteropContract({
          mode: 'app-api-hub',
          provider: {
            baseUrl: 'https://api.sdkwork.local/craw-chat',
            kind: 'sdkwork-cloud-app-api',
            providerKey: 'craw-chat-remote',
          },
        }),
        tokenHeaders: {
          accessTokenHeaderName: 'Access-Token',
          authorizationHeaderName: 'Auth-Token',
          authorizationScheme: 'Bearer',
          refreshTokenHeaderName: 'Refresh-Token',
          sessionHeaderName: 'x-sdkwork-user-center-session-id',
        },
      },
    },
  );

  await assert.rejects(() => client.getProfile(), /tokenHeaders\.authorizationHeaderName/u);
  assert.equal(requestCount, 0);
});

test('craw-chat portal runtime bridge resolves browser runtime overrides into the canonical local/app-api/external user-center modes', async () => {
  const runtimeBridge = await loadPortalUserCenterRuntimeBridge();
  const previousWindow = globalThis.window;

  try {
    globalThis.window = {
      __CRAW_CHAT_PORTAL_USER_CENTER_MODE__: ' sdkwork-cloud-app-api ',
      __CRAW_CHAT_PORTAL_USER_CENTER_APP_API_BASE_URL__:
        ' https://api.sdkwork.local/craw-runtime/ ',
      __CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY__: ' Craw Runtime Remote ',
      __CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH__: ' /gateway/user-center ',
    };

    const upstreamConfig = runtimeBridge.createCrawChatPortalCanonicalUserCenterConfig();

    assert.equal(upstreamConfig.mode, 'app-api-hub');
    assert.equal(upstreamConfig.provider.kind, 'sdkwork-cloud-app-api');
    assert.equal(
      upstreamConfig.provider.baseUrl,
      'https://api.sdkwork.local/craw-runtime',
    );
    assert.equal(upstreamConfig.provider.providerKey, 'craw-runtime-remote');
    assert.equal(
      upstreamConfig.integration.builtinLocal.localApiBasePath,
      '/gateway/user-center',
    );

    globalThis.window = {
      __CRAW_CHAT_PORTAL_USER_CENTER_MODE__: ' external-user-center ',
      __CRAW_CHAT_PORTAL_USER_CENTER_EXTERNAL_BASE_URL__:
        ' https://identity.vendor.local/craw-runtime/ ',
      __CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY__: ' Craw Vendor SSO ',
      __CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH__: ' /external/user-center ',
    };

    const externalConfig = runtimeBridge.createCrawChatPortalCanonicalUserCenterConfig();

    assert.equal(externalConfig.mode, 'external-hub');
    assert.equal(externalConfig.provider.kind, 'external-user-center');
    assert.equal(
      externalConfig.provider.baseUrl,
      'https://identity.vendor.local/craw-runtime',
    );
    assert.equal(externalConfig.provider.providerKey, 'craw-vendor-sso');
    assert.equal(externalConfig.integration.activeKind, 'external-user-center');
    assert.equal(externalConfig.auth.mode, 'upstream-external-token-bridge');
    assert.equal(
      externalConfig.integration.builtinLocal.localApiBasePath,
      '/external/user-center',
    );

    globalThis.window = {
      __CRAW_CHAT_PORTAL_USER_CENTER_MODE__: ' builtin-local ',
      __CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY__: ' craw-local-window ',
      __CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH__: ' /window/user-center ',
    };

    const localConfig = runtimeBridge.createCrawChatPortalCanonicalUserCenterConfig();

    assert.equal(localConfig.mode, 'local-native');
    assert.equal(localConfig.provider.kind, 'builtin-local');
    assert.equal(localConfig.provider.providerKey, 'craw-local-window');
    assert.equal(
      localConfig.integration.builtinLocal.localApiBasePath,
      '/window/user-center',
    );
  } finally {
    globalThis.window = previousWindow;
  }
});

test('craw-chat portal validation bridge aligns to the canonical validation package and depends on the local user-center bridge', () => {
  const upstreamIndexPath = path.join(
    appbaseRoot,
    'packages/pc-react/identity/sdkwork-user-center-validation-pc-react/src/index.ts',
  );
  const bridgePath = path.join(
    appRoot,
    'packages/craw-chat-portal-portal-api/src/validation.js',
  );

  assert.equal(existsSync(upstreamIndexPath), true);
  assert.equal(existsSync(bridgePath), true);

  const upstreamIndexSource = readAppbase(
    'packages/pc-react/identity/sdkwork-user-center-validation-pc-react/src/index.ts',
  );
  const bridgeSource = read('packages/craw-chat-portal-portal-api/src/validation.js');

  assert.match(upstreamIndexSource, /@sdkwork\/user-center-validation-pc-react/);

  for (const exportName of [
    'CRAW_CHAT_PORTAL_USER_CENTER_VALIDATION_SOURCE_PACKAGE',
    'CRAW_CHAT_PORTAL_USER_CENTER_VALIDATION_PLUGIN_PACKAGES',
    'createCrawChatPortalUserCenterValidationInteropContract',
    'createCrawChatPortalUserCenterValidationPluginDefinition',
    'createCrawChatPortalUserCenterValidationPreflightReport',
    'createCrawChatPortalUserCenterValidationSnapshot',
    'assertCrawChatPortalUserCenterValidationPreflight',
    'resolveCrawChatPortalProtectedToken',
    'requireCrawChatPortalProtectedToken',
  ]) {
    assert.match(bridgeSource, new RegExp(`export\\s+(?:const|function)\\s+${exportName}`));
  }

  assert.match(bridgeSource, /sdkwork-user-center-validation-pc-react\/src\/index\.ts/);
  assert.match(bridgeSource, /from '\.\/userCenter\.js'/);
  assert.doesNotMatch(bridgeSource, /sdkwork-user-center-validation-pc-react\/src\/domain\//);
  assert.doesNotMatch(bridgeSource, /sdkwork-user-center-validation-pc-react\/src\/types\//);
});

test('craw-chat portal packages restrict canonical appbase validation imports to the local validation bridge module', () => {
  const directImportPattern =
    /sdkwork-appbase[\\/]packages[\\/]pc-react[\\/]identity[\\/]sdkwork-user-center-validation-pc-react[\\/]src[\\/]index\.ts/u;
  const allowedImporters = new Set([
    normalizeTestPath(
      path.join(appRoot, 'packages', 'craw-chat-portal-portal-api', 'src', 'validation.js'),
    ),
  ]);

  const offenders = collectSourceFiles(path.join(appRoot, 'packages'))
    .filter((filePath) => directImportPattern.test(readFileSync(filePath, 'utf8')))
    .map((filePath) => normalizeTestPath(filePath))
    .filter((filePath) => !allowedImporters.has(filePath));

  assert.deepEqual(offenders, []);
});

test('craw-chat portal user-center bridge materializes canonical integration and storage topology contracts', async () => {
  const bridge = await loadPortalUserCenterBridge();

  const localConfig = bridge.createCrawChatPortalUserCenterConfig();

  assert.equal(localConfig.auth.mode, 'auth-access-token');
  assert.equal(localConfig.auth.validationStrategy, 'auth-access-token');
  assert.equal(localConfig.auth.secretResolution.resolverKind, 'local-static');
  assert.equal(localConfig.auth.secretResolution.scope, 'organization-preferred');
  assert.equal(localConfig.auth.tokenHeaders.authorizationHeaderName, 'Authorization');
  assert.equal(localConfig.auth.tokenHeaders.accessTokenHeaderName, 'Access-Token');
  assert.equal(localConfig.auth.handshake.enabled, false);
  assert.equal(localConfig.auth.handshake.freshnessWindowMs, 30000);
  assert.equal(localConfig.integration.activeKind, 'builtin-local');
  assert.equal(localConfig.integration.builtinLocal.authMode, 'auth-access-token');
  assert.equal(localConfig.integration.builtinLocal.enabled, true);
  assert.equal(localConfig.integration.builtinLocal.handshakeEnabled, false);
  assert.equal(localConfig.integration.builtinLocal.localApiBasePath, '/api/app/v1/user-center');
  assert.equal(localConfig.integration.builtinLocal.secretResolverKind, 'local-static');
  assert.equal(localConfig.integration.builtinLocal.sessionTransport, 'header');
  assert.equal(localConfig.integration.builtinLocal.userSystemScope, 'application');
  assert.equal(localConfig.integration.builtinLocal.validationStrategy, 'auth-access-token');
  assert.equal(localConfig.integration.externalAppApi.enabled, false);
  assert.equal(localConfig.integration.externalAppApi.authMode, 'upstream-app-api-token-bridge');
  assert.equal(localConfig.integration.externalAppApi.handshakeEnabled, false);
  assert.equal(localConfig.integration.externalAppApi.providerKey, 'craw-chat-portal-app-api');
  assert.equal(localConfig.integration.externalAppApi.secretResolverKind, 'upstream-secret-bridge');
  assert.equal(localConfig.integration.externalAppApi.validationStrategy, 'auth-access-token');
  assert.equal(localConfig.storageTopology.databaseKey, 'craw-chat-portal-user-center');
  assert.equal(localConfig.storageTopology.migrationNamespace, 'craw-chat-portal.user-center');
  assert.equal(localConfig.storageTopology.tablePrefix, 'uc_');
  assert.deepEqual(
    localConfig.storageTopology.entityBindings.map((binding) => binding.standardEntityName),
    Array.from(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STANDARD_ENTITIES),
  );
  assert.deepEqual(
    localConfig.storageTopology.entityBindings.map((binding) => binding.tableName),
    ['uc_user', 'uc_tenant', 'uc_account', 'uc_vip_user', 'uc_organization_member', 'uc_member_relations'],
  );

  const remoteConfig = bridge.createCrawChatPortalUserCenterConfig({
    mode: 'app-api-hub',
    provider: {
      baseUrl: ' https://app-api.example.com/craw-tenant/ ',
      kind: 'sdkwork-cloud-app-api',
      providerKey: ' Craw Tenant App API ',
    },
  });

  assert.equal(remoteConfig.auth.mode, 'upstream-app-api-token-bridge');
  assert.equal(remoteConfig.auth.handshake.enabled, true);
  assert.equal(remoteConfig.auth.handshake.freshnessWindowMs, 30000);
  assert.deepEqual(remoteConfig.auth.handshake.staticHeaders, {
    'x-sdkwork-app-id': 'craw-chat-portal',
    'x-sdkwork-user-center-handshake-mode': 'provider-shared-secret',
    'x-sdkwork-user-center-provider-key': 'craw-tenant-app-api',
  });
  assert.equal(remoteConfig.auth.secretResolution.resolverKind, 'upstream-secret-bridge');
  assert.equal(remoteConfig.integration.activeKind, 'sdkwork-cloud-app-api');
  assert.equal(remoteConfig.integration.externalAppApi.authMode, 'upstream-app-api-token-bridge');
  assert.equal(remoteConfig.integration.externalAppApi.enabled, true);
  assert.equal(remoteConfig.integration.externalAppApi.handshakeEnabled, true);
  assert.equal(remoteConfig.integration.externalAppApi.providerKey, 'craw-tenant-app-api');
  assert.equal(remoteConfig.integration.externalAppApi.upstreamBaseUrl, 'https://app-api.example.com/craw-tenant');
  assert.equal(remoteConfig.storageTopology.databaseKey, 'craw-chat-portal-user-center');
  assert.deepEqual(
    bridge.createCrawChatPortalUserCenterHandshakeVerificationContext({
      config: remoteConfig,
      headers: {
        'x-sdkwork-app-id': 'craw-chat-portal',
        'x-sdkwork-user-center-handshake-mode': 'provider-shared-secret',
        'x-sdkwork-user-center-provider-key': 'craw-tenant-app-api',
        'x-sdkwork-user-center-secret-id': 'secret-401',
        'x-sdkwork-user-center-signature': 'signature-401',
        'x-sdkwork-user-center-signed-at': '2026-04-21T10:10:00Z',
      },
      method: 'GET',
      now: '2026-04-21T10:10:20Z',
      path: remoteConfig.localApi.profile,
    }).handshake,
    {
      appId: 'craw-chat-portal',
      handshakeMode: 'provider-shared-secret',
      providerKey: 'craw-tenant-app-api',
      secretId: 'secret-401',
      signature: 'signature-401',
      signedAt: '2026-04-21T10:10:00Z',
    },
  );

  const plugin = bridge.createCrawChatPortalUserCenterPluginDefinition();

  assert.deepEqual(plugin.capabilities, ['auth']);
  assert.equal(plugin.bridgeConfig.namespace, 'craw-chat-portal');
  assert.equal(plugin.manifests.auth.loginRoutePath, '/login');
  assert.equal(plugin.manifests.auth.registerRoutePath, undefined);
  assert.equal(plugin.manifests.auth.forgotPasswordRoutePath, undefined);
  assert.equal(plugin.manifests.auth.oauthCallbackRoutePattern, undefined);
  assert.equal(plugin.manifests.auth.qrRoutePath, undefined);
  assert.equal(plugin.manifests.user, undefined);
  assert.equal(plugin.manifests.vip, undefined);

  assert.equal(plugin.portalDeployment.activeKind, 'builtin-local');
  assert.equal(plugin.portalDeployment.builtinLocal.standard.handshake.freshnessWindowMs, 30000);
  assert.deepEqual(
    plugin.portalDeployment.builtinLocal.artifacts.map((artifact) => artifact.fileName),
    [
      'craw-chat-portal.builtin-local.runtime.env.example',
      'craw-chat-portal.builtin-local.gateway.env.example',
    ],
  );
  assert.deepEqual(
    plugin.portalDeployment.builtinLocal.runtimeEnvArtifact.variables.map((entry) => entry.envName),
    [
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_MODE',
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY',
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
      ...createStandardUserCenterAuthEnvNames('VITE_CRAW_CHAT_PORTAL_USER_CENTER_'),
    ],
  );
  assert.deepEqual(
    plugin.portalDeployment.builtinLocal.gatewayEnvArtifact.variables.map((entry) => entry.envName),
    [
      'CRAW_CHAT_PORTAL_USER_CENTER_MODE',
      'CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY',
      'CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
      ...createStandardUserCenterAuthEnvNames('CRAW_CHAT_PORTAL_USER_CENTER_'),
      'CRAW_CHAT_PORTAL_USER_CENTER_SQLITE_PATH',
      'CRAW_CHAT_PORTAL_USER_CENTER_DATABASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_SCHEMA_NAME',
      'CRAW_CHAT_PORTAL_USER_CENTER_TABLE_PREFIX',
    ],
  );
  assert.equal(plugin.portalDeployment.builtinLocal.runtimeEnvArtifact.audience, 'application-runtime');
  assert.equal(plugin.portalDeployment.builtinLocal.runtimeEnvArtifact.format, 'dotenv');
  assert.equal(
    plugin.portalDeployment.builtinLocal.gatewayEnvArtifact.audience,
    'gateway-runtime',
  );
  assert.equal(plugin.portalDeployment.builtinLocal.gatewayEnvArtifact.format, 'dotenv');
  assert.match(
    plugin.portalDeployment.builtinLocal.runtimeEnvArtifact.content,
    /VITE_CRAW_CHAT_PORTAL_USER_CENTER_MODE=builtin-local/,
  );
  assert.doesNotMatch(
    plugin.portalDeployment.builtinLocal.runtimeEnvArtifact.content,
    /SHARED_SECRET/,
  );
  assert.match(
    plugin.portalDeployment.builtinLocal.gatewayEnvArtifact.content,
    /CRAW_CHAT_PORTAL_USER_CENTER_SQLITE_PATH=\.\/data\/user-center\.db/,
  );

  const remotePlugin = bridge.createCrawChatPortalUserCenterPluginDefinition({
    mode: 'app-api-hub',
    provider: {
      baseUrl: 'https://app-api.example.com/craw-tenant',
      kind: 'sdkwork-cloud-app-api',
      providerKey: 'craw-tenant-app-api',
    },
  });

  assert.equal(remotePlugin.portalDeployment.activeKind, 'sdkwork-cloud-app-api');
  assert.equal(remotePlugin.portalDeployment.externalAppApi.providerKey, 'craw-tenant-app-api');
  assert.equal(remotePlugin.portalDeployment.externalAppApi.handshakeEnabled, true);
  assert.equal(
    remotePlugin.portalDeployment.externalAppApi.standard.handshake.freshnessWindowMs,
    30000,
  );
  assert.deepEqual(
    remotePlugin.portalDeployment.externalAppApi.artifacts.map((artifact) => artifact.fileName),
    [
      'craw-chat-portal.sdkwork-cloud-app-api.runtime.env.example',
      'craw-chat-portal.sdkwork-cloud-app-api.gateway.env.example',
    ],
  );
  assert.deepEqual(
    remotePlugin.portalDeployment.externalAppApi.runtimeEnvArtifact.variables.map((entry) => entry.envName),
    [
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_MODE',
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY',
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
      ...createStandardUserCenterAuthEnvNames('VITE_CRAW_CHAT_PORTAL_USER_CENTER_'),
    ],
  );
  assert.deepEqual(
    remotePlugin.portalDeployment.externalAppApi.gatewayEnvArtifact.variables.map((entry) => entry.envName),
    [
      'CRAW_CHAT_PORTAL_USER_CENTER_MODE',
      'CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY',
      'CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
      ...createStandardUserCenterAuthEnvNames('CRAW_CHAT_PORTAL_USER_CENTER_'),
      'CRAW_CHAT_PORTAL_USER_CENTER_APP_API_BASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_APP_ID',
      'CRAW_CHAT_PORTAL_USER_CENTER_SECRET_ID',
      'CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET',
      'CRAW_CHAT_PORTAL_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS',
      'CRAW_CHAT_PORTAL_USER_CENTER_SQLITE_PATH',
      'CRAW_CHAT_PORTAL_USER_CENTER_DATABASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_SCHEMA_NAME',
      'CRAW_CHAT_PORTAL_USER_CENTER_TABLE_PREFIX',
    ],
  );
  assert.deepEqual(
    remotePlugin.portalDeployment.externalAppApi.gatewayEnvArtifact.variables
      .filter((entry) => entry.required)
      .map((entry) => entry.envName),
    [
      'CRAW_CHAT_PORTAL_USER_CENTER_APP_API_BASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_SECRET_ID',
      'CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET',
    ],
  );
  assert.equal(
    remotePlugin.portalDeployment.externalAppApi.runtimeEnvArtifact.variables.some(
      (entry) => entry.envName === 'VITE_CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET',
    ),
    false,
  );
  assert.match(
    remotePlugin.portalDeployment.externalAppApi.runtimeEnvArtifact.content,
    /VITE_CRAW_CHAT_PORTAL_USER_CENTER_MODE=sdkwork-cloud-app-api/,
  );
  assert.doesNotMatch(
    remotePlugin.portalDeployment.externalAppApi.runtimeEnvArtifact.content,
    /SHARED_SECRET/,
  );
  assert.match(
    remotePlugin.portalDeployment.externalAppApi.gatewayEnvArtifact.content,
    /CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET=<required-secret>/,
  );
  assert.match(
    remotePlugin.portalDeployment.externalAppApi.gatewayEnvArtifact.content,
    /CRAW_CHAT_PORTAL_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS=30000/,
  );
  assert.match(
    remotePlugin.portalDeployment.externalAppApi.gatewayEnvArtifact.content,
    /CRAW_CHAT_PORTAL_USER_CENTER_APP_API_BASE_URL=https:\/\/app-api\.example\.com\/craw-tenant/,
  );

  const externalPlugin = bridge.createCrawChatPortalUserCenterPluginDefinition({
    mode: 'external-hub',
    provider: {
      baseUrl: 'https://identity.vendor.local/craw-tenant',
      kind: 'external-user-center',
      providerKey: 'craw-tenant-sso',
    },
  });

  assert.equal(externalPlugin.portalDeployment.activeKind, 'external-user-center');
  assert.equal(externalPlugin.portalDeployment.externalUserCenter?.providerKey, 'craw-tenant-sso');
  assert.equal(externalPlugin.portalDeployment.externalUserCenter?.handshakeEnabled, true);
  assert.equal(
    externalPlugin.portalDeployment.externalUserCenter?.standard.handshake.freshnessWindowMs,
    30000,
  );
  assert.deepEqual(
    externalPlugin.portalDeployment.externalUserCenter?.artifacts.map((artifact) => artifact.fileName),
    [
      'craw-chat-portal.external-user-center.runtime.env.example',
      'craw-chat-portal.external-user-center.gateway.env.example',
    ],
  );
  assert.deepEqual(
    externalPlugin.portalDeployment.externalUserCenter?.runtimeEnvArtifact.variables.map((entry) => entry.envName),
    [
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_MODE',
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY',
      'VITE_CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
      ...createStandardUserCenterAuthEnvNames('VITE_CRAW_CHAT_PORTAL_USER_CENTER_'),
    ],
  );
  assert.deepEqual(
    externalPlugin.portalDeployment.externalUserCenter?.gatewayEnvArtifact.variables.map((entry) => entry.envName),
    [
      'CRAW_CHAT_PORTAL_USER_CENTER_MODE',
      'CRAW_CHAT_PORTAL_USER_CENTER_PROVIDER_KEY',
      'CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH',
      ...createStandardUserCenterAuthEnvNames('CRAW_CHAT_PORTAL_USER_CENTER_'),
      'CRAW_CHAT_PORTAL_USER_CENTER_EXTERNAL_BASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_APP_ID',
      'CRAW_CHAT_PORTAL_USER_CENTER_SECRET_ID',
      'CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET',
      'CRAW_CHAT_PORTAL_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS',
      'CRAW_CHAT_PORTAL_USER_CENTER_SQLITE_PATH',
      'CRAW_CHAT_PORTAL_USER_CENTER_DATABASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_SCHEMA_NAME',
      'CRAW_CHAT_PORTAL_USER_CENTER_TABLE_PREFIX',
    ],
  );
  assert.deepEqual(
    externalPlugin.portalDeployment.externalUserCenter?.gatewayEnvArtifact.variables
      .filter((entry) => entry.required)
      .map((entry) => entry.envName),
    [
      'CRAW_CHAT_PORTAL_USER_CENTER_EXTERNAL_BASE_URL',
      'CRAW_CHAT_PORTAL_USER_CENTER_SECRET_ID',
      'CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET',
    ],
  );
  assert.equal(
    externalPlugin.portalDeployment.externalUserCenter?.runtimeEnvArtifact.variables.some(
      (entry) => entry.envName === 'VITE_CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET',
    ),
    false,
  );
  assert.match(
    externalPlugin.portalDeployment.externalUserCenter?.runtimeEnvArtifact.content ?? '',
    /VITE_CRAW_CHAT_PORTAL_USER_CENTER_MODE=external-user-center/,
  );
  assert.doesNotMatch(
    externalPlugin.portalDeployment.externalUserCenter?.runtimeEnvArtifact.content ?? '',
    /SHARED_SECRET/,
  );
  assert.match(
    externalPlugin.portalDeployment.externalUserCenter?.gatewayEnvArtifact.content ?? '',
    /CRAW_CHAT_PORTAL_USER_CENTER_SHARED_SECRET=<required-secret>/,
  );
  assert.match(
    externalPlugin.portalDeployment.externalUserCenter?.gatewayEnvArtifact.content ?? '',
    /CRAW_CHAT_PORTAL_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS=30000/,
  );
  assert.match(
    externalPlugin.portalDeployment.externalUserCenter?.gatewayEnvArtifact.content ?? '',
    /CRAW_CHAT_PORTAL_USER_CENTER_EXTERNAL_BASE_URL=https:\/\/identity\.vendor\.local\/craw-tenant/,
  );
});

test('craw-chat portal-api session storage uses the shared user-center bridge instead of a hardcoded token key', () => {
  const portalApi = read('packages/craw-chat-portal-portal-api/src/index.js');

  assert.match(portalApi, /from '\.\/userCenter\.js'/);
  assert.match(portalApi, /export \* from '\.\/userCenter\.js';/);
  assert.match(portalApi, /createCrawChatPortalUserCenterSessionStore/);
  assert.match(portalApi, /createCrawChatPortalUserCenterTokenStore/);
  assert.doesNotMatch(portalApi, /sdkwork-user-center-core-pc-react\/src\/index\.ts/);
  assert.doesNotMatch(portalApi, /sdkwork-user-center-core-pc-react\/src\/domain\//);
  assert.match(portalApi, /CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN/);
  assert.match(portalApi, /createUserCenterSessionStore/);
  assert.match(portalApi, /createUserCenterTokenStore/);
  assert.match(portalApi, /readPortalTokenBundle/);
  assert.match(portalApi, /persistPortalTokenBundle/);
  assert.match(portalApi, /clearPortalTokenBundle/);
  assert.doesNotMatch(portalApi, /craw-chat-portal\.session\.v1/);
  assert.doesNotMatch(portalApi, /sessionTokenKey/);
});

test('craw-chat validation bridge materializes canonical validation policy and portal-api delegates protected token resolution to it', async () => {
  const bridge = await loadPortalValidationBridge();
  const portalApi = read('packages/craw-chat-portal-portal-api/src/index.js');

  const validation = bridge.createCrawChatPortalUserCenterValidationPluginDefinition({
    mode: 'app-api-hub',
    provider: {
      baseUrl: 'https://app-api.example.com/craw-tenant',
      kind: 'sdkwork-cloud-app-api',
      providerKey: 'craw-tenant-app-api',
    },
  });

  assert.equal(validation.capability, 'user-center-validation');
  assert.equal(validation.dependency.capability, 'user-center');
  assert.equal(validation.dependency.namespace, 'craw-chat-portal');
  assert.equal(validation.dependency.providerKey, 'craw-tenant-app-api');
  assert.equal(validation.validation.authMode, 'upstream-app-api-token-bridge');
  assert.equal(validation.validation.handshake.enabled, true);
  assert.equal(validation.validation.handshake.freshnessWindowMs, 30000);
  assert.deepEqual(validation.validation.secretResolution, {
    organizationClaimKey: 'organizationId',
    resolverKind: 'upstream-secret-bridge',
    scope: 'organization-preferred',
    tenantClaimKey: 'tenantId',
  });
  assert.deepEqual(validation.validation.governedHeaderNames, [
    'Authorization',
    'Access-Token',
    'Refresh-Token',
    'x-sdkwork-user-center-session-id',
    'x-sdkwork-app-id',
    'x-sdkwork-user-center-handshake-mode',
    'x-sdkwork-user-center-provider-key',
    'x-sdkwork-user-center-secret-id',
    'x-sdkwork-user-center-signature',
    'x-sdkwork-user-center-signed-at',
  ]);
  assert.equal(validation.manifests.validation.capability, 'validation');
  assert.equal(validation.manifests.validation.dependencyCapability, 'user-center');

  assert.equal(
    bridge.resolveCrawChatPortalProtectedToken({
      providedToken: 'session-token',
      tokenBundle: {
        authToken: 'auth-token',
        sessionToken: 'session-token',
      },
    }),
    'auth-token',
  );
  assert.equal(
    bridge.requireCrawChatPortalProtectedToken({
      tokenBundle: {
        accessToken: 'access-token',
      },
    }),
    'access-token',
  );

  const interopContract = bridge.createCrawChatPortalUserCenterValidationInteropContract({
    mode: 'app-api-hub',
    provider: {
      baseUrl: 'https://app-api.example.com/craw-tenant',
      kind: 'sdkwork-cloud-app-api',
      providerKey: 'craw-tenant-app-api',
    },
  });

  const preflight = bridge.createCrawChatPortalUserCenterValidationPreflightReport({
    mode: 'app-api-hub',
    peerContract: interopContract,
    provider: {
      baseUrl: 'https://app-api.example.com/craw-tenant',
      kind: 'sdkwork-cloud-app-api',
      providerKey: 'craw-tenant-app-api',
    },
  });

  assert.deepEqual(preflight, {
    compatible: true,
    diff: {
      compatible: true,
      mismatches: [],
    },
    localContract: interopContract,
    peerContract: interopContract,
  });
  assert.deepEqual(
    bridge.assertCrawChatPortalUserCenterValidationPreflight({
      mode: 'app-api-hub',
      peerContract: interopContract,
      provider: {
        baseUrl: 'https://app-api.example.com/craw-tenant',
        kind: 'sdkwork-cloud-app-api',
        providerKey: 'craw-tenant-app-api',
      },
    }),
    preflight,
  );

  const mismatchedPeerContract = {
    ...interopContract,
    handshake: {
      ...interopContract.handshake,
      freshnessWindowMs: 15000,
    },
  };

  assert.deepEqual(
    bridge.createCrawChatPortalUserCenterValidationPreflightReport({
      mode: 'app-api-hub',
      peerContract: mismatchedPeerContract,
      provider: {
        baseUrl: 'https://app-api.example.com/craw-tenant',
        kind: 'sdkwork-cloud-app-api',
        providerKey: 'craw-tenant-app-api',
      },
    }).diff,
    {
      compatible: false,
      mismatches: [
        {
          actual: 30000,
          expected: 15000,
          fieldPath: 'handshake.freshnessWindowMs',
        },
      ],
    },
  );
  assert.throws(
    () => bridge.assertCrawChatPortalUserCenterValidationPreflight({
      mode: 'app-api-hub',
      peerContract: mismatchedPeerContract,
      provider: {
        baseUrl: 'https://app-api.example.com/craw-tenant',
        kind: 'sdkwork-cloud-app-api',
        providerKey: 'craw-tenant-app-api',
      },
    }),
    /handshake\.freshnessWindowMs/u,
  );

  assert.match(portalApi, /resolveCrawChatPortalProtectedToken/);
  assert.doesNotMatch(portalApi, /function resolveProtectedPortalToken/);
  assert.doesNotMatch(portalApi, /function resolveStoredPortalToken/);
});

test('craw-chat portal-api session helpers migrate legacy local storage tokens into session storage', async () => {
  const portalApi = await loadPortalApi();
  const bridge = await loadPortalUserCenterBridge();
  const previousSessionStorage = globalThis.sessionStorage;
  const previousLocalStorage = globalThis.localStorage;
  const sessionStorage = storageDouble();
  const localStorage = storageDouble();

  localStorage.setItem(
    bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey,
    'legacy-session-token',
  );

  globalThis.sessionStorage = sessionStorage;
  globalThis.localStorage = localStorage;

  try {
    assert.equal(portalApi.readPortalSessionToken(), 'legacy-session-token');
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      'legacy-session-token',
    );
    assert.equal(
      localStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      null,
    );

    portalApi.persistPortalSessionToken('fresh-session-token');

    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      'fresh-session-token',
    );

    portalApi.clearPortalSessionToken();

    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      null,
    );
    assert.equal(
      localStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      null,
    );
  } finally {
    globalThis.sessionStorage = previousSessionStorage;
    globalThis.localStorage = previousLocalStorage;
  }
});

test('craw-chat portal-api token bundle helpers persist the canonical auth tokens without hardcoded storage keys', async () => {
  const portalApi = await loadPortalApi();
  const bridge = await loadPortalUserCenterBridge();
  const previousSessionStorage = globalThis.sessionStorage;
  const previousLocalStorage = globalThis.localStorage;
  const sessionStorage = storageDouble();
  const localStorage = storageDouble();

  globalThis.sessionStorage = sessionStorage;
  globalThis.localStorage = localStorage;

  try {
    portalApi.persistPortalTokenBundle({
      accessToken: 'tenant-demo-access',
      authToken: 'tenant-demo-auth',
      refreshToken: 'tenant-demo-refresh',
      sessionToken: 'tenant-demo-session',
      tokenType: 'Bearer',
    });

    assert.deepEqual(portalApi.readPortalTokenBundle(), {
      accessToken: 'tenant-demo-access',
      authToken: 'tenant-demo-auth',
      refreshToken: 'tenant-demo-refresh',
      sessionToken: 'tenant-demo-session',
      tokenType: 'Bearer',
    });
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.accessTokenKey),
      'tenant-demo-access',
    );
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.authTokenKey),
      'tenant-demo-auth',
    );
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.refreshTokenKey),
      'tenant-demo-refresh',
    );
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      'tenant-demo-session',
    );

    portalApi.clearPortalTokenBundle();

    assert.deepEqual(portalApi.readPortalTokenBundle(), {});
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.accessTokenKey),
      null,
    );
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.authTokenKey),
      null,
    );
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.refreshTokenKey),
      null,
    );
    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      null,
    );
  } finally {
    globalThis.sessionStorage = previousSessionStorage;
    globalThis.localStorage = previousLocalStorage;
  }
});

test('craw-chat portal-api session helpers reject malformed tokens and fail closed when browser storage is unavailable', async () => {
  const portalApi = await loadPortalApi();
  const bridge = await loadPortalUserCenterBridge();
  const previousSessionStorage = globalThis.sessionStorage;
  const previousLocalStorage = globalThis.localStorage;

  globalThis.sessionStorage = throwingStorageDouble();
  globalThis.localStorage = throwingStorageDouble();

  try {
    assert.equal(portalApi.readPortalSessionToken(), null);
    assert.doesNotThrow(() => portalApi.persistPortalSessionToken('tenant-demo-session'));
    assert.doesNotThrow(() => portalApi.clearPortalSessionToken());
  } finally {
    globalThis.sessionStorage = previousSessionStorage;
    globalThis.localStorage = previousLocalStorage;
  }

  const sessionStorage = storageDouble();
  const localStorage = storageDouble();
  globalThis.sessionStorage = sessionStorage;
  globalThis.localStorage = localStorage;

  try {
    assert.throws(
      () => portalApi.persistPortalSessionToken(''),
      {
        name: 'TypeError',
      },
    );

    assert.equal(
      sessionStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      null,
    );
    assert.equal(
      localStorage.getItem(bridge.CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN.sessionTokenKey),
      null,
    );
  } finally {
    globalThis.sessionStorage = previousSessionStorage;
    globalThis.localStorage = previousLocalStorage;
  }
});

test('craw-chat portal server user-center bridge exposes canonical server and server-validation plugins', async () => {
  const bridge = await loadPortalUserCenterBridge();
  const validationBridge = await loadPortalValidationBridge();
  const bridgeSource = read('packages/craw-chat-portal-portal-api/src/userCenter.js');
  const validationSource = read('packages/craw-chat-portal-portal-api/src/validation.js');

  assert.match(bridgeSource, /createUserCenterServerPluginDefinition/u);
  assert.match(validationSource, /createUserCenterServerValidationPluginDefinition/u);

  const localServerPlugin = bridge.createCrawChatPortalUserCenterServerPluginDefinition();
  assert.equal(localServerPlugin.capability, 'user-center-server');
  assert.equal(localServerPlugin.server.authority.activeIntegrationKind, 'builtin-local');
  assert.equal(localServerPlugin.server.manifests.server.host, 'server');
  assert.equal(localServerPlugin.server.authority.localAuthority.enabled, true);

  const externalServerPlugin = bridge.createCrawChatPortalUserCenterServerPluginDefinition({
    mode: 'external-hub',
    provider: {
      baseUrl: 'https://identity.vendor.local/craw',
      kind: 'external-user-center',
      providerKey: 'craw-sso',
    },
  });
  assert.equal(
    externalServerPlugin.server.authority.activeIntegrationKind,
    'external-user-center',
  );
  assert.equal(
    externalServerPlugin.server.deployment.externalUserCenter.providerKey,
    'craw-sso',
  );

  const serverValidation =
    validationBridge.createCrawChatPortalUserCenterServerValidationPluginDefinition({
      mode: 'app-api-hub',
      provider: {
        baseUrl: 'https://app-api.sdkwork.local/craw',
        kind: 'sdkwork-cloud-app-api',
        providerKey: 'craw-app-api',
      },
    });
  assert.equal(serverValidation.capability, 'user-center-server-validation');
  assert.equal(serverValidation.dependency.capability, 'user-center-server');
  assert.equal(serverValidation.middleware.handshake.required, true);
});
