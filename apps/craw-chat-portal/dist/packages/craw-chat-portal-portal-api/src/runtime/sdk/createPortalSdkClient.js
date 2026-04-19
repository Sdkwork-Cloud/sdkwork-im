const DEFAULT_PORTAL_API_PORT = '18124';
const BROWSER_PORTAL_SDK_SPECIFIER = '/__vendor__/sdkwork-im-sdk/index.js';
const NODE_PORTAL_SDK_MODULE_URL = new URL(
  '../../../../../../../sdks/sdkwork-im-sdk/sdkwork-im-sdk-typescript/dist/index.js',
  import.meta.url,
);

let portalSdkModulePromise = null;
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

function createClientCacheKey(sdkConfig) {
  return JSON.stringify({
    baseUrl: sdkConfig.baseUrl,
    authToken: sdkConfig.authToken ?? null,
  });
}

export function resolvePortalSdkConfig({ authToken } = {}) {
  const sdkConfig = {
    baseUrl: resolvePortalApiBaseUrl(),
  };

  if (isNonEmptyString(authToken)) {
    sdkConfig.authToken = authToken.trim();
  }

  return sdkConfig;
}

export async function createPortalSdkClient(options = {}) {
  const sdkConfig = resolvePortalSdkConfig(options);
  const cacheKey = createClientCacheKey(sdkConfig);

  if (!clientPromiseCache.has(cacheKey)) {
    const portalSdkModule = await loadPortalSdkModule();
    const ImSdkClient = portalSdkModule?.ImSdkClient;
    if (typeof ImSdkClient !== 'function') {
      throw new Error('Unable to resolve ImSdkClient from the IM SDK runtime.');
    }

    const clientPromise = Promise.resolve(
      new ImSdkClient(sdkConfig),
    ).catch((error) => {
      clientPromiseCache.delete(cacheKey);
      throw error;
    });
    clientPromiseCache.set(cacheKey, clientPromise);
  }

  return clientPromiseCache.get(cacheKey);
}
