import { createClient as createGeneratedClient } from '@sdkwork/craw-chat-backend-sdk';
import type { CrawChatBackendClientLike } from './backend-client-like.js';
import type {
  CrawChatSdkClientOptions,
  CrawChatWebSocketFactory,
  SdkworkBackendConfig,
} from './types.js';
import { CrawChatSdkError } from './errors.js';

export const DEFAULT_REALTIME_WEBSOCKET_PATH = 'api/v1/realtime/ws';

interface CrawChatInternalTransportConfig {
  apiBaseUrl?: string;
  websocketBaseUrl?: string;
}

interface CrawChatInternalResolvedSdkClientOptions {
  backendClient?: CrawChatBackendClientLike;
  backendConfig?: SdkworkBackendConfig;
  transport: CrawChatInternalTransportConfig;
  authToken?: string;
  webSocketFactory?: CrawChatWebSocketFactory;
}

interface CrawChatInternalGeneratedTransportOverrides {
  tokenManager?: SdkworkBackendConfig['tokenManager'];
  timeout?: number;
  headers?: Record<string, string>;
}

type CrawChatSdkClientRuntimeOptions = CrawChatSdkClientOptions
  & CrawChatInternalGeneratedTransportOverrides
  & {
  backendClient?: CrawChatBackendClientLike;
};

interface CrawChatInternalSdkContextOptions {
  backendClient: CrawChatBackendClientLike;
  transport?: CrawChatInternalTransportConfig;
  authToken?: string;
  webSocketFactory?: CrawChatWebSocketFactory;
}

export function createGeneratedBackendClient(
  backendConfig: SdkworkBackendConfig,
): CrawChatBackendClientLike {
  return createGeneratedClient(backendConfig) as CrawChatBackendClientLike;
}

export function normalizeCrawChatSdkCreateOptions(
  options: CrawChatSdkClientRuntimeOptions,
): CrawChatInternalResolvedSdkClientOptions {
  const apiBaseUrl = firstDefinedString(
    options.apiBaseUrl,
    options.baseUrl,
  );
  const authToken = firstDefinedString(options.authToken);

  const transport: CrawChatInternalTransportConfig = {
    apiBaseUrl,
    websocketBaseUrl: firstDefinedString(options.websocketBaseUrl),
  };

  if (options.backendClient) {
    return {
      backendClient: options.backendClient,
      transport,
      authToken,
      webSocketFactory: options.webSocketFactory,
    };
  }

  const tokenManager =
    options.tokenProvider
    ?? options.tokenManager;

  const hasTransportOverrides =
    apiBaseUrl != null
    || authToken != null
    || tokenManager != null
    || options.timeout != null
    || options.headers != null;

  if (!hasTransportOverrides) {
    return {
      transport,
      authToken,
      webSocketFactory: options.webSocketFactory,
    };
  }

  if (!apiBaseUrl) {
    throw new CrawChatSdkError(
      'api_base_url_required',
      'baseUrl or apiBaseUrl is required when creating a generated Craw Chat SDK client',
    );
  }

  return {
    backendConfig: omitUndefined({
      baseUrl: apiBaseUrl,
      authToken,
      tokenManager,
      timeout: options.timeout,
      headers: options.headers,
    }),
    transport,
    authToken,
    webSocketFactory: options.webSocketFactory,
  };
}

export function resolveBackendClient(
  options: CrawChatSdkClientRuntimeOptions,
): CrawChatBackendClientLike {
  const normalized = normalizeCrawChatSdkCreateOptions(options);

  if (normalized.backendClient) {
    return normalized.backendClient;
  }
  if (normalized.backendConfig) {
    return createGeneratedBackendClient(normalized.backendConfig);
  }
  throw new CrawChatSdkError(
    'backend_client_missing',
    'baseUrl or apiBaseUrl is required',
  );
}

export function resolveCrawChatClientOptions(
  options: CrawChatSdkClientRuntimeOptions,
): CrawChatInternalSdkContextOptions {
  const normalized = normalizeCrawChatSdkCreateOptions(options);

  return {
    backendClient: resolveBackendClient(options),
    transport: normalized.transport,
    authToken: normalized.authToken,
    webSocketFactory: normalized.webSocketFactory,
  };
}

export function resolveCrawChatWebSocketBaseUrl(baseUrl: string): string {
  const parsedUrl = new URL(baseUrl);
  if (parsedUrl.protocol === 'https:') {
    parsedUrl.protocol = 'wss:';
  } else if (parsedUrl.protocol === 'http:') {
    parsedUrl.protocol = 'ws:';
  }

  return stripTrailingSlash(parsedUrl.toString());
}

export class CrawChatSdkContext {
  private authToken?: string;

  constructor(
    readonly backendClient: CrawChatBackendClientLike,
    private readonly transport: CrawChatInternalTransportConfig = {},
    readonly webSocketFactory?: CrawChatInternalSdkContextOptions['webSocketFactory'],
    initialAuthToken?: string,
  ) {
    if (initialAuthToken) {
      this.setAuthToken(initialAuthToken);
    }
  }

  setAuthToken(token: string): void {
    this.authToken = token;
    this.backendClient.setAuthToken?.(token);
  }

  clearAuthToken(): void {
    this.authToken = undefined;
    if (typeof this.backendClient.clearAuthToken === 'function') {
      this.backendClient.clearAuthToken();
      return;
    }
    this.backendClient.setAuthToken?.('');
  }

  getAuthToken(): string | undefined {
    return this.authToken;
  }

  getApiBaseUrl(): string | undefined {
    return this.transport.apiBaseUrl;
  }

  getWebSocketBaseUrl(): string | undefined {
    if (this.transport.websocketBaseUrl) {
      return this.transport.websocketBaseUrl;
    }
    if (this.transport.apiBaseUrl) {
      return resolveCrawChatWebSocketBaseUrl(this.transport.apiBaseUrl);
    }
    return undefined;
  }

  resolveRealtimeWebSocketUrl(
    path: string = DEFAULT_REALTIME_WEBSOCKET_PATH,
  ): string | undefined {
    const websocketBaseUrl = this.getWebSocketBaseUrl();
    if (!websocketBaseUrl) {
      return undefined;
    }

    return `${stripTrailingSlash(websocketBaseUrl)}/${stripLeadingSlash(path)}`;
  }
}

function firstDefinedString(
  ...values: Array<string | undefined>
): string | undefined {
  for (const value of values) {
    if (typeof value === 'string' && value.trim().length > 0) {
      return value;
    }
  }

  return undefined;
}

function stripTrailingSlash(value: string): string {
  return value.replace(/\/+$/, '');
}

function stripLeadingSlash(value: string): string {
  return value.replace(/^\/+/, '');
}

function omitUndefined<T extends Record<string, unknown>>(value: T): T {
  return Object.fromEntries(
    Object.entries(value).filter(([, entryValue]) => entryValue !== undefined),
  ) as T;
}
