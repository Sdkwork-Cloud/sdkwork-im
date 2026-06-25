import { ImSdkClient, ImWebSocketAuthOptions, IM_REALTIME_WS, type ImSdkClientOptions } from '@sdkwork/im-sdk';
import {
  DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL,
  DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL,
  VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL,
  VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL,
} from './topologyEnvKeys';
import {
  getSdkworkChatGlobalTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  resolveAppSdkSessionId,
  type SdkworkChatSession,
} from './session';

let imSdkClient: ImSdkClient | null = null;
let imSdkClientSessionKey: string | null = null;
const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';

type RuntimeImportMetaEnv = Record<string, string | boolean | undefined> & {
  DEV?: boolean | string;
};

function readRuntimeImportMetaEnv(): RuntimeImportMetaEnv {
  return (import.meta.env ?? {}) as RuntimeImportMetaEnv;
}

function hasViteImportMetaEnv(): boolean {
  return typeof import.meta.env !== 'undefined';
}

function readEnvValue(key: string): string | undefined {
  const value = readRuntimeImportMetaEnv()[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function readNodeEnvValue(key: string): string | undefined {
  const processLike = (globalThis as {
    process?: {
      env?: Record<string, string | undefined>;
    };
  }).process;
  const value = processLike?.env?.[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function isRuntimeDev(): boolean {
  const env = readRuntimeImportMetaEnv();
  if (env.DEV === true || env.DEV === 'true') {
    return true;
  }
  if (env.DEV === false || env.DEV === 'false') {
    return false;
  }
  const nodeEnv = readNodeEnvValue('NODE_ENV');
  if (nodeEnv) {
    return nodeEnv !== 'production';
  }
  return typeof window === 'undefined';
}

function stripSdkOwnedPathSuffix(pathname: string, suffixes: string[]): string {
  const normalizedPathname = pathname.replace(/\/+$/u, '');
  if (!normalizedPathname || normalizedPathname === '/') {
    return '';
  }

  for (const suffix of suffixes) {
    const normalizedSuffix = `/${suffix.replace(/^\/+|\/+$/gu, '')}`;
    if (normalizedPathname === normalizedSuffix) {
      return '';
    }
    if (normalizedPathname.endsWith(normalizedSuffix)) {
      return normalizedPathname.slice(0, -normalizedSuffix.length) || '';
    }
  }

  return normalizedPathname;
}

function normalizeHttpSdkBaseUrl(value: string): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== 'http:' && parsedUrl.protocol !== 'https:') {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(parsedUrl.pathname, [
      SDKWORK_APP_API_PREFIX,
      SDKWORK_IM_API_PREFIX,
    ]);
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

function normalizeWebSocketSdkBaseUrl(value: string): string {
  try {
    const parsedUrl = new URL(value);
    if (parsedUrl.protocol !== 'ws:' && parsedUrl.protocol !== 'wss:') {
      return value;
    }
    const normalizedPathname = stripSdkOwnedPathSuffix(parsedUrl.pathname, [
      IM_REALTIME_WS,
      SDKWORK_IM_API_PREFIX,
    ]);
    return `${parsedUrl.origin}${normalizedPathname}`;
  } catch {
    return value;
  }
}

function deriveWebSocketBaseUrlFromHttpBaseUrl(value: string | undefined): string | undefined {
  if (!value) {
    return undefined;
  }

  try {
    const parsedUrl = new URL(normalizeHttpSdkBaseUrl(value));
    parsedUrl.protocol = parsedUrl.protocol === 'https:' ? 'wss:' : 'ws:';
    return normalizeWebSocketSdkBaseUrl(parsedUrl.toString());
  } catch {
    return undefined;
  }
}

function resolveLocalDevImApiBaseUrl(): string | undefined {
  if (hasViteImportMetaEnv()) {
    if (!import.meta.env.DEV) {
      return undefined;
    }
  } else if (!isRuntimeDev()) {
    return undefined;
  }
  return DEFAULT_LOCAL_APPLICATION_PUBLIC_HTTP_URL;
}

function resolveLocalDevImWebSocketBaseUrl(): string | undefined {
  if (hasViteImportMetaEnv()) {
    if (!import.meta.env.DEV) {
      return undefined;
    }
  } else if (!isRuntimeDev()) {
    return undefined;
  }
  return DEFAULT_LOCAL_APPLICATION_PUBLIC_WEBSOCKET_URL;
}

function resolveSameOriginHttpBaseUrl(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const origin = window.location.origin;
  return typeof origin === 'string' && origin.length > 0 ? origin : undefined;
}

function resolveSameOriginWebSocketBaseUrl(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const { protocol, host } = window.location;
  if (!host) {
    return undefined;
  }
  return `${protocol === 'https:' ? 'wss' : 'ws'}://${host}`;
}

export function resolveImSdkApiBaseUrl(): string {
  const baseUrl = readEnvValue(VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL)
    ?? resolveLocalDevImApiBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Sdkwork IM SDK API base URL is not configured. Set VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL.',
    );
  }
  return normalizeHttpSdkBaseUrl(baseUrl);
}

export function resolveImSdkWebSocketBaseUrl(): string {
  const explicitBaseUrl = readEnvValue(VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL);
  const baseUrl = explicitBaseUrl
    ?? deriveWebSocketBaseUrlFromHttpBaseUrl(readEnvValue(VITE_SDKWORK_IM_APPLICATION_PUBLIC_HTTP_URL))
    ?? resolveLocalDevImWebSocketBaseUrl()
    ?? resolveSameOriginWebSocketBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Sdkwork IM SDK websocket base URL is not configured. Set VITE_SDKWORK_IM_APPLICATION_PUBLIC_WEBSOCKET_URL.',
    );
  }
  return explicitBaseUrl ? normalizeWebSocketSdkBaseUrl(baseUrl) : baseUrl;
}

