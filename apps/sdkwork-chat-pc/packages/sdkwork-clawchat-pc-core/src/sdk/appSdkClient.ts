import {
  createClient,
  type SdkworkImAppClient as GeneratedSdkworkImAppClient,
  type SdkworkAppConfig,
} from '@sdkwork-internal/im-app-api-generated';
import {
  createSdkworkChatRequestContextInterceptors,
  createSdkworkChatSessionTokenManager,
  readAppSdkSessionTokens,
  resolveAppSdkAccessToken,
  resolveAppSdkAuthToken,
  type SdkworkChatSession,
} from './session';
import type { Interceptors } from '@sdkwork/sdk-common';

export type SdkworkImAppClient = GeneratedSdkworkImAppClient;
export type SdkworkImAppClientConfig = SdkworkAppConfig & {
  interceptors?: Interceptors;
};

let appSdkClient: SdkworkImAppClient | null = null;
const SDKWORK_APP_API_PREFIX = '/app/v3/api';
const SDKWORK_IM_API_PREFIX = '/im/v3/api';

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

function normalizeAppSdkBaseUrl(value: string): string {
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

function resolveLocalDevAppApiBaseUrl(): string | undefined {
  if (!import.meta.env.DEV) {
    return undefined;
  }
  return 'http://127.0.0.1:18079';
}

function resolveSameOriginHttpBaseUrl(): string | undefined {
  if (typeof window === 'undefined') {
    return undefined;
  }

  const origin = window.location.origin;
  return typeof origin === 'string' && origin.length > 0 ? origin : undefined;
}

export function resolveAppSdkBaseUrl(): string {
  const baseUrl = readEnvValue('VITE_SDKWORK_IAM_APP_API_BASE_URL')
    ?? readEnvValue('VITE_CRAW_CHAT_APP_API_BASE_URL')
    ?? resolveLocalDevAppApiBaseUrl()
    ?? resolveSameOriginHttpBaseUrl();
  if (!baseUrl) {
    throw new Error(
      'Craw Chat app SDK base URL is not configured. Set VITE_CRAW_CHAT_APP_API_BASE_URL or serve the web build from the unified gateway origin.',
    );
  }
  return normalizeAppSdkBaseUrl(baseUrl);
}

export function createAppSdkClientConfig(session?: SdkworkChatSession | null): SdkworkImAppClientConfig {
  const currentSession = session ?? readAppSdkSessionTokens();
  return {
    baseUrl: resolveAppSdkBaseUrl(),
    accessToken: resolveAppSdkAccessToken(currentSession),
    authToken: resolveAppSdkAuthToken(currentSession),
    interceptors: createSdkworkChatRequestContextInterceptors(() => readAppSdkSessionTokens() ?? currentSession),
    platform: 'pc',
    tokenManager: createSdkworkChatSessionTokenManager(currentSession),
  };
}

export function initAppSdkClient(
  config: SdkworkImAppClientConfig = createAppSdkClientConfig(),
): SdkworkImAppClient {
  appSdkClient = createClient(config);
  return appSdkClient;
}

export function getAppSdkClient(): SdkworkImAppClient {
  return appSdkClient ?? initAppSdkClient();
}

export function getAppSdkClientWithSession(
  session = readAppSdkSessionTokens(),
): SdkworkImAppClient {
  return initAppSdkClient(createAppSdkClientConfig(session));
}

export function resetAppSdkClient(): void {
  appSdkClient = null;
}

export function useAppSdkClient(): SdkworkImAppClient {
  return getAppSdkClientWithSession();
}
