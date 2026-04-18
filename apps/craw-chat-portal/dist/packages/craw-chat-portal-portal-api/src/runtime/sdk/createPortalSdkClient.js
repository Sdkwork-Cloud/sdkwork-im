const DEFAULT_PORTAL_API_PORT = '18124';
const BROWSER_PORTAL_SDK_SPECIFIER = '/__vendor__/sdkwork-craw-chat-sdk/index.js';
const BROWSER_BACKEND_SDK_SPECIFIER = '/__vendor__/sdkwork-craw-chat-backend-sdk/index.js';
const NODE_PORTAL_SDK_MODULE_URL = new URL(
  '../../../../../../../sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript/composed/dist/index.js',
  import.meta.url,
);
const NODE_BACKEND_SDK_MODULE_URL = new URL(
  '../../../../../../../sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript/generated/server-openapi/browser/index.js',
  import.meta.url,
);

let portalSdkModulePromise = null;
let backendSdkModulePromise = null;
const clientPromiseCache = new Map();

function isNonEmptyString(value) {
  return typeof value === 'string' && value.trim().length > 0;
}

function normalizeBaseUrl(baseUrl) {
  return baseUrl.replace(/\/+$/, '');
}

function resolvePortalApiBaseUrl() {
  if (
    typeof window !== 'undefined' &&
    isNonEmptyString(window.__CRAW_CHAT_PORTAL_API_BASE_URL__)
  ) {
    return normalizeBaseUrl(window.__CRAW_CHAT_PORTAL_API_BASE_URL__.trim());
  }

  if (typeof window !== 'undefined' && window.location) {
    if (isNonEmptyString(window.location.origin)) {
      return normalizeBaseUrl(window.location.origin.trim());
    }

    const hostname = window.location.hostname || '127.0.0.1';
    return `http://${hostname}:${DEFAULT_PORTAL_API_PORT}`;
  }

  return `http://127.0.0.1:${DEFAULT_PORTAL_API_PORT}`;
}

async function loadPortalSdkModule() {
  if (portalSdkModulePromise) {
    return portalSdkModulePromise;
  }

  portalSdkModulePromise =
    typeof window !== 'undefined'
      ? import(BROWSER_PORTAL_SDK_SPECIFIER)
      : import(NODE_PORTAL_SDK_MODULE_URL.href);

  return portalSdkModulePromise;
}

async function loadBackendSdkModule() {
  if (backendSdkModulePromise) {
    return backendSdkModulePromise;
  }

  backendSdkModulePromise =
    typeof window !== 'undefined'
      ? import(BROWSER_BACKEND_SDK_SPECIFIER)
      : import(NODE_BACKEND_SDK_MODULE_URL.href);

  return backendSdkModulePromise;
}

function createClientCacheKey(backendConfig) {
  return JSON.stringify({
    baseUrl: backendConfig.baseUrl,
    authToken: backendConfig.authToken ?? null,
  });
}

export function resolvePortalBackendConfig({ authToken } = {}) {
  const backendConfig = {
    baseUrl: resolvePortalApiBaseUrl(),
  };

  if (isNonEmptyString(authToken)) {
    backendConfig.authToken = authToken.trim();
  }

  return backendConfig;
}

export async function createPortalSdkClient(options = {}) {
  const backendConfig = resolvePortalBackendConfig(options);
  const cacheKey = createClientCacheKey(backendConfig);

  if (!clientPromiseCache.has(cacheKey)) {
    const portalSdkModule = await loadPortalSdkModule();
    const backendSdkModule = await loadBackendSdkModule();
    const CrawChatClient = portalSdkModule?.CrawChatClient;
    const createClient = backendSdkModule?.createClient;
    if (typeof CrawChatClient !== 'function') {
      throw new Error('Unable to resolve CrawChatClient from the portal SDK runtime.');
    }
    if (typeof createClient !== 'function') {
      throw new Error('Unable to resolve createClient from the backend SDK runtime.');
    }

    const clientPromise = Promise.resolve(
      new CrawChatClient({
        backendClient: createClient(backendConfig),
      }),
    ).catch((error) => {
      clientPromiseCache.delete(cacheKey);
      throw error;
    });
    clientPromiseCache.set(cacheKey, clientPromise);
  }

  return clientPromiseCache.get(cacheKey);
}