export function createImSdkClientOptions(session?: SdkworkChatSession | null): ImSdkClientOptions {
  const currentSession = session ?? readAppSdkSessionTokens();
  const tokenManager = getSdkworkChatGlobalTokenManager();
  return {
    apiBaseUrl: resolveImSdkApiBaseUrl(),
    websocketBaseUrl: resolveImSdkWebSocketBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    platform: 'pc',
    tokenProvider: tokenManager,
    webSocketAuth: ImWebSocketAuthOptions.automatic({
      credentialProvider: () => resolveAppSdkAuthToken(readAppSdkSessionTokens()),
    }),
  };
}

function createImSdkClientSessionKey(session?: SdkworkChatSession | null): string {
  const currentSession = session ?? readAppSdkSessionTokens();
  const context = currentSession?.context;
  return JSON.stringify({
    accessToken: resolveAppSdkAccessToken(currentSession) ?? null,
    authToken: resolveAppSdkAuthToken(currentSession) ?? null,
    organizationId: context?.organizationId ?? null,
    sessionId: resolveAppSdkSessionId(currentSession) ?? null,
    tenantId: context?.tenantId ?? null,
    userId: context?.userId ?? currentSession?.user?.userId ?? currentSession?.user?.id ?? null,
  });
}

export function initImSdkClient(options: ImSdkClientOptions = createImSdkClientOptions()): ImSdkClient {
  imSdkClient = new ImSdkClient(options);
  imSdkClientSessionKey = null;
  return imSdkClient;
}

export function getImSdkClient(): ImSdkClient {
  return imSdkClient ?? initImSdkClient();
}

export function getImSdkClientWithSession(session = readAppSdkSessionTokens()): ImSdkClient {
  const sessionKey = createImSdkClientSessionKey(session);
  if (imSdkClient && imSdkClientSessionKey === sessionKey) {
    return imSdkClient;
  }
  imSdkClient = new ImSdkClient(createImSdkClientOptions(session));
  imSdkClientSessionKey = sessionKey;
  return imSdkClient;
}

export function resetImSdkClient(): void {
  imSdkClient = null;
  imSdkClientSessionKey = null;
}
