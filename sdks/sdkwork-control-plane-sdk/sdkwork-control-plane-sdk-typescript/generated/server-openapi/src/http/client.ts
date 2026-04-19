import type {
  ControlPlaneBackendConfig,
  FetchLike,
  JsonObject,
  QueryParams,
} from '../types/common.js';
import { AdminApiError, DEFAULT_TIMEOUT } from '../types/common.js';

type HttpRequestOptions = {
  method?: string;
  params?: QueryParams;
  body?: JsonObject;
  headers?: Record<string, string>;
};

function normalizeBaseUrl(baseUrl: string): string {
  return baseUrl.replace(/\/+$/, '');
}

function buildQueryString(params?: QueryParams): string {
  if (!params) {
    return '';
  }

  const entries = Object.entries(params).filter(([, value]) => value !== undefined);
  if (entries.length === 0) {
    return '';
  }

  return entries
    .map(([key, value]) => `${encodeURIComponent(key)}=${encodeURIComponent(String(value))}`)
    .join('&');
}

function resolveFetchImplementation(config: ControlPlaneBackendConfig): FetchLike {
  if (config.fetch) {
    return config.fetch;
  }

  const candidate = (globalThis as { fetch?: unknown }).fetch;
  if (typeof candidate !== 'function') {
    throw new Error('A fetch implementation is required to use the admin backend SDK.');
  }

  return candidate as FetchLike;
}

async function withTimeout<T>(promise: Promise<T>, timeoutMs: number): Promise<T> {
  if (!Number.isFinite(timeoutMs) || timeoutMs <= 0) {
    return promise;
  }

  return await Promise.race([
    promise,
    new Promise<T>((_resolve, reject) => {
      setTimeout(() => reject(new Error(`Admin backend request timed out after ${timeoutMs}ms.`)), timeoutMs);
    }),
  ]);
}

export class HttpClient {
  private readonly config: ControlPlaneBackendConfig;

  constructor(config: ControlPlaneBackendConfig) {
    this.config = {
      ...config,
      baseUrl: normalizeBaseUrl(config.baseUrl),
      headers: { ...(config.headers ?? {}) },
      timeout: config.timeout ?? DEFAULT_TIMEOUT,
    };
  }

  setAuthToken(token: string): void {
    this.config.authToken = token;
  }

  private buildUrl(path: string, params?: QueryParams): string {
    const queryString = buildQueryString(params);
    const normalizedPath = path.startsWith('/') ? path : `/${path}`;
    return `${this.config.baseUrl}${normalizedPath}${queryString ? `?${queryString}` : ''}`;
  }

  private buildHeaders(headers?: Record<string, string>): Record<string, string> {
    return {
      accept: 'application/json',
      ...(this.config.authToken ? { authorization: `Bearer ${this.config.authToken}` } : {}),
      ...(this.config.headers ?? {}),
      ...(headers ?? {}),
    };
  }

  private async parsePayload(response: { json(): Promise<unknown>; text(): Promise<string> }): Promise<unknown> {
    try {
      return await response.json();
    } catch {
      const text = await response.text();
      return text.length > 0 ? { message: text } : {};
    }
  }

  async request<T extends JsonObject>(path: string, options: HttpRequestOptions = {}): Promise<T> {
    const fetchImpl = resolveFetchImplementation(this.config);
    const response = await withTimeout(
      fetchImpl(this.buildUrl(path, options.params), {
        method: options.method ?? 'GET',
        headers: this.buildHeaders(
          options.body ? { 'content-type': 'application/json', ...(options.headers ?? {}) } : options.headers,
        ),
        body: options.body ? JSON.stringify(options.body) : undefined,
      }),
      this.config.timeout ?? DEFAULT_TIMEOUT,
    );

    const payload = await this.parsePayload(response);
    if (!response.ok) {
      const errorMessage =
        payload &&
        typeof payload === 'object' &&
        'message' in payload &&
        typeof (payload as { message?: unknown }).message === 'string'
          ? (payload as { message: string }).message
          : undefined;
      throw new AdminApiError(response.status, payload, errorMessage);
    }

    return payload as T;
  }

  get<T extends JsonObject>(path: string, params?: QueryParams): Promise<T> {
    return this.request<T>(path, { method: 'GET', params });
  }

  post<T extends JsonObject>(path: string, body?: JsonObject, params?: QueryParams): Promise<T> {
    return this.request<T>(path, { method: 'POST', body, params });
  }
}

export function createHttpClient(config: ControlPlaneBackendConfig): HttpClient {
  return new HttpClient(config);
}
