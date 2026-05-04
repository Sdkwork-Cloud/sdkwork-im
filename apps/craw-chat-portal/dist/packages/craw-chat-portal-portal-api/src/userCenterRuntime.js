import {
  createDefaultUserCenterConfig,
  createUserCenterRuntimeClient,
  createUserCenterSessionStore,
  createUserCenterTokenStore,
  resolveUserCenterRuntimeConfigInput,
} from '../../../../../../sdkwork-appbase/packages/pc-react/identity/sdkwork-user-center-core-pc-react/src/index.ts';
import {
  CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_PREFIX,
  CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_PREFIX,
  createCrawChatPortalUserCenterConfig,
} from './userCenter.js';
import { createCrawChatPortalUserCenterValidationInteropContract } from './validation.js';

export {
  createUserCenterRuntimeClient,
  createUserCenterSessionStore,
  createUserCenterTokenStore,
};

export const CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_SQLITE_PATH =
  'app://craw-chat-portal/user-center.db';
export const CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_DATABASE_KEY =
  'craw-chat-portal-user-center';
export const CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_MIGRATION_NAMESPACE =
  'craw-chat-portal.user-center';
export const CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_TABLE_PREFIX = 'cc_uc_';

function resolveCrawChatPortalRuntimeWindow() {
  if (typeof window === 'undefined') {
    return undefined;
  }

  return /** @type {Record<string, unknown>} */ (window);
}

function resolveCrawChatPortalRuntimeEnv() {
  return /** @type {Record<string, unknown> | undefined} */ (import.meta.env ?? undefined);
}

function resolveCrawChatPortalRuntimeConfigOptions(options) {
  return Object.freeze(resolveUserCenterRuntimeConfigInput(options, {
    env: resolveCrawChatPortalRuntimeEnv(),
    envPrefix: CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_PREFIX,
    window: resolveCrawChatPortalRuntimeWindow(),
    windowPrefix: CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_PREFIX,
  }));
}

function createCrawChatPortalCanonicalStorageTopology(runtimeConfig) {
  return {
    ...runtimeConfig.storageTopology,
    databaseKey: CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_DATABASE_KEY,
    migrationNamespace: CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_MIGRATION_NAMESPACE,
    tablePrefix: CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_TABLE_PREFIX,
  };
}

function createDefaultCrawChatPortalValidationInteropContract(runtimeConfig) {
  return createCrawChatPortalUserCenterValidationInteropContract({
    auth: runtimeConfig.auth,
    localApiBasePath: runtimeConfig.integration.builtinLocal.localApiBasePath,
    mode: runtimeConfig.mode,
    provider: runtimeConfig.provider,
    routes: runtimeConfig.routes,
    storageTopology: runtimeConfig.storageTopology,
  });
}

export function createCrawChatPortalCanonicalUserCenterConfig(options = {}) {
  const resolvedOptions = resolveCrawChatPortalRuntimeConfigOptions(options);
  const bridgeConfig = createCrawChatPortalUserCenterConfig(resolvedOptions);

  return Object.freeze(
    createDefaultUserCenterConfig({
      auth: bridgeConfig.auth,
      localApiBasePath: bridgeConfig.integration.builtinLocal.localApiBasePath,
      mode: bridgeConfig.mode,
      namespace: bridgeConfig.namespace,
      provider: bridgeConfig.provider,
      routes: bridgeConfig.routes,
      storage: {
        dialect: 'sqlite',
        sqlitePath: CRAW_CHAT_PORTAL_CANONICAL_USER_CENTER_SQLITE_PATH,
      },
      storageTopology: createCrawChatPortalCanonicalStorageTopology(bridgeConfig),
    }),
  );
}

export function createCrawChatPortalUserCenterRuntimeClient(configOptions = {}, options = {}) {
  const runtimeConfig = createCrawChatPortalCanonicalUserCenterConfig(configOptions);

  return createUserCenterRuntimeClient(runtimeConfig, {
    ...options,
    ...(options.validationInteropContract || options.resolveValidationInteropContract
      ? {}
      : {
          validationInteropContract:
            createDefaultCrawChatPortalValidationInteropContract(runtimeConfig),
        }),
  });
}
