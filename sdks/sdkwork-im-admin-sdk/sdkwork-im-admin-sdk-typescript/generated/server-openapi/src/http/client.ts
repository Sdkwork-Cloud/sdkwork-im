import type { ImAdminBackendConfig } from '../types/common';
import type { RequestOptions, QueryParams } from '@sdkwork/sdk-common';
import type { AuthTokenManager } from '@sdkwork/sdk-common';
import { BaseHttpClient, SdkError, SUCCESS_CODES, withRetry } from '@sdkwork/sdk-common';

type HttpRequestOptions = RequestOptions & {
  body?: unknown;
  headers?: Record<string, string>;
  contentType?: string;
};

type ResultEnvelope = {
  code?: string | number;
  msg?: string;
  message?: string;
  data?: unknown;
  error?: {
    message?: string;
  };
};

function hasOwn(value: object, key: string): boolean {
  return Object.prototype.hasOwnProperty.call(value, key);
}

export class HttpClient extends BaseHttpClient {
  constructor(config: ImAdminBackendConfig) {
    super(config as any);
  }

  private buildRequestHeaders(
    headers?: Record<string, string>,
    contentType?: string,
  ): Record<string, string> | undefined {
    const mergedHeaders = {
      ...(headers ?? {}),
    };

    if (contentType && contentType.toLowerCase() !== 'multipart/form-data') {
      mergedHeaders['Content-Type'] = contentType;
    }

    return Object.keys(mergedHeaders).length > 0 ? mergedHeaders : undefined;
  }

  private isResultEnvelope(value: unknown): value is ResultEnvelope {
    return typeof value === 'object'
      && value !== null
      && (hasOwn(value, 'code') || hasOwn(value, 'data') || hasOwn(value, 'msg') || hasOwn(value, 'message'));
  }

  private hasSuccessCode(code: unknown): boolean {
    return SUCCESS_CODES.includes(code as never) || SUCCESS_CODES.includes(String(code) as never);
  }

  private async handleErrorResponse(response: Response, requestConfig: unknown): Promise<never> {
    let payload: unknown = null;

    try {
      payload = await response.json();
    } catch {
      payload = null;
    }

    let message = `HTTP ${response.status}: ${response.statusText}`;
    if (typeof payload === 'object' && payload !== null) {
      const candidate = payload as ResultEnvelope;
      message = candidate.error?.message?.trim()
        || candidate.msg?.trim()
        || candidate.message?.trim()
        || message;
    }

    const error = SdkError.fromHttpStatus(response.status, message);
    const applyErrorInterceptors = (this as any).applyErrorInterceptors;
    if (typeof applyErrorInterceptors === 'function') {
      await applyErrorInterceptors.call(this, error, requestConfig);
    }

    throw error;
  }

  async processResponse<T>(response: Response, requestConfig: unknown): Promise<T> {
    if (!response.ok) {
      return this.handleErrorResponse(response, requestConfig);
    }

    if (response.status === 204) {
      return undefined as T;
    }

    const contentType = response.headers.get('content-type') ?? '';
    if (contentType.includes('application/json')) {
      const result = await response.json();
      if (this.isResultEnvelope(result) && this.hasSuccessCode(result.code)) {
        return result.data as T;
      }
      if (this.isResultEnvelope(result) && hasOwn(result, 'code')) {
        throw SdkError.fromApiResult(result as never, response.status);
      }
      return result as T;
    }

    if (contentType.includes('text/')) {
      return await response.text() as T;
    }

    return await response.json() as T;
  }

  setAuthToken(token: string): void {
    super.setAuthToken(token);
  }

  setTokenManager(manager: AuthTokenManager): void {
    const baseProto = Object.getPrototypeOf(HttpClient.prototype) as {
      setTokenManager?: (this: HttpClient, m: AuthTokenManager) => void;
    };
    if (typeof baseProto.setTokenManager === 'function') {
      baseProto.setTokenManager.call(this, manager);
      return;
    }
    (this as any).authConfig = (this as any).authConfig || {};
    (this as any).authConfig.tokenManager = manager;
  }

  async request<T>(path: string, options: HttpRequestOptions = {}): Promise<T> {
    const execute = (this as any).execute;
    if (typeof execute !== 'function') {
      throw new Error('BaseHttpClient execute method is not available');
    }
    const { body, headers, contentType, method = 'GET', ...rest } = options;
    return withRetry(
      () =>
        execute.call(this, {
          url: path,
          method,
          ...rest,
          body,
          headers: this.buildRequestHeaders(headers, body == null ? undefined : contentType),
        }),
      { maxRetries: 3 },
    );
  }

  async get<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.request<T>(path, { method: 'GET', params, headers });
  }

  async post<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'POST', body, params, headers, contentType });
  }

  async put<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'PUT', body, params, headers, contentType });
  }

  async delete<T>(path: string, params?: QueryParams, headers?: Record<string, string>): Promise<T> {
    return this.request<T>(path, { method: 'DELETE', params, headers });
  }

  async patch<T>(
    path: string,
    body?: unknown,
    params?: QueryParams,
    headers?: Record<string, string>,
    contentType?: string,
  ): Promise<T> {
    return this.request<T>(path, { method: 'PATCH', body, params, headers, contentType });
  }
}

export function createHttpClient(config: ImAdminBackendConfig): HttpClient {
  return new HttpClient(config);
}
