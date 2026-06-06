import { ImSdkClient, ImWebSocketAuthOptions, type ImSdkClientOptions } from '@sdkwork/im-sdk';
import {
  buildSdkworkChatAppContextHeaders,
  createSdkworkChatSessionTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  resolveAppSdkSessionId,
  type SdkworkChatSession,
} from './session';

let imSdkClient: ImSdkClient | null = null;
const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';
const SDKWORK_IM_REALTIME_WEBSOCKET_PATH = '/im/v3/api/realtime/ws';

function readEnvValue(key: string): string | undefined {
  const value = import.meta.env?.[key];
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
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
      SDKWORK_IM_REALTIME_WEBSOCKET_PATH,
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
  if (!import.meta.env.DEV) {
    return undefined;
  }
  return 'http://127.0.0.1:18079';
}

function resolveLocalDevImWebSocketBaseUrl(): string | undefined {
  if (!import.meta.env.DEV) {
    return undefined;
  }
  return 'ws://127.0.0.1:18079';
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
  const baseUrl = readEnvValue('VITE_CRAW_CHAT_IM_API_BASE_URL')
    ?? resolveLocalDevImApiBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Craw Chat IM SDK API base URL is not configured. Set VITE_CRAW_CHAT_IM_API_BASE_URL or serve the web build from the unified gateway origin.',
    );
  }
  return normalizeHttpSdkBaseUrl(baseUrl);
}

export function resolveImSdkWebSocketBaseUrl(): string {
  const explicitBaseUrl = readEnvValue('VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL');
  const baseUrl = readEnvValue('VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL')
    ?? deriveWebSocketBaseUrlFromHttpBaseUrl(readEnvValue('VITE_CRAW_CHAT_IM_API_BASE_URL'))
    ?? resolveLocalDevImWebSocketBaseUrl()
    ?? resolveSameOriginWebSocketBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Craw Chat IM SDK websocket base URL is not configured. Set VITE_CRAW_CHAT_IM_WEBSOCKET_BASE_URL or serve the web build from the unified gateway origin.',
    );
  }
  return explicitBaseUrl ? normalizeWebSocketSdkBaseUrl(baseUrl) : baseUrl;
}

function buildImSdkContextHeaders(session?: SdkworkChatSession | null): Record<string, string> {
  const headers = buildSdkworkChatAppContextHeaders(session) ?? {};
  const sessionId = resolveAppSdkSessionId(session);
  return {
    ...headers,
    ...(sessionId ? { 'X-Sdkwork-Session-Id': sessionId } : {}),
  };
}

export function createImSdkClientOptions(session?: SdkworkChatSession | null): ImSdkClientOptions {
  const currentSession = session ?? readAppSdkSessionTokens();
  const tokenManager = createSdkworkChatSessionTokenManager(currentSession);
  return {
    apiBaseUrl: resolveImSdkApiBaseUrl(),
    websocketBaseUrl: resolveImSdkWebSocketBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    headerProvider: () => buildImSdkContextHeaders(readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc',
    tokenProvider: tokenManager,
    webSocketAuth: ImWebSocketAuthOptions.automatic({
      credentialProvider: () => resolveAppSdkAuthToken(readAppSdkSessionTokens()),
    }),
  };
}

export function initImSdkClient(options: ImSdkClientOptions = createImSdkClientOptions()): ImSdkClient {
  imSdkClient = new ImSdkClient(options);
  return imSdkClient;
}

export function getImSdkClient(): ImSdkClient {
  return imSdkClient ?? initImSdkClient();
}

export function getImSdkClientWithSession(session = readAppSdkSessionTokens()): ImSdkClient {
  return initImSdkClient(createImSdkClientOptions(session));
}

export function resetImSdkClient(): void {
  imSdkClient = null;
}
