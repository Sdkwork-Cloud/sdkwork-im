import {
  USER_CENTER_SESSION_HEADER_NAME,
  USER_CENTER_SOURCE_PACKAGE_NAME,
  createUserCenterDeploymentEnvArtifact,
  createUserCenterHandshakeSigningMessage,
  createUserCenterHandshakeVerificationContext,
  createUserCenterLocalApiRoutes,
  createUserCenterBridgeConfig,
  createUserCenterPluginDefinition,
  createUserCenterServerPluginDefinition,
  createUserCenterSignedHandshakeHeaders,
  createUserCenterSessionStore,
  createUserCenterStoragePlan,
  createUserCenterTokenStore,
  mapUserCenterDeploymentVariablesToEnvironmentVariables,
  mergeUserCenterDeploymentVariables,
  selectUserCenterDeploymentVariables,
  USER_CENTER_STANDARD_ENTITY_NAMES,
} from '../../../../../../sdkwork-appbase/packages/pc-react/identity/sdkwork-user-center-core-pc-react/src/index.ts';

export const CRAW_CHAT_PORTAL_USER_CENTER_SOURCE_PACKAGE = USER_CENTER_SOURCE_PACKAGE_NAME;
export const CRAW_CHAT_PORTAL_USER_CENTER_NAMESPACE = 'craw-chat-portal';
export const CRAW_CHAT_PORTAL_USER_CENTER_SESSION_HEADER_NAME = USER_CENTER_SESSION_HEADER_NAME;
export const CRAW_CHAT_PORTAL_USER_CENTER_STANDARD_ENTITIES = Object.freeze(
  Array.from(USER_CENTER_STANDARD_ENTITY_NAMES),
);
export const CRAW_CHAT_PORTAL_USER_CENTER_PLUGIN_PACKAGES = Object.freeze([
  'craw-chat-portal-auth',
]);
export const CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN = Object.freeze(
  createUserCenterStoragePlan(CRAW_CHAT_PORTAL_USER_CENTER_NAMESPACE),
);
export const CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH = '/api/app/v1/user-center';
export const CRAW_CHAT_PORTAL_USER_CENTER_ROUTES = Object.freeze({
  authBasePath: '/login',
  userRoutePath: '/user',
  vipRoutePath: '/vip',
});
export const CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API = Object.freeze(
  createUserCenterLocalApiRoutes(CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH),
);
export const CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_PREFIX = 'VITE_CRAW_CHAT_PORTAL_USER_CENTER_';
export const CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_PREFIX = 'CRAW_CHAT_PORTAL_USER_CENTER_';
export const CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_ARTIFACT_BASENAME = 'runtime.env.example';
export const CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_ARTIFACT_BASENAME = 'gateway.env.example';

function createCrawChatPortalUserCenterBasePluginArtifacts(options = {}) {
  const bridgeConfig = createCrawChatPortalUserCenterConfig(options);
  const plugin = createUserCenterPluginDefinition({
    auth: options.auth,
    capabilities: options.capabilities ?? ['auth'],
    host: options.host,
    localApiBasePath:
      options.localApiBasePath ?? CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH,
    mode: options.mode,
    namespace: CRAW_CHAT_PORTAL_USER_CENTER_NAMESPACE,
    packageNames: options.packageNames ?? [...CRAW_CHAT_PORTAL_USER_CENTER_PLUGIN_PACKAGES],
    provider: options.provider,
    routes: {
      authBasePath: '',
      userRoutePath: options.routes?.userRoutePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.userRoutePath,
      vipRoutePath: options.routes?.vipRoutePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.vipRoutePath,
    },
    storageTopology: options.storageTopology,
    theme: options.theme,
    title: options.title ?? 'Craw Chat User Center',
  });

  return {
    bridgeConfig,
    plugin,
  };
}

function mapCrawChatPortalUserCenterEnvironmentVariables(variables, prefix) {
  return mapUserCenterDeploymentVariablesToEnvironmentVariables(variables, prefix);
}

function createCrawChatPortalDeploymentArtifactFileName(kind, basename) {
  return `craw-chat-portal.${kind}.${basename}`;
}

function createCrawChatPortalUserCenterPortalDeploymentProfile(profile) {
  const runtimeEnv = Object.freeze(
    mapCrawChatPortalUserCenterEnvironmentVariables(
      selectUserCenterDeploymentVariables(profile, 'application-runtime'),
      CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_PREFIX,
    ),
  );
  const gatewayEnv = Object.freeze(
    mapCrawChatPortalUserCenterEnvironmentVariables(
      mergeUserCenterDeploymentVariables(
        selectUserCenterDeploymentVariables(profile, 'upstream-bridge'),
        selectUserCenterDeploymentVariables(profile, 'external-authority-bridge'),
        selectUserCenterDeploymentVariables(profile, 'local-authority'),
      ),
      CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_PREFIX,
    ),
  );
  const runtimeEnvArtifact = Object.freeze(
    createUserCenterDeploymentEnvArtifact({
      audience: 'application-runtime',
      fileName: createCrawChatPortalDeploymentArtifactFileName(
        profile.kind,
        CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_ENV_ARTIFACT_BASENAME,
      ),
      headerComment: `Craw Chat Portal ${profile.kind} runtime env`,
      purpose: `Public runtime env artifact for the Craw Chat Portal ${profile.kind} user-center deployment.`,
      variables: runtimeEnv,
    }),
  );
  const gatewayEnvArtifact = Object.freeze(
    createUserCenterDeploymentEnvArtifact({
      audience: 'gateway-runtime',
      fileName: createCrawChatPortalDeploymentArtifactFileName(
        profile.kind,
        CRAW_CHAT_PORTAL_USER_CENTER_GATEWAY_ENV_ARTIFACT_BASENAME,
      ),
      headerComment: `Craw Chat Portal ${profile.kind} gateway env`,
      purpose: `Private gateway env artifact for the Craw Chat Portal ${profile.kind} user-center deployment.`,
      variables: gatewayEnv,
    }),
  );

  return Object.freeze({
    artifacts: Object.freeze([runtimeEnvArtifact, gatewayEnvArtifact]),
    gatewayEnvArtifact,
    handshakeEnabled: profile.handshake.enabled,
    kind: profile.kind,
    providerKey: profile.providerKey,
    runtimeEnvArtifact,
    standard: profile,
  });
}

function createCrawChatPortalUserCenterPortalDeploymentProfileSet(plugin) {
  return Object.freeze({
    activeKind: plugin.deployment.activeKind,
    builtinLocal: createCrawChatPortalUserCenterPortalDeploymentProfile(
      plugin.deployment.builtinLocal,
    ),
    externalAppApi: createCrawChatPortalUserCenterPortalDeploymentProfile(
      plugin.deployment.externalAppApi,
    ),
    ...(plugin.deployment.externalUserCenter
      ? {
          externalUserCenter: createCrawChatPortalUserCenterPortalDeploymentProfile(
            plugin.deployment.externalUserCenter,
          ),
        }
      : {}),
  });
}

export function createCrawChatPortalUserCenterSessionStore(
  storagePlan = CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN,
) {
  return createUserCenterSessionStore(storagePlan);
}

export function createCrawChatPortalUserCenterTokenStore(
  storagePlan = CRAW_CHAT_PORTAL_USER_CENTER_STORAGE_PLAN,
) {
  return createUserCenterTokenStore(storagePlan);
}

export function createCrawChatPortalUserCenterHandshakeSigningMessage({
  config = CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_CONFIG,
  method,
  path,
  signedAt,
}) {
  return createUserCenterHandshakeSigningMessage({
    config,
    method,
    path,
    signedAt,
  });
}

export function createCrawChatPortalUserCenterSignedHandshakeHeaders(
  signature,
  config = CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_CONFIG,
) {
  return createUserCenterSignedHandshakeHeaders(config, signature);
}

export function createCrawChatPortalUserCenterHandshakeVerificationContext({
  config = CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_CONFIG,
  ...options
}) {
  return createUserCenterHandshakeVerificationContext({
    ...options,
    config,
  });
}

export function createCrawChatPortalUserCenterConfig(options = {}) {
  return Object.freeze(
    createUserCenterBridgeConfig({
      auth: options.auth,
      localApiBasePath:
        options.localApiBasePath ?? CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH,
      mode: options.mode,
      namespace: CRAW_CHAT_PORTAL_USER_CENTER_NAMESPACE,
      provider: options.provider,
      routes: {
        authBasePath: options.routes?.authBasePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.authBasePath,
        userRoutePath: options.routes?.userRoutePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.userRoutePath,
        vipRoutePath: options.routes?.vipRoutePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.vipRoutePath,
      },
      storageTopology: options.storageTopology,
    }),
  );
}

export function createCrawChatPortalUserCenterPluginDefinition(options = {}) {
  const { bridgeConfig, plugin } = createCrawChatPortalUserCenterBasePluginArtifacts(options);

  return Object.freeze({
    auth: bridgeConfig.auth,
    ...plugin,
    bridgeConfig,
    integration: bridgeConfig.integration,
    manifests: Object.freeze({
      ...(plugin.manifests.auth
        ? {
            auth: Object.freeze({
              ...plugin.manifests.auth,
              forgotPasswordRoutePath: undefined,
              loginRoutePath: CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.authBasePath,
              oauthCallbackRoutePattern: undefined,
              qrRoutePath: undefined,
              registerRoutePath: undefined,
            }),
          }
        : {}),
    }),
    portalDeployment: createCrawChatPortalUserCenterPortalDeploymentProfileSet(plugin),
    storagePlan: bridgeConfig.storagePlan,
    storageTopology: bridgeConfig.storageTopology,
  });
}

export function createCrawChatPortalUserCenterServerPluginDefinition(options = {}) {
  return Object.freeze(
    createUserCenterServerPluginDefinition({
      auth: options.auth,
      description: options.description,
      localApiBasePath:
        options.localApiBasePath ?? CRAW_CHAT_PORTAL_USER_CENTER_LOCAL_API_BASE_PATH,
      mode: options.mode,
      namespace: CRAW_CHAT_PORTAL_USER_CENTER_NAMESPACE,
      packageNames: options.packageNames ?? [...CRAW_CHAT_PORTAL_USER_CENTER_PLUGIN_PACKAGES],
      provider: options.provider,
      routes: {
        authBasePath: options.routes?.authBasePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.authBasePath,
        userRoutePath: options.routes?.userRoutePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.userRoutePath,
        vipRoutePath: options.routes?.vipRoutePath ?? CRAW_CHAT_PORTAL_USER_CENTER_ROUTES.vipRoutePath,
      },
      storageTopology: options.storageTopology,
      title: options.title ?? 'Craw Chat User Center Server',
    }),
  );
}

export function createCrawChatPortalUserCenterPortalDeploymentProfiles(options = {}) {
  const { plugin } = createCrawChatPortalUserCenterBasePluginArtifacts(options);
  return createCrawChatPortalUserCenterPortalDeploymentProfileSet(plugin);
}

export const CRAW_CHAT_PORTAL_USER_CENTER_RUNTIME_CONFIG =
  createCrawChatPortalUserCenterConfig();
